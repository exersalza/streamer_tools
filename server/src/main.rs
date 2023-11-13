mod config;
mod subathon;
mod ws;

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
use serde::Deserialize;
use sqlite::State;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Mutex;
use tokio::fs;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use crate::config::Config;
use crate::subathon::subathon_timer::subathon_timer;

lazy_static! {
    static ref CONFIG: Mutex<Config> = Mutex::new(Config::new("./config.toml"));
}

#[derive(Parser, Debug)]
#[clap(name = "server", about = "a randomly spawned server")]
struct Opt {
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,

    /// Set the listen address
    #[clap(short = 'a', long = "addr", default_value = "::1")]
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

    let sql = Sql::new();

    let timer_endpoints = Router::new()
        .route("/api/timer", post(timer_post))
        .route("/api/timer/:id", get(timer_get))
        .route("/api/timer/:id", delete(timer_del))
        .route("/api/get_all_timer", get(timer_get_all))
        .route("/api/subathon_timer", get(subathon_timer));

    // enable consolel logging
    tracing_subscriber::fmt::init();

    let app: Router = Router::new()
        .route("/ping", get(pong))
        .merge(timer_endpoints)
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
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    log::info!("Web listening on http://{addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start server");

    log::info!("test");
}

struct Sql {
    conn: sqlite::Connection,
}

impl Sql {
    pub fn new() -> Self {
        let cfg = CONFIG.lock().unwrap();
        debug!("{}", cfg.sql_path.clone());
        let conn = sqlite::open(cfg.sql_path.clone()).unwrap();

        Self { conn }
    }

    pub fn get_all_timers(&self) -> Vec<(i64, String)> {
        let query = "select * from timers";
        let mut statement = self.conn.prepare(query).unwrap();
        let mut ret: Vec<(i64, String)> = Vec::new();

        while let Ok(sqlite::State::Row) = statement.next() {
            let item_id = statement.read::<i64, _>("timer_id").unwrap();
            let time = statement.read::<String, _>("time").unwrap();

            ret.push((item_id, time));
        }

        ret
    }

    /// Create timer with init time and id, already existing timer get overwritten
    pub fn create_timer(&self, timer: Timer) -> i32 {
        let query = format!(
            "INSERT INTO timers (timer_id, time)
VALUES ({}, '{}')
ON CONFLICT (timer_id)
DO UPDATE SET time = excluded.time;",
            timer.id, timer.time
        );

        self.conn.execute(query).unwrap();

        debug!("created {}", timer.id);

        timer.id
    }

    /// Delete a timer by its id
    pub fn delete_timer(&self, timer_id: i32) {
        let query = format!("delete from timers where id = {timer_id}");

        self.conn.execute(query).unwrap();

        debug!("deleted {}", timer_id);
    }

    /// get the stored time for an id
    pub fn get_time(&self, timer_id: i32) -> Option<Timer> {
        let query = format!("select * from timers where timer_id = {timer_id}");
        let mut statement = self.conn.prepare(query).unwrap();
        let mut timer = Timer {
            id: -1,
            time: "00:00:00".to_string(),
        };

        while let Ok(State::Row) = statement.next() {
            timer.id = statement.read::<i64, _>("timer_id").unwrap() as i32;
            timer.time = statement.read::<String, _>("time").unwrap();
        }

        // check if we have data
        if timer.id == -1 {
            return None;
        }

        Some(timer)
    }
}

pub struct Time {
    hours: i32,
    minutes: i32,
    seconds: i32
}

#[derive(Debug)]
pub struct Timer {
    id: i32,
    time: String,
}

impl Timer {
    pub fn parse(time: String) -> Option<Time> {
        let items: Vec<_> = time.split(':').collect();

        let hours: i32 = items[0].parse::<i32>().unwrap();
        let minutes: i32 = items[1].parse::<i32>().unwrap();
        let seconds: i32 = items[2].parse::<i32>().unwrap();


        Some(Time {hours, minutes, seconds})
    }
}

#[derive(Debug, Deserialize)]
struct TimerPostBody {
    id: i32,
    hours: i32,
    minutes: i32,
    seconds: i32,
}

async fn timer_get(axum_path(id): axum_path<i32>) -> impl IntoResponse {
    let time = Sql::new().get_time(id);

    if time.is_none() {
        return "No timer".to_string();
    }

    let timer: Timer = time.unwrap();

    timer.time
}

async fn timer_del(axum_path(id): axum_path<i32>) -> impl IntoResponse {
    debug!("del triggers");
    format!("{id}")
}

async fn timer_post(Json(data): Json<TimerPostBody>) -> impl IntoResponse {
    debug!("{data:?}");
    format!("post")
}

async fn timer_get_all() -> impl IntoResponse {
    debug!("get all timer");
    "all timer"
}

async fn pong() -> impl IntoResponse {
    "Pong"
}

// Response::builder()
//                     .status(StatusCode::INTERNAL_SERVER_ERROR)
//                     .body(boxed(Body::from(format!("error: {err}"))))
//                     .expect("error response")
