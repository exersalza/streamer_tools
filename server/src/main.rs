mod ws;
mod subathon;

use axum::{
    Json,
    body::{boxed, Body},
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::{get, Router, post, delete},
};
use std::path::{Path, PathBuf};
use tokio::{fs, task_local};
use clap::Parser;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::str::FromStr;
use axum::extract::{Path as axum_path, Query};
use axum::response::Html;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use serde::Deserialize;
use tracing_subscriber::fmt::format;

use crate::subathon::subathon_timer::subathon_timer;

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

    let sql = Sql::new("./settings.db".to_string());

    sql.get_all_timers();

    // enable consolel logging
    tracing_subscriber::fmt::init();

    let app: Router = Router::new()
        .route("/api/hello", get(hello))
        .route("/api/timer/:id", get(timer))
        .route("/api/timer/:id", delete(timer_del))
        .route("/api/timer", post(timer_post))
        .route("/api/subathon_timer", get(subathon_timer))
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
    conn: sqlite::Connection
}

impl Sql {
    pub fn new(path: String) -> Self {
        let conn = sqlite::open(path).unwrap();

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

    pub fn create_timer(&self, timer: Timer) -> i32 {
        let query = format!("INSERT INTO timers (timer_id, time)
VALUES ({}, '{}')
ON CONFLICT (timer_id)
DO UPDATE SET time = excluded.time;", timer.id, timer.time);


        self.conn.execute(query).unwrap();

        timer.id
    }

    pub fn delete_timer(&self, timer_id: i32) {
        let query = format!("delete from timers where id = {timer_id}");
    }

    pub fn get_time(&self, timer_id: i32) -> Option<Timer> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Timer {
    id: i32,
    time: String
}

#[derive(Debug, Deserialize)]
struct TimerPostData {
    id: i32,
    hours: i32,
    minutes: i32,
    seconds: i32
}

async fn timer(axum_path(id): axum_path<i32>) -> impl IntoResponse {
    format!("{id}")
}

async fn timer_del(axum_path(id): axum_path<i32>) -> impl IntoResponse {
    log::debug!("del triggers");
    format!("{id}")
}

async fn timer_post(Json(data): Json<TimerPostData>) -> impl IntoResponse  {
    log::debug!("{data:?}");
    format!("post")
}

async fn hello() -> impl IntoResponse {
    "hello from da other side"
}

// Response::builder()
//                     .status(StatusCode::INTERNAL_SERVER_ERROR)
//                     .body(boxed(Body::from(format!("error: {err}"))))
//                     .expect("error response")