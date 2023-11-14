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


pub struct Tick {
    max: i64,
    thread_handle: i32,
    tx: Option<Sender<()>>,
    rx: Option<Receiver<()>>,
    task: Option<JoinHandle<()>>,
}

impl Tick {
    pub fn new(max: i64) -> Self {
        let (tx, rx) = oneshot::channel::<()>();
        let task = start_loop();
        Self { max, thread_handle: 32, tx: Some(tx), rx: Some(rx), task: Some(task) }
    }

    /// starts a new loop when ::new is called
    fn start_loop(self) -> JoinHandle<()> {
        let mut l_rx = self.rx.unwrap();

       tokio::spawn(async move {
            loop {
                println!("doing some stuff");

                if let Ok(_) = l_rx.try_recv() {
                    break;
                }

                sleep(Duration::from_secs(1)).await
            }
        })
    }

    /// kills the loop
    pub fn kill_loop(&self) {
        if let Ok(_) = self.tx {
           self.tx.unwrap().send(());
        }
    }
}

#[tokio::main]
async fn main() {
    let subathon_timer = Tick::new(0);

    let t = tokio::spawn(async {
        sleep(Duration::from_secs(5)).await;
        subathon_timer.kill_loop();
    });
}
