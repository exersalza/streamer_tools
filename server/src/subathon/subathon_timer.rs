use std::net::Ipv4Addr;
use axum::response::IntoResponse;
use rand;
use rand::Rng;
use tracing_subscriber::fmt::format;

pub async fn subathon_timer() -> impl IntoResponse {
    let port = rand::thread_rng().gen_range(50000..50400);
    let addr = format!("wss://127.0.0.1:{port}");

    addr
}


