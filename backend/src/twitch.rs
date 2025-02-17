use core::{fmt, str};
use std::sync::Arc;

use anyhow::Result;
use futures::{SinkExt, StreamExt};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use serde::Deserialize;
use serde_json::json;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{client::IntoClientRequest, Message},
};

use crate::{config::AM, sql::SQL};

const KEEPALIVE_TIMEOUT: i32 = 10;
const MSG_LOG_LEN: usize = 256;

lazy_static! {
    static ref last_message: AM<String> =
        Arc::new(Mutex::new(String::from("1970-01-01T01:01:01.000000000Z")));
    static ref msg_id_log: AM<Vec<String>> = Arc::new(Mutex::new(Vec::with_capacity(MSG_LOG_LEN)));
    static ref current_ws_id: AM<String> = Arc::new(Mutex::new("".to_string()));
    static ref TWITCH_EVENTS: Vec<&'static str> = vec![];
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

pub async fn get_user_id<T: fmt::Display>(login: T) -> Result<()> {
    let token = SQL.get_twitch_user_token();

    let f = reqwest::get(format!("https://api.twitch.tv/helix/users?login={login}"))
        .await?
        .text()
        .await?;

    Ok(())
}

fn regit_twitch_events() {}

fn start_message_watchdog() {
    tokio::spawn(async {
        // we do 11 secs here instead of 10, so we can have a puffer if needed
        let mut inter = tokio::time::interval(tokio::time::Duration::from_secs(10));

        loop {
            inter.tick().await;

            let last_time = last_message.lock();
            dbg!(&last_time);
            let date = chrono::DateTime::parse_from_rfc3339(&last_time)
                // this should be infallible bc its coming from the twitch api, maybe fix later
                .unwrap()
                .timestamp();
            let now = chrono::Utc::now().timestamp();

            dbg!(now - date);
        }
    });
}

impl Twitch {
    pub async fn new() -> Self {
        // twitch websocket shit
        let req = "wss://eventsub.wss.twitch.tv/ws"
            .into_client_request()
            .unwrap();

        let (mut stream, _res) = connect_async(req).await.unwrap();

        //stream.send(Message::Text("".into())).await.unwrap();
        let mut data: InitResponse;

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
                            dbg!(&text);
                            let session: Session = serde_json::from_str::<MofoginWelcomeThingi>(
                                str::from_utf8(text.as_bytes()).unwrap(),
                            )
                            .unwrap()
                            .payload
                            .session;

                            let mut cur_id = current_ws_id.lock();
                            *cur_id = session.id;

                            regit_twitch_events();
                        }
                        "session_keepalive" => {}
                        _ => {}
                    }
                    //sub_to_events(payload.payload.session.id).await;
                }
                Message::Close(e) => {
                    dbg!("close ws", e);
                }
                Message::Ping(e) => {
                    dbg!("send pong", &e);
                    let _ = stream.send(Message::Pong(e)).await;
                }
                _ => {
                    dbg!("some default");
                }
            }
        }
        Self {}
    }
}
