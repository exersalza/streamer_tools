use std::net::TcpListener;

use axum::Router;

async fn root() -> &str {
    "hello"
}

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(root));

    let listener = TcpListener::bind("0.0.0.0:72727").await.unwrap();
}
