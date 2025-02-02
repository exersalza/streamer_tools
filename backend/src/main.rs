pub mod config;
pub mod macros;
pub mod oauth;
pub mod routes;
pub mod sql;
pub mod twitch;
pub mod utils;

use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use twitch::Twitch;

async fn root() -> String {
    "hello".to_string()
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

    Twitch::new().await;

    let listener = TcpListener::bind("0.0.0.0:22727").await.unwrap();
    println!("starting api");
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
