/// this gonna be a messy file, dw about it
use parking_lot::Mutex;
use serde::Deserialize;
use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use tokio::sync::broadcast;

use crate::{
    config,
    sql::{Timer, SQL},
};

const API_VERSION: &str = "v1";

#[derive(Deserialize, Clone, Debug)]
pub struct FetchTimer {
    uuid: String,
}

#[derive(Clone)]
pub struct RouteStates {
    pub tx: Arc<Mutex<broadcast::Sender<String>>>,
}

impl RouteStates {
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(254);
        Self {
            tx: Arc::new(Mutex::new(tx)),
        }
    }
}

impl Default for RouteStates {
    fn default() -> Self {
        Self::new()
    }
}

fn pre(input: &str) -> String {
    format!("/api/{API_VERSION}{input}")
}

async fn get_twitch_username() -> String {
    config!().twitch.username.clone()
}

async fn get_all_timers() -> impl IntoResponse {
    if let Ok(ret) = SQL.get_all_timer().await {
        return (StatusCode::OK, serde_json::json!(&ret).to_string());
    }

    (StatusCode::INTERNAL_SERVER_ERROR, String::new())
}

async fn get_timer(Query(query): Query<FetchTimer>) -> impl IntoResponse {
    if let Ok(ret) = SQL.get_timer(query.uuid).await {
        return (StatusCode::OK, serde_json::json!(&ret).to_string());
    }
    (StatusCode::INTERNAL_SERVER_ERROR, "broke".to_string())
}

async fn post_create_timer(Json(payload): Json<Timer>) -> impl IntoResponse {
    if SQL.post_create_timer(payload).await.is_ok() {
        return (StatusCode::OK, "Timer created");
    }
    (StatusCode::INTERNAL_SERVER_ERROR, "broke")
}

async fn post_update_timer(Json(payload): Json<Timer>) -> impl IntoResponse {
    ""
}

async fn ws_stuff(ws: WebSocketUpgrade, State(state): State<RouteStates>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

pub fn create_routes() -> Router {
    Router::new()
        .route(&pre("/get_twitch_username"), get(get_twitch_username))
        .route(&pre("/get_all_timers"), get(get_all_timers))
        .route(&pre("/get_timer"), get(get_timer))
        .route(&pre("/post_create_timer"), post(post_create_timer))
        .route(&pre("/post_update_timer"), post(post_update_timer))
        .route(&pre("/ws"), get(ws_stuff))
        .with_state(RouteStates::default())
}

async fn handle_socket(socket: WebSocket, state: RouteStates) {
    let (tx, rx) = socket.split();

    tokio::spawn(write(tx, state.clone()));
    tokio::spawn(read(rx, state.clone()));
}

async fn read(mut rec: SplitStream<WebSocket>, state: RouteStates) {
    while let Some(msg) = rec.next().await {
        match msg {
            Ok(Message::Text(text)) => {}
            Ok(Message::Close(_)) => {
                let tx = state.tx.lock();
                let _ = tx.send(String::from("close"));
                break;
            }
            Err(e) => println!("{e:?}"),
            _ => (),
        }
    }
}

async fn write(mut sen: SplitSink<WebSocket, Message>, state: RouteStates) {
    let mut rx = state.tx.lock().subscribe();

    if sen.send(Message::Ping(vec![1, 2, 3].into())).await.is_err() {
        return;
    }

    while let Ok(msg) = rx.recv().await {
        if msg == "close" {
            break;
        }

        if sen.send(Message::Text(msg.clone().into())).await.is_err() {
            break;
        }
    }
}
