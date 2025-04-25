use anyhow::bail;
/// this gonna be a messy file, dw about it
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, sync::Arc, thread, time::Duration};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    http::{HeaderMap, StatusCode},
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
    pub static ref ws_write_fn: AM<broadcast::Sender<String>> = Arc::new(Mutex::new({
        let (tx, _) = broadcast::channel(254);
        tx
    }));
    static ref tick_oneshot: AM<bool> = Arc::new(Mutex::new(true));
    pub static ref running_timer: AM<HashMap<String, i32>> = Arc::new(Mutex::new(HashMap::new()));
}

const API_VERSION: &str = "v1";

#[derive(Deserialize, Clone, Debug)]
pub struct FetchTimer {
    uuid: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ButtonPressed {
    id: String,
    function: ButtonFunction,
}

#[derive(Clone)]
pub struct RouteStates {
    pub tx: Arc<Mutex<broadcast::Sender<String>>>,
}

impl RouteStates {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(254);

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
    dbg!(&payload);

    let _ = match payload.function {
        ButtonFunction::M5 => todo!(),
        ButtonFunction::M1 => todo!(),
        ButtonFunction::Stop => SQL.set_timer_active(payload.id, false).await,
        ButtonFunction::Play => SQL.set_timer_active(payload.id, true).await,
        ButtonFunction::P1 => todo!(),
        ButtonFunction::P5 => todo!(),
    };

    "passed"
}

async fn post_update_timer(Json(payload): Json<Timer>) -> impl IntoResponse {
    ""
}

#[derive(Deserialize)]
struct TimerActive {
    id: String,
}

async fn post_toggle_timer_active(Json(payload): Json<TimerActive>) -> impl IntoResponse {
    format!("{:?}", SQL.toggle_timer_active(payload.id).await)
}

async fn get_timer_ids() -> impl IntoResponse {
    serde_json::to_string(&SQL.get_timer_ids().await.unwrap()).unwrap()
}

async fn ws_stuff(
    ws: WebSocketUpgrade,
    header: HeaderMap,
    State(state): State<RouteStates>,
) -> Response {
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

#[derive(Deserialize, Debug, Clone)]
pub struct AuthTokenResponseOk {
    pub access_token: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub scope: Vec<String>,
    pub token_type: String,
}

struct AuthTokenResponseNotOk {}

async fn refresh_twitch_token() -> anyhow::Result<Option<String>> {
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

    return match SQL.update_user_access_token(ff.clone()).await {
        Ok(_) => Ok(Some(ff.access_token)),
        Err(e) => bail!("Failed to update user access token in db. Error: {e}"),
    };
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

async fn get_active_timers() -> impl IntoResponse {
    if let Ok(d) = SQL.get_active_timer().await {
        return serde_json::to_string(&d).unwrap_or_default();
    }

    String::from("borke")
}

pub fn create_routes() -> Router {
    Router::new()
        .route(&pre("/get_twitch_username"), get(get_twitch_username))
        .route(&pre("/get_all_timers"), get(get_all_timers))
        .route(&pre("/get_timer"), get(get_timer))
        .route(&pre("/get_timer_names"), get(get_timer_ids))
        .route(&pre("/post_create_timer"), post(post_create_timer))
        .route(&pre("/post_update_timer"), post(post_update_timer))
        .route(
            &pre("/post_toggle_timer_active"),
            post(post_toggle_timer_active),
        )
        .route(&pre("/post_button_pressed"), post(post_button_pressed))
        .route(&pre("/twitch_auth"), get(twitch_auth))
        .route(&pre("/is_connected_to_twitch"), get(connected_to_twitch))
        .route(&pre("/get_user"), get(get_user))
        .route(&pre("/update_user"), post(update_user))
        .route(&pre("/ping"), get(async || "pong"))
        .route(&pre("/get_active_timers"), get(get_active_timers))
        .route("/twitch_invalid", get(twitch_invalid))
        .route("/ws", get(ws_stuff))
        .with_state(RouteStates::default())
}

async fn handle_socket(socket: WebSocket, state: RouteStates) {
    let (tx, rx) = socket.split();

    let mut oneshot = tick_oneshot.lock();
    let id = uuid::Uuid::new_v4();

    tokio::spawn(write(tx, state.clone(), id));
    tokio::spawn(read(rx, state.clone(), id));

    if *oneshot {
        *oneshot = false;
        let state_copy = state.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            loop {
                let data = (SQL.get_active_timer().await).unwrap_or_default();

                let payload_data = json!({
                    "type": "tick",
                    "payload": data
                });

                {
                    let tx = state_copy.tx.lock();
                    let _ = tx.send(payload_data.to_string());
                }

                let _ = SQL.dec_all_timer().await;
                interval.tick().await;
            }
        });
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "action")]
enum Action {
    Dec,
    Inc,
    Reg,
    UnReg,
}

#[derive(Deserialize, Debug)]
struct WsPayload {
    id: String,
    payload: Action,
}

async fn read(mut rec: SplitStream<WebSocket>, state: RouteStates, id: uuid::Uuid) {
    while let Some(msg) = rec.next().await {
        match msg {
            Ok(Message::Text(text)) => match serde_json::from_str::<WsPayload>(&text.to_string()) {
                Ok(v) => match v.payload {
                    Action::Dec => {}
                    Action::Inc => {}
                    // this keeps track of the id counts so we know when to update an id and when
                    // not to, hopefully i'll still know when i update the update mechanism
                    Action::Reg => {
                        let mut lock = running_timer.lock();

                        if let Some(f) = lock.get_mut(&v.id) {
                            *f += 1;
                            return;
                        }

                        lock.insert(v.id.clone(), 1);
                    }
                    Action::UnReg => {
                        let mut lock = running_timer.lock();

                        if let Some(f) = lock.get_mut(&v.id) {
                            if *f > 0 {
                                *f -= 1;
                            }
                        }
                    }
                },
                Err(e) => {
                    let tx = state.tx.lock();
                    // we dont care about this Result here, bc it's pretty useless on the backend,
                    // its just to tell the frontend that it should start formatting its shit
                    // right.
                    let _ = tx.send(format!("Couldn't decode what ever the fuck you send. {e}"));
                    continue;
                }
            },
            Ok(Message::Close(_)) => {
                let tx = state.tx.lock();
                let _ = tx.send(format!("close-{id}"));
                break;
            }
            Err(e) => println!("{e:?}"),
            _ => (),
        }
    }
}

async fn write(mut sen: SplitSink<WebSocket, Message>, state: RouteStates, id: uuid::Uuid) {
    let mut rx = state.tx.lock().subscribe();

    if let Err(err) = sen.send(Message::Ping(vec![1, 2, 3].into())).await {
        crate::error!("Client did not answer to ping... Error message: {err}");
        return;
    }

    // this is basically jus sending out whatever is sent on the broadcast channel
    while let Ok(msg) = rx.recv().await {
        if msg == format!("close-{id}") {
            break;
        }

        if let Err(e) = sen.send(Message::Text(msg.clone().into())).await {
            dbg!(e);
            break;
        }
    }
}
