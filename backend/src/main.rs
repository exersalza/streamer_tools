pub mod config;
pub mod logs;
pub mod macros;
pub mod oauth;
pub mod routes;
pub mod sql;
pub mod twitch;
pub mod utils;

use std::sync::Arc;

use axum::{response::Redirect, routing::get, Router};
use parking_lot::Mutex;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use twitch::Twitch;

type AM<T> = Arc<Mutex<T>>;

const RECONNECT_ATTEMPTS: i32 = 3;
const RECONNECT_AFTER: u64 = 30; // in seconds

async fn root() -> Redirect {
    Redirect::to("http://localhost:5173")
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods(Any) // Allow all methods
        .allow_origin(Any) // Allow all origins
        .allow_headers(Any) // Allow all headers
        .expose_headers(Any);

    let router = Router::new()
        .route("/", get(root))
        .merge(routes::create_routes())
        .layer(cors);

    //get_oauth().await;
    let twitch_cl = Twitch::new().await;

    // connect to twitch websocket to receive events and stuff
    tokio::spawn(async {
        let mut i = tokio::time::interval(tokio::time::Duration::from_secs(RECONNECT_AFTER));
        let mut attempts = 0;

        loop {
            if attempts >= RECONNECT_ATTEMPTS {
                crate::error!("Failed after {RECONNECT_ATTEMPTS}, wont try again until restart...");
                break;
            }
            crate::debug!("connecting to websocket...");

            match twitch::Twitch::connect().await {
                Err(e) => {
                    attempts += 1;
                    crate::error!(
                        "Websocket failed unexpectly. Error code: {e}. trying to reconnect in 30 seconds..."
                    );
                }
                Ok(_) => {
                    attempts = 0;
                }
            }
            i.tick().await;
        }
    });

    let listener = TcpListener::bind("0.0.0.0:22727").await.unwrap();
    println!("starting api");
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
