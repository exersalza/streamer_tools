use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Mutex;

use axum::extract::Path as axum_path;
use axum::response::Html;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, Router},
    Json,
};
use clap::Parser;
use lazy_static::lazy_static;
use log::debug;
use serde_derive::{Deserialize, Serialize};
use tokio::fs;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use crate::sql::{Sql};
use crate::subathon::subathon_timer::subathon_timer;
use crate::ws::ws_handler;
use shared::globals;

mod config;
mod sql;
mod subathon;
mod ws;

extern crate frontend;
use frontend::components::timer::*;

// lazy_static! {
//     pub static ref CONFIG: Mutex<Config> = Mutex::new(Config::new("./config.toml"));
// }

lazy_static!{
    pub static ref SQL: Mutex<Sql> = Mutex::new(Sql::new());
}

#[derive(Parser, Debug)]
#[clap(name = "server", about = "a randomly spawned server")]
struct Opt {
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,

    /// Set the listen address
    #[clap(short = 'a', long = "addr", default_value = "localhost")]
    addr: String,

    /// Set the port
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,

    /// set the static dir
    #[clap(long = "static-dir", default_value = "./dist")]
    static_dir: String,

    /// define config path
    #[clap(short = 'c', long = "config", default_value = "./config.toml")]
    config: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }

    let mut url_lock = globals::URL.lock().expect("can't lock");
    *url_lock = format!("{}:{}", opt.addr, opt.port);

    let timer_endpoints = Router::new()
        .route("/api/timer", post(timer_post))
        .route("/api/timer/:id", get(timer_get))
        .route("/api/timer/:id", delete(timer_del))
        .route("/api/get_all_timer", get(timer_get_all))
        .route("/api/subathon_timer", get(subathon_timer));

    let cors = CorsLayer::new()
        .allow_methods(Any) // Allow all methods
        .allow_origin(Any) // Allow all origins
        .allow_headers(Any) // Allow all headers
        .expose_headers(Any);

    // enable consolel logging
    tracing_subscriber::fmt::init();

    let app: Router = Router::new()
        .route("/ping", get(pong))
        .route("/ws/:t", get(ws_handler))
        .merge(timer_endpoints)
        .layer(cors)
        .fallback_service(get(|req| async move {
            let res = ServeDir::new(&opt.static_dir).oneshot(req).await.unwrap(); // serve dir is infallible
            let status = res.status();
            match status {
                StatusCode::NOT_FOUND => {
                    let index_path = PathBuf::from(&opt.static_dir).join("index.html");
                    fs::read_to_string(index_path)
                        .await
                        .map(|index_content| (StatusCode::OK, Html(index_content)).into_response())
                        .unwrap_or_else(|_| {
                            (StatusCode::INTERNAL_SERVER_ERROR, "index.html not found")
                                .into_response()
                        })
                }

                _ => res.into_response(),
            }
        }))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST)),
        opt.port,
    ));

    log::info!("Web listening on http://{addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("Unable to start server");
}


#[derive(Debug, Deserialize)]
struct TimerPostBody {
    id: i32,
    hours: i32,
    minutes: i32,
    seconds: i32,
}

#[derive(Serialize)]
struct GetAllResponse {
    body: HashMap<i64, i64>,
}

// wrongly named here, we just get the time that the object holds
async fn timer_get(axum_path(id): axum_path<i32>) -> impl IntoResponse {
    let time = SQL.lock().expect("Can't lock").get_time(id);

    if time.is_none() {
        return "No timer".to_string();
    }

    // ._.
    time.unwrap().timer.to_seconds().to_string()
}

async fn timer_del(axum_path(id): axum_path<i32>) -> impl IntoResponse {
    debug!("del triggers");
    SQL.lock().expect("Can't lock").delete_timer(id);
    format!("{id}")
}

async fn timer_post(Json(data): Json<TimerPostBody>) -> impl IntoResponse {
    debug!("post {data:#?}");
    let mut timer: Timer = Timer::new();
    timer.convert_and_insert(data.id, data.hours, data.minutes, data.seconds);
    SQL.lock().expect("Can't lock").create_timer(&timer);

    timer.id.to_string()
}

async fn timer_get_all() -> impl IntoResponse {
    debug!("get all timer");

    let timers = SQL.lock().expect("Can't lock").get_all_timers();
    if timers.len() == 0 {
        return (StatusCode::OK, serde_json::to_string("{}").unwrap());
    }

    let mut ret: HashMap<i64, i64> = HashMap::new();

    for (key, value) in timers {
        ret.insert(key, value);
    }

    (
        StatusCode::OK,
        serde_json::to_string(&ret).expect("Failed to create json"),
    )
}

async fn timer_update(axum_path(id): axum_path<i32>) -> impl IntoResponse {}


async fn pong() -> impl IntoResponse {
    "Pong"
}
// Response::builder()
//                     .status(StatusCode::INTERNAL_SERVER_ERROR)
//                     .body(boxed(Body::from(format!("error: {err}"))))
//                     .expect("error response")
