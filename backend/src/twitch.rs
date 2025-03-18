use core::{fmt, str};
use std::{any::Any, collections::HashMap, sync::Arc};

use anyhow::Result;
use futures::{SinkExt, StreamExt};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use tokio::task::JoinHandle;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{client::IntoClientRequest, Message},
};

use crate::{
    config::{self, AM},
    sql::SQL,
};

const KEEPALIVE_TIMEOUT: i32 = 10;
const MSG_LOG_LEN: usize = 256;

lazy_static! {
    static ref last_message: AM<String> = Arc::new(Mutex::new(
        // get the startup time, bc the loop managing the reconnect would break with the start of
        // the unix time
        chrono::offset::Utc::now().to_rfc3339().to_string()
    ));
    static ref msg_id_log: AM<Vec<String>> = Arc::new(Mutex::new(Vec::with_capacity(MSG_LOG_LEN)));
    static ref current_ws_id: AM<String> = Arc::new(Mutex::new("".to_string()));
    static ref TWITCH_EVENTS: Vec<&'static str> = vec![];
    pub static ref rew_cl: reqwest::Client = reqwest::Client::new();
}

pub struct Twitch {}

#[derive(Deserialize, Debug)]
struct MetaData {
    message_id: String,
    message_type: String,
    message_timestamp: String,
}

#[derive(Deserialize, Debug)]
struct Session {
    id: String,
    status: String,
    connected_at: String,
    keepalive_timeout_seconds: i32,
    reconnect_url: Option<String>,
}

#[derive(Deserialize, Debug)]
struct MofoginWelcomeThingi {
    payload: WelcomePayload,
}

#[derive(Deserialize, Debug)]
struct WelcomePayload {
    session: Session,
}

#[derive(Deserialize, Debug)]
struct InitResponse {
    metadata: MetaData,
}

#[derive(Deserialize, Debug)]
struct GetUser {
    data: Vec<User>,
}

// https://dev.twitch.tv/docs/api/reference/#get-users
#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: String,
    pub login: String,
    pub display_name: String,
    pub broadcaster_type: String,
    pub profile_image_url: String,
}

pub async fn update_user_in_db<T: fmt::Display>(login: T) -> Result<()> {
    let id = crate::config!().twitch.client_id.clone();
    let (token, _) = SQL.get_bot_oauth().await.unwrap();

    let f = rew_cl
        .get(format!("https://api.twitch.tv/helix/users?login={login}"))
        .header("Authorization", format!("Bearer {}", token))
        .header("Client-ID", id)
        .send()
        .await?
        .text()
        .await?;

    SQL.update_user(
        serde_json::from_str::<GetUser>(&f)
            .unwrap()
            .data
            .first()
            .unwrap(),
    )
    .await
    .unwrap();

    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct OAuthRes {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

pub async fn get_oauth() -> Result<OAuthRes> {
    let mut fo: HashMap<&str, String> = HashMap::new();
    let twitch = crate::config!().twitch.clone();

    fo.insert("client_id", twitch.client_id);
    fo.insert("client_secret", twitch.client_secret);
    fo.insert("grant_type", "client_credentials".to_string());

    let f = rew_cl
        .post("https://id.twitch.tv/oauth2/token")
        .form(&fo)
        .send()
        .await?
        .text()
        .await?;

    Ok(serde_json::from_str(&f)?)
}

pub async fn get_and_store_oauth() -> Result<()> {
    if let Ok(Some(time)) = SQL.get_expires_in_oauth().await {
        let now = chrono::Utc::now().timestamp();
        // trigger if the token is valid for another day
        if (time - now) >= (60 * 60 * 24) {
            return Ok(());
        }
    }

    let res = get_oauth().await?;

    let now = chrono::Utc::now();
    let time = now + chrono::Duration::seconds(res.expires_in);
    SQL.update_oauth_data(res.access_token, time, res.token_type)
        .await?;

    Ok(())
}

async fn regit_twitch_events() -> anyhow::Result<()> {
    let user_id = SQL.get_user().await.unwrap().unwrap().id;
    let token = SQL.get_twitch_user_token().await.unwrap();
    let client_id = crate::config!().twitch.client_id.clone();

    let mut events = vec![""];

    let cur_ws_id = current_ws_id.lock().clone();

    let transport = json!({"method": "websocket", "session_id": cur_ws_id.clone()});

    let f = json!({
        "type": "channel.follow",
        "version": "2",
        "condition": {
            "broadcaster_user_id": user_id,
            "moderator_user_id": user_id
        },
        "transport": transport
    });

    dbg!(&f);
    dbg!(&token);

    let res = rew_cl
        .post("https://api.twitch.tv/helix/eventsub/subscriptions")
        .header("Content-Type", "application/json")
        .header("Client-Id", client_id)
        .header("Authorization", format!("Bearer {}", token.unwrap()))
        .body(serde_json::to_string(&f).unwrap_or("{}".to_string()))
        .send()
        .await?
        .text()
        .await?;

    dbg!(res);
    Ok(())
}

fn start_message_watchdog() {
    tokio::spawn(async {
        // we do 11 secs here instead of 10, so we can have a puffer if needed
        let mut inter = tokio::time::interval(tokio::time::Duration::from_secs(10));

        loop {
            inter.tick().await;

            let last_time = last_message.lock();
            //dbg!(&last_time);
            let date = chrono::DateTime::parse_from_rfc3339(&last_time)
                // this should be infallible bc its coming from the twitch api, maybe fix later
                .unwrap()
                .timestamp();
            let now = chrono::Utc::now().timestamp();

            if (now - date) > 10 {
                // TODO: implement reconnect
                dbg!("implement reconnect");
                break;
            }
        }
    });
}

impl Twitch {
    pub async fn new() -> Self {
        let _ = get_and_store_oauth().await;
        Self {}
    }

    pub async fn connect() {
        // twitch websocket shit
        let req = "wss://eventsub.wss.twitch.tv/ws"
            .into_client_request()
            .unwrap();

        let (mut stream, _res) = connect_async(req).await.unwrap();
        let mut current_threads = vec![];

        start_message_watchdog();
        // Receive messages
        while let Some(msg) = stream.next().await {
            match msg.unwrap().clone() {
                Message::Text(text) => {
                    let metadata: MetaData = serde_json::from_str::<InitResponse>(
                        str::from_utf8(text.as_bytes()).unwrap(),
                    )
                    .unwrap()
                    .metadata;

                    let mut msg_log = msg_id_log.lock();
                    // twitch sometimes send duplicated messages
                    if msg_log.contains(&metadata.message_id) {
                        continue;
                    }

                    msg_log.push(metadata.message_id);
                    if msg_log.len() >= MSG_LOG_LEN {
                        msg_log.pop();
                    }

                    let mut lst_msg = last_message.lock();
                    *lst_msg = metadata.message_timestamp;

                    match metadata.message_type.as_str() {
                        "session_reconnect" => {}
                        "session_welcome" => {
                            //dbg!(&text);
                            let session: Session = serde_json::from_str::<MofoginWelcomeThingi>(
                                str::from_utf8(text.as_bytes()).unwrap(),
                            )
                            .unwrap()
                            .payload
                            .session;

                            let mut cur_id = current_ws_id.lock();
                            *cur_id = session.id;
                            drop(cur_id);

                            current_threads.push(tokio::spawn(regit_twitch_events()));
                        }
                        "session_keepalive" => {
                            crate::debug!("[twitch] heartbeat");
                        }
                        _ => {
                            dbg!(&text);
                        }
                    }
                    //sub_to_events(payload.payload.session.id).await;
                }
                Message::Close(e) => {
                    dbg!("close ws", e);
                    break;
                }
                Message::Ping(e) => {
                    crate::debug!("[twitch] received ping...");
                    let _ = stream.send(Message::Pong(e)).await;
                    crate::debug!("[twitch] sent pong...");
                }
                _ => {
                    dbg!("some default");
                }
            }
        }
    }
}
