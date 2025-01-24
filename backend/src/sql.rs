use std::{str::FromStr, sync::Arc};

use lazy_static::lazy_static;
use parking_lot::Mutex;
use serde::Serialize;
use sqlx::{Executor, Sqlite, SqlitePool};
use uuid::Uuid;

lazy_static! {
    pub static ref SQL: Sql = Sql::new();
}

#[derive(Clone)]
pub struct Sql {
    pool: sqlx::Pool<sqlx::Sqlite>,
}

// this should be all valid events that can increase the timer
// maybe add channel points later for the maniacs out there
#[derive(Clone, Serialize, Debug)]
pub struct IncTimes {
    follow: Option<i64>,
    sub_t1: Option<i64>,
    sub_t2: Option<i64>,
    sub_t3: Option<i64>,
    dono_each_n: Option<i64>,
    dono_n: Option<i64>,
    bits_each_n: Option<i64>,
    bits_n: Option<i64>,
}

#[derive(Serialize, Debug)]
pub struct Timer {
    pub id: Uuid,
    pub timer: Option<i64>, // when the timer is supposed to end
    pub name: String,       // name of the timer duh
    pub increase_times: IncTimes,
}

#[allow(clippy::new_without_default)]
impl Sql {
    pub fn new() -> Self {
        let url = crate::config!().db.url.clone();
        let pool = SqlitePool::connect_lazy(&url).unwrap();

        Self { pool }
    }

    pub async fn get_timers(&self) -> anyhow::Result<Vec<Timer>> {
        let res = sqlx::query!("SELECT s.id, s.name, s.time, t.follow, t.sub_t1, t.sub_t2, t.sub_t3, t.dono_each_n, t.dono_n, t.bits_each_n, t.bits_n FROM timer AS s LEFT OUTER JOIN timer_go_down_by AS t ON s.id = t.id;")
            .fetch_all(&self.pool)
            .await?;

        let mut ret = vec![];

        // what a mess, optimize later
        res.iter().for_each(|item| {
            ret.push(Timer {
                id: Uuid::from_str(&item.id).unwrap_or(Uuid::default()),
                name: item.name.clone(),
                timer: Some(item.time.unwrap_or(0)),
                increase_times: IncTimes {
                    follow: item.follow,
                    sub_t1: item.sub_t1,
                    sub_t2: item.sub_t2,
                    sub_t3: item.sub_t3,
                    dono_each_n: item.dono_each_n,
                    dono_n: item.dono_n,
                    bits_each_n: item.bits_each_n,
                    bits_n: item.bits_n,
                },
            });
        });

        Ok(ret)
    }
}
