use anyhow::Result;
use lazy_static::lazy_static;
use notify::Watcher;
use parking_lot::Mutex;
use serde_derive::Deserialize;
use tokio::time::{self, Duration, Instant};
use toml;

use std::{fs, sync::Arc, thread};

/// AM<T>> short cut
pub type AM<T> = Arc<Mutex<T>>;
/// Thread fn pointer
pub type TFnPtr = Box<dyn Fn() + Send + Sync>;

lazy_static! {
    pub static ref conf: AM<Config> = Arc::new(Mutex::new(Config::new("./config.toml")));
    static ref running_threads: AM<Vec<String>> = Arc::new(Mutex::new(vec![]));
    static ref last_event: AM<Instant> = Arc::new(Mutex::new(Instant::now()));
    static ref callbacks: AM<Vec<TFnPtr>> = Arc::new(Mutex::new(vec![]));
}

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub twitch: Twitch,
    pub db: Db,
    path: String,
}

#[derive(Deserialize, Debug)]
pub struct InnerConfig {
    pub twitch: Twitch,
    pub db: Db,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Twitch {
    pub username: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Db {
    pub url: String,
}

/// open the config file and return the contents of it parsed to the struct
fn open_config(path: &str) -> InnerConfig {
    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            panic!("Couldn't open config file due to {e}");
        }
    };

    match toml::from_str(&contents) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{e}");
            panic!("Unable to load data from {}", path);
        }
    }
}

impl Config {
    pub fn new(path: &str) -> Self {
        let data = open_config(path);

        Config {
            twitch: data.twitch,
            db: data.db,
            path: path.to_string(),
        }
    }

    /// Starts the file watchdog needed for hot reloading config files
    pub fn start_watchdog(config: AM<Self>) {
        tokio::spawn(Self::watchdog_thread(config));
    }

    /// Add callbacks that trigger when the config updates
    pub fn add_callback(callback: TFnPtr) {
        let ptr = callbacks.clone();
        let mut lock = ptr.lock();

        lock.push(callback);
    }

    /// manually reload the config
    pub fn config_reload() {
        let mut config_lock = conf.lock();
        let data = open_config(&config_lock.path);

        config_lock.twitch = data.twitch;
        config_lock.db = data.db;
    }

    /// Event handler for the file watcher
    fn do_stuff(event: notify::Event) {
        if let notify::EventKind::Modify(_) = event.kind {
            let mut time_lock = last_event.lock();
            let now = Instant::now();

            // just try to catch the first event and then return everytime
            if now.duration_since(*time_lock).as_millis() >= 25 {
                // introduce small delay to cope with the many different file saving things
                thread::sleep(Duration::from_millis(50));
                Self::config_reload();

                {
                    // call all the callbacks
                    let cb_lock = callbacks.lock();
                    for i in cb_lock.iter() {
                        i();
                    }
                }
            }

            *time_lock = Instant::now();
        }
    }

    /// Start watchdog
    async fn watchdog_thread(config: AM<Self>) -> Result<()> {
        let mut interval = time::interval(Duration::from_secs(1));
        let path = config.clone().lock().path.clone();

        // trigger the first event, basically runs the function inside the lazy_static
        drop(last_event.lock());
        let mut watcher = notify::recommended_watcher(move |r| match r {
            Ok(event) => Self::do_stuff(event),
            Err(_) => todo!(),
        })?;

        watcher.watch(
            std::path::Path::new(&path),
            notify::RecursiveMode::NonRecursive,
        )?;

        // keep the thread alive
        loop {
            interval.tick().await;
        }
    }

    async fn save_to_file(self) -> anyhow::Result<()> {
        let handle = tokio::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(self.path)
            .await?;

        Ok(())
    }
}
