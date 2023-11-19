use std::time::Duration;

use axum::response::IntoResponse;
use rand;
use tokio::sync::oneshot;
use tokio::time::sleep;

const WS_PORT: i32 = 8080;

pub async fn subathon_timer() -> impl IntoResponse {
    let addr = format!("ws://127.0.0.1:{WS_PORT}/ws");
    // let (tx, mut rx) = mpsc::channel(32);
    // open ws for the frontend to get

    // close ws if there is no connection in the past 5 minutes

    addr
}


#[derive()]
pub struct Tick {
    max: i64,
    // tx: Option<Sender<&'static str>>,
    // rx: Option<Receiver<&'static str>>,
}

impl Tick {
    pub fn new(max: i64) -> Self {
        Self {
            max,
            // tx: None,
            // rx: None,
        }
    }

    pub async fn runner() {
        loop {
            log::info!("tiCk");

            sleep(Duration::from_secs(1)).await;
        }
    }

    pub fn start(&self) {
        let (tx, rx) = oneshot::channel::<&'static str>();

        tokio::spawn(Self::runner());
    }
}
