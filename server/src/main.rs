use axum::{
    response::IntoResponse,
    routing::{get, Router},
};
use clap::Parser;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::str::FromStr;

#[derive(Parser, Debug)]
#[clap(name = "server", about = "a randomly spawned server")]
struct Opt {
    /// Set the listen address
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,

    /// Set the port
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();
    let app: Router = Router::new().route("/", get(hello));

    let addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    println!("Listening on http://{addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start server");
}

async fn hello() -> impl IntoResponse {
    "hello from da other side"
}
