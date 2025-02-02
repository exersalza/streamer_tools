use core::str;

use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::json;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{client::IntoClientRequest, Message},
};

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
struct Payload {
    session: Session,
}

#[derive(Deserialize, Debug)]
struct InitResponse {
    metadata: MetaData,
    payload: Payload,
}

impl Twitch {
    pub async fn new() -> Self {
        // twitch websocket shit
        let req = "wss://eventsub.wss.twitch.tv/ws"
            .into_client_request()
            .unwrap();

        let (mut stream, _res) = connect_async(req).await.unwrap();

        stream.send(Message::Text("".into())).await.unwrap();
        let mut data: InitResponse;

        // Receive messages
        while let Some(msg) = stream.next().await {
            if let Ok(Message::Text(text)) = msg {
                let json =
                    serde_json::from_str::<InitResponse>(str::from_utf8(text.as_bytes()).unwrap());
                dbg!("adding data");
                break;
            }
        }
        Self {}
    }
}
