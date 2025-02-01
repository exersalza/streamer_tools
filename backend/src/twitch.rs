use futures::{SinkExt, StreamExt};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{client::IntoClientRequest, Message},
};

pub struct Twitch {}

impl Twitch {
    pub async fn new() -> Self {
        // twitch websocket shit
        let req = "wss://eventsub.wss.twitch.tv/ws"
            .into_client_request()
            .unwrap();

        let (mut stream, _res) = connect_async(req).await.unwrap();

        stream.send(Message::Text("".into())).await.unwrap();

        // Receive messages
        while let Some(msg) = stream.next().await {
            if let Ok(Message::Text(text)) = msg {
                println!("Received: {}", text);
            }
        }
        Self {}
    }
}
