use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
    routing::get,
    Router,
};

use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};

use crate::config;

const API_VERSION: &str = "v1";

#[derive(Clone)]
pub struct RouteStates {}

fn pre(input: &str) -> String {
    format!("/api/{API_VERSION}{input}")
}

async fn get_twitch_username() -> String {
    config!().twitch.username.clone()
}

async fn ws_stuff(ws: WebSocketUpgrade, State(state): State<RouteStates>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: RouteStates) {
    let (mut tx, mut rx) = socket.split();

    tokio::spawn(ws_tx(tx));
    tokio::spawn(ws_rx(rx));
}

async fn ws_tx(tx: SplitSink<WebSocket, Message>) {}

async fn ws_rx(rx: SplitStream<WebSocket>) {}

pub fn create_routes() -> Router {
    Router::new()
        .route(&pre("/get_twitch_username"), get(get_twitch_username))
        .route(&pre("/ws"), get(ws_stuff))
        .with_state(RouteStates {})
}
