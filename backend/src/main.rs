pub mod config;
pub mod logs;
pub mod macros;
pub mod oauth;
pub mod routes;
pub mod sql;
pub mod twitch;
pub mod utils;

use std::sync::Arc;

use axum::{
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
    Router,
};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use twitch::Twitch;

type AM<T> = Arc<Mutex<T>>;

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
    tokio::spawn(twitch::Twitch::connect());

    let listener = TcpListener::bind("0.0.0.0:22727").await.unwrap();
    println!("starting api");
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
