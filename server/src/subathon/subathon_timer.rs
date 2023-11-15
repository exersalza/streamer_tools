use std::time::Duration;

use axum::response::IntoResponse;
use rand;
use rand::Rng;
use tokio::sync::oneshot;
use tokio::sync::oneshot::{Receiver, Sender};
use tokio::task::JoinHandle;
use tokio::time::sleep;

pub async fn subathon_timer() -> impl IntoResponse {
    let port = rand::thread_rng().gen_range(50000..50400);
    let addr = format!("wss://127.0.0.1:{port}");

    // open ws for the frontend to get

    // close ws if there is no connection in the past 5 minutes

    addr
}

#[derive()]
pub struct Tick {
    max: i64,
    tx: Option<Sender<&'static str>>,
    rx: Option<Receiver<&'static str>>,
}

impl Tick {
    pub fn new(max: i64) -> Self {
        Self {
            max,
            tx: None,
            rx: None,
        }
    }

    async fn runner(&mut self) {
        loop {
            if let Some(rx) = self.rx.as_mut() {
                match rx.try_recv() {
                    Ok(_) | Err(tokio::sync::oneshot::error::TryRecvError::Closed) => {
                        log::debug!("Loop break");
                        break;
                    }
                    _ => {}
                }
            }

            sleep(Duration::from_secs(1));
        }
    }

    pub async fn start(&mut self) {
        let (tx, rx) = oneshot::channel::<&'static str>();
        self.tx = Some(tx);
        self.rx = Some(rx);
    }
}
