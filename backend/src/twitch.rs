use core::{fmt, str};
use std::{collections::HashMap, sync::Arc};

use anyhow::{Result, bail};
use axum::http::status;
use futures::{SinkExt, StreamExt};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Message, client::IntoClientRequest},
};

use crate::{config::AM, error, sql::SQL};

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

#[derive(Deserialize, Debug)]
struct TokenJson {
    refresh_token: Option<String>,
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
    // BUG: can't reload if the timer is not in specific range
    //if let Ok(Some(time)) = SQL.get_expires_in_oauth().await {
    //    let now = chrono::Utc::now().timestamp();
    //    // trigger if the token is valid for another day
    //    if (time - now) >= 86400 {
    //        return Ok(());
    //    }
    //}

    let res = get_oauth().await?;

    let now = chrono::Utc::now();
    let time = now + chrono::Duration::seconds(res.expires_in);
    SQL.update_oauth_data(res.access_token, time, res.token_type)
        .await?;

    Ok(())
}

async fn regit_twitch_events() -> anyhow::Result<()> {
    let user_id = match SQL.get_user().await.unwrap() {
        Some(user) => user.id,
        None => {
            eprintln!("User has to link on the website");
            bail!("")
        }
    };

    let token = match SQL.get_twitch_user_token().await.unwrap() {
        Some(t) => t,
        None => {
            eprintln!("No token, user has to log in again");
            bail!("")
        }
    };
    dbg!(&token, &user_id);
    let client_id = crate::config!().twitch.client_id.clone();

    let events = vec![""];

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
        .header("Authorization", format!("Bearer {}", token))
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

/// Filter the big array out
#[derive(Serialize, Deserialize, Debug)]
struct NotifPayload {
    payload: NotificationPayload,
}

/// payload contents
#[derive(Serialize, Deserialize, Debug)]
struct NotificationPayload {
    subscription: SubNotifPayload,
    event: NotifEventData,
}

#[derive(Serialize, Deserialize, Debug)]
struct SubNotifPayload {
    id: String,
    status: String,
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct NotifEventData {
    broadcaster_user_id: String,
    broadcaster_user_login: String,
    broadcaster_user_name: String,
    followed_at: String,
    user_id: String,
    user_login: String,
    user_name: String,
}

fn handle_notification(text: String) {
    let parsed: NotifPayload = serde_json::from_str(&text).unwrap();
}

async fn is_token_valid(token: &str) -> bool {
    let is_valid = match rew_cl
        .get("https://id.twitch.tv/oauth2/validate")
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
    {
        Ok(v) => v,
        Err(e) => {
            error!("can't validate token: {e}");
            return false;
        }
    };

    is_valid.status() != status::StatusCode::UNAUTHORIZED
}

async fn refresh_token(
    token: String,
    refresh_token: String,
    expires_in: String,
) -> (String, String, String) {
    let def_ret = (token, refresh_token, expires_in);
    let twitch_cfg = &crate::config!().twitch;
    let refresh_token = match SQL.get_refresh_token().await {
        Ok(v) => v,
        Err(e) => {
            error!("can't get refresh token {e}");
            return def_ret;
        }
    };

    let body = json!({
        "client_id": twitch_cfg.client_id,
        "client_secret": twitch_cfg.client_secret,
        "grant_type": "refresh_token",
        "refresh_token": refresh_token
    })
    .to_string();

    let token_res = match rew_cl
        .post("https://id.twitch.tv/oauth2/token")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
    {
        Ok(v) => v,
        Err(e) => {
            dbg!(e);
            return def_ret;
        }
    };

    if token_res.status() != StatusCode::OK {
        error!(
            "refresh token request failed: {}",
            token_res.text().await.unwrap()
        );
        return def_ret;
    }

    let token_json = match token_res.text().await {
        Ok(v) => match serde_json::from_str::<TokenJson>(dbg!(v.as_str())) {
            Ok(v) => v,
            Err(e) => {
                error!("token_json is invalid json: {e}");
                return def_ret;
            }
        },
        Err(e) => {
            error!("new token res text is invalid: {e}");
            return def_ret;
        }
    };

    if token_json.refresh_token.is_none() {
        error!("something went wrong getting the refresh token, token is `none`");
    }

    def_ret
}

impl Twitch {
    pub async fn new() -> Self {
        let (mut token, token_type) = if let Ok(tok) = SQL.get_bot_oauth().await {
            tok
        } else {
            ("gibberish".to_string(), "Bearer".to_string())
        };
        dbg!(&token, &token_type);

        if !is_token_valid(&token).await {
            let (token, refresh_token, expires_in) =
                    // TODO: do this
                refresh_token(token, String::new(), String::new()).await;
        }

        //let _ = get_and_store_oauth().await;
        Self {}
    }

    pub async fn connect() -> anyhow::Result<()> {
        // twitch websocket shit

        // change the ws url depending on the build, if we're on the debug build, we only want the
        // localhost websocket server as on the real server the events are not going through
        let req = (if cfg!(debug_assertions) {
            "ws://127.0.0.1:8080/ws"
        } else {
            "wss://eventsub.wss.twitch.tv/ws"
        })
        .into_client_request()
        .unwrap();

        let (mut stream, _res) = connect_async(req).await?;
        let mut current_threads = vec![];

        start_message_watchdog();
        // Receive messages
        while let Some(msg) = stream.next().await {
            match msg?.clone() {
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
                        "notification" => handle_notification(text.to_string()),
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
                    let _ = stream.send(Message::Pong(e)).await;
                }
                _ => {
                    dbg!("some default");
                }
            }
        }
        Ok(())
    }
}
