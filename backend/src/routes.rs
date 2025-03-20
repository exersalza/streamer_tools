/// this gonna be a messy file, dw about it
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc, thread, time::Duration};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
    Json, Router,
};
use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use lazy_static::lazy_static;
use tokio::sync::broadcast;

use crate::{
    config::AM,
    sql::{Timer, SQL},
    twitch::update_user_in_db,
    utils::ButtonFunction,
};

lazy_static! {
    static ref ws_write_fn: AM<Vec<SplitSink<WebSocket, Message>>> = Arc::new(Mutex::new(vec![]));
    static ref tick_oneshot: AM<bool> = Arc::new(Mutex::new(true));
}

const API_VERSION: &str = "v1";

#[derive(Deserialize, Clone, Debug)]
pub struct FetchTimer {
    uuid: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ButtonPressed {
    function: ButtonFunction,
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
    crate::config!().twitch.username.clone()
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

async fn post_button_pressed(Json(payload): Json<ButtonPressed>) -> impl IntoResponse {
    dbg!(payload);
    "cool"
}

async fn post_update_timer(Json(payload): Json<Timer>) -> impl IntoResponse {
    ""
}

async fn get_timer_ids() -> impl IntoResponse {
    serde_json::to_string(&SQL.get_timer_ids().await.unwrap()).unwrap()
}

async fn ws_stuff(ws: WebSocketUpgrade, State(state): State<RouteStates>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

#[derive(Deserialize, Debug, Clone, Serialize)]
struct TwitchAuth {
    code: Option<String>,
    scope: Option<String>,
    error: Option<String>,
    error_message: Option<String>,
}

fn to_x_www_thingies_fuck_of(input: Vec<(&str, String)>) -> String {
    let mut ret = String::new();

    input.iter().for_each(|item| {
        ret += &format!("{}={}&", item.0, item.1);
    });

    ret.remove(ret.len() - 1);
    ret
}

#[derive(Deserialize, Debug)]
pub struct AuthTokenResponseOk {
    pub access_token: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub scope: Vec<String>,
    pub token_type: String,
}

struct AuthTokenResponseNotOk {}

async fn refresh_twitch_token() {
    let client = reqwest::Client::new();
    let twitch = crate::config!().twitch.clone();

    let token = match SQL.get_refresh_token().await {
        Ok(toki) => toki,
        Err(e) => panic!("{e} erm"),
    };

    // TODO: refactor bc DRY and stuff
    let fjdaslkjfkls = vec![
        ("client_id", twitch.client_id.clone()),
        ("client_secret", twitch.client_secret.clone()),
        ("refresh_token", token),
        ("grant_type", "refresh_token".to_string()),
    ];

    let fdjasklfsjad: HashMap<&str, String> = HashMap::from_iter(fjdaslkjfkls);

    let res = client
        .post("https://id.twitch.tv/oauth2/token")
        .form(&fdjasklfsjad)
        .send()
        .await;

    let f = &res.unwrap().text().await.unwrap_or("{}".to_string());
    let ff: AuthTokenResponseOk = serde_json::from_str(f).unwrap();

    match SQL.update_user_access_token(ff).await {
        Ok(e) => (),
        Err(e) => (),
    }
}

async fn twitch_auth(Query(query): Query<TwitchAuth>) -> impl IntoResponse {
    let token = query.code;
    let scope = query.scope;

    if let Some(error) = query.error {
        eprintln!("{} {}", error, query.error_message.unwrap_or("".into()));
        return Redirect::permanent("/twitch_invalid");
    }

    let client = reqwest::Client::new();
    let twitch = crate::config!().twitch.clone();

    let fjdaslkjfkls = vec![
        ("client_id", twitch.client_id.clone()),
        ("client_secret", twitch.client_secret.clone()),
        ("code", token.unwrap()),
        ("grant_type", "authorization_code".to_string()),
        ("redirect_uri", "http://localhost:22727/".to_string()),
    ];

    let fdjasklfsjad: HashMap<&str, String> = HashMap::from_iter(fjdaslkjfkls);

    let res = client
        .post("https://id.twitch.tv/oauth2/token")
        .form(&fdjasklfsjad)
        .send()
        .await;

    let f = &res.unwrap().text().await.unwrap_or("{}".to_string());
    let ff: AuthTokenResponseOk = serde_json::from_str(f).unwrap();
    dbg!(&ff);

    match SQL.update_user_access_token(ff).await {
        Ok(e) => {
            dbg!(e);
        }
        Err(e) => {
            dbg!(e);
        }
    }

    Redirect::permanent("/")
}

async fn twitch_invalid() -> impl IntoResponse {
    Html(
        r#"You declined the permissions <img src="https://cdn.7tv.app/emote/01JG36RF6KEFA8M22X4J0SKR9J/1x.avif" /><a href="/">back home</a>"#,
    )
}

async fn connected_to_twitch() -> impl IntoResponse {
    SQL.get_user_token_exist()
        .await
        .unwrap_or(false)
        .to_string()
}

async fn get_user() -> impl IntoResponse {
    match SQL.get_user().await {
        Ok(v) => serde_json::to_string(&v).unwrap_or("{}".to_string()),
        Err(e) => e.to_string(),
    }
}

#[derive(Deserialize)]
struct UpdateUser {
    username: String,
}

async fn update_user(Json(payload): Json<UpdateUser>) -> impl IntoResponse {
    if let Err(e) = SQL.update_username(payload.username.clone()).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string());
    }

    if let Err(e) = update_user_in_db(payload.username).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string());
    };

    (StatusCode::OK, String::new())
}

pub fn create_routes() -> Router {
    Router::new()
        .route(&pre("/get_twitch_username"), get(get_twitch_username))
        .route(&pre("/get_all_timers"), get(get_all_timers))
        .route(&pre("/get_timer"), get(get_timer))
        .route(&pre("/get_timer_names"), get(get_timer_ids))
        .route(&pre("/post_create_timer"), post(post_create_timer))
        .route(&pre("/post_update_timer"), post(post_update_timer))
        .route(&pre("/post_button_pressed"), post(post_button_pressed))
        .route(&pre("/twitch_auth"), get(twitch_auth))
        .route(&pre("/is_connected_to_twitch"), get(connected_to_twitch))
        .route(&pre("/get_user"), get(get_user))
        .route(&pre("/update_user"), post(update_user))
        .route("/twitch_invalid", get(twitch_invalid))
        .route("/ws", get(ws_stuff))
        .with_state(RouteStates::default())
}

async fn handle_socket(socket: WebSocket, state: RouteStates) {
    let (tx, rx) = socket.split();

    let mut oneshot = tick_oneshot.lock();

    tokio::spawn(write(tx, state.clone()));
    tokio::spawn(read(rx, state.clone()));

    if *oneshot {
        *oneshot = false;
        let state_copy = state.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            loop {
                {
                    let tx = state_copy.tx.lock();
                    let _ = tx.send("tick".to_string());
                }
                interval.tick().await;
            }
        });
    }
}

async fn read(mut rec: SplitStream<WebSocket>, state: RouteStates) {
    while let Some(msg) = rec.next().await {
        match msg {
            Ok(Message::Text(_text)) => {}
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

    if let Err(err) = sen.send(Message::Ping(vec![1, 2, 3].into())).await {
        crate::error!("Client did not answer to ping... Error message: {err}");
        return;
    }

    // this is basically jus sending out whatever is sent on the broadcast channel
    while let Ok(msg) = rx.recv().await {
        if msg == "close" {
            break;
        }

        if let Err(e) = sen.send(Message::Text(msg.clone().into())).await {
            dbg!(e);
            break;
        }
    }
}
