use lazy_static::lazy_static;
use log::{debug, error};
use sqlite::State;
use std::sync::Mutex;

use crate::config::Config;

lazy_static! {
    pub static ref CONFIG: Mutex<Config> = Mutex::new(Config::new("./config.toml"));
}

pub struct Sql {
    conn: sqlite::Connection,
}

#[derive(Debug)]
pub struct Timer {
    pub id: i32,
    pub time: String,
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
    pub fn create_timer(&self, timer: &Timer) -> i32 {
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
        let query = format!("delete from timers where timer_id = {timer_id};");
        debug!("{query}");

        match self.conn.execute(query) {
            Ok(t) => t,
            Err(e) => error!("failed to delete {}, {e}", timer_id),
        };

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
