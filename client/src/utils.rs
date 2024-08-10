use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "server", about = "a randomly spawned server")]
pub struct Opt {
    /// Set the listen address
    #[clap(short = 'a', long = "addr", default_value = "localhost")]
    pub addr: String,

    /// Set the port
    #[clap(short = 'p', long = "port", default_value = "8080")]
    pub port: u16,

    /// set the static dir
    #[clap(long = "static-dir", default_value = "./dist")]
    pub static_dir: String,

    /// define config path
    #[clap(short = 'c', long = "config", default_value = "./config.toml")]
    pub config: String,
}
