use std::fs;
use std::process::exit;

use serde_derive::Deserialize;
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub twitch: Twitch,
    pub sql_path: String,
}

#[derive(Deserialize)]
pub struct Twitch {
    pub token: String,
    pub channel: String,
}

fn open_config(path: &str) -> Config {
    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Couldn't open config file due to {e}");
            exit(1)
        }
    };

    match toml::from_str(&contents) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{e}");
            eprintln!("Unable to load data from {}", path);
            exit(1)
        }
    }
}

impl Config {
    pub fn new(path: &str) -> Self {
        let data = open_config(&path);
        let _config = Config {
            twitch: Twitch {
                token: data.twitch.token,
                channel: data.twitch.channel,
            },
            sql_path: data.sql_path,
        };

        return _config;
    }
}
