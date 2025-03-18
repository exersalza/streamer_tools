pub mod config;
pub mod logs;
pub mod macros;
pub mod oauth;
pub mod routes;
pub mod sql;
pub mod twitch;
pub mod utils;

use axum::{routing::get, Router};
use sql::SQL;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use twitch::{get_and_store_oauth, get_oauth, update_user_in_db, Twitch};

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

    //get_oauth().await;
    let twitch_cl = Twitch::new().await;
    tokio::spawn(twitch::Twitch::connect());

    let listener = TcpListener::bind("0.0.0.0:22727").await.unwrap();
    println!("starting api");
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
