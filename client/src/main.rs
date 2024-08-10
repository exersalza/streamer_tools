use std::path::PathBuf;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use clap::Parser;
use tokio::{fs, net::TcpListener};
use tower::util::ServiceExt;
use tower_http::services::ServeDir;
use utils::Opt;

pub mod config;
pub mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let opt = Opt::parse();

    let app = Router::new().fallback_service(get(|req| async move {
        let res = ServeDir::new(&opt.static_dir).oneshot(req).await.unwrap(); // serve dir is infallible
        let status = res.status();
        match status {
            StatusCode::NOT_FOUND => {
                let index_path = PathBuf::from(&opt.static_dir).join("index.html");
                fs::read_to_string(index_path)
                    .await
                    .map(|index_content| (StatusCode::OK, Html(index_content)).into_response())
                    .unwrap_or_else(|_| {
                        (StatusCode::INTERNAL_SERVER_ERROR, "index.html not found").into_response()
                    })
            }

            _ => res.into_response(),
        }
    }));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    log::info!("test");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> impl IntoResponse {
    ""
}
