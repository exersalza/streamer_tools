use std::{str::FromStr, sync::Arc};

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Sqlite, SqlitePool};
use uuid::Uuid;

use crate::routes::AuthTokenResponseOk;

lazy_static! {
    pub static ref SQL: Sql = Sql::new();
}

#[derive(Clone)]
pub struct Sql {
    pool: sqlx::Pool<sqlx::Sqlite>,
}

// this should be all valid events that can increase the timer
// maybe add channel points later for the maniacs out there
#[derive(Clone, Serialize, Debug, Deserialize)]
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

#[derive(Serialize, Debug, Deserialize)]
pub struct Timer {
    pub id: Uuid,
    pub timer: Option<i64>, // when the timer is supposed to end
    pub name: String,       // name of the timer duh
    pub main: bool,
    pub increase_times: IncTimes,
    pub is_active: bool,
    pub color: String,
    pub overtitle: String,
    pub undertitle: String
}

#[derive(Serialize, Debug, Deserialize)]
pub struct StrippedTimer {
    pub id: Uuid,
    pub timer: Option<i64>, // when the timer is supposed to end
    pub name: String,       // name of the timer duh
    pub main: bool,
    pub is_active: bool,
    pub color: String
}


// TODO:
//  - Add implementation for recovery/first start of the prgram. eg. add migration scripts 
#[allow(clippy::new_without_default)]
impl Sql {
    pub fn new() -> Self {
        let url = crate::config!().db.url.clone();
        let pool = SqlitePool::connect_lazy(&url).unwrap();

        Self { pool }
    }

    pub async fn get_timer(&self, id: String) -> anyhow::Result<Vec<Timer>> {
        let res = sqlx::query!("SELECT s.id, s.name, s.time, s.main,s.overtitle, s.undertitle, s.is_active, s.color, t.follow, t.sub_t1, t.sub_t2, t.sub_t3, t.dono_each_n, t.dono_n, t.bits_each_n, t.bits_n FROM timer AS s LEFT OUTER JOIN timer_go_down_by AS t ON s.id = t.id where t.id = ?", id)
            .fetch_all(&self.pool)
            .await?;

        let mut ret = vec![];

        // what a mess, optimize later, frick this function
        res.iter().for_each(|item| {
            ret.push(Timer {
                id: Uuid::from_str(&item.id).unwrap_or(Uuid::default()),
                name: item.name.clone(),
                timer: Some(item.time.unwrap_or(0)),
                main: item.main.unwrap_or(0) == 1,
                is_active: item.is_active.unwrap_or(0) == 1,
                color: item.color.clone().unwrap_or("#000000".into()),
                overtitle: item.overtitle.clone().unwrap_or("".into()),
                undertitle: item.undertitle.clone().unwrap_or("".into()),
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

    pub async fn get_all_timer(&self) -> anyhow::Result<Vec<StrippedTimer>> {

        let res = sqlx::query!("SELECT id, name, time, main, is_active, color FROM timer")
            .fetch_all(&self.pool)
            .await?;

        let mut ret = vec![];

        // what a mess, optimize later
        res.iter().for_each(|item| {
            ret.push(StrippedTimer {
                id: Uuid::from_str(&item.id).unwrap_or(Uuid::default()),
                name: item.name.clone(),
                timer: Some(item.time.unwrap_or(0)),
                main: item.main.unwrap_or(0) == 1,
                is_active: item.is_active.unwrap_or(0) == 1,
                color: item.color.clone().unwrap_or("#000000".into()),

            });
        });

        Ok(ret)
    }

    pub async fn post_create_timer(&self, payload: Timer) -> anyhow::Result<()> {
        let id = payload.id.to_string();

        let _ = sqlx::query!(
            r#"INSERT INTO timer (id, name, time, main, is_active, color) values (?, ?, ?, ?, ?, ?)"#,
            id,
            payload.name,
            payload.timer,
            payload.main,
            payload.is_active,
            payload.color
        ).execute(&self.pool).await?;


        // yanky ass queries
        let _ = sqlx::query!("INSERT INTO timer_go_down_by (id, follow, sub_t1, sub_t2, sub_t3, dono_each_n, dono_n, bits_each_n, bits_n) values (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            id, 
            payload.increase_times.follow, 
            payload.increase_times.sub_t1,
            payload.increase_times.sub_t2,
            payload.increase_times.sub_t3,
            payload.increase_times.dono_each_n,
            payload.increase_times.dono_n,
            payload.increase_times.bits_each_n,
            payload.increase_times.bits_n
        ).execute(&self.pool).await?;
        Ok(())
    }

    // janky ass function, gotta code something for this
    pub async fn post_update_timer(&self, payload: Timer) -> anyhow::Result<()> {
        let id = payload.id.to_string();

        let _ = sqlx::query!(
            r#"update timer set name = ?, time = ?, main = ?, overtitle = ?, undertitle = ? where id = ?"#,
            payload.name,
            payload.timer,
            payload.main,
            payload.overtitle,
            payload.undertitle,
            id
        ).execute(&self.pool).await?;

        let _ = sqlx::query!("UPDATE timer_go_down_by set follow = ?, sub_t1 = ?, sub_t2 = ?, sub_t3 = ?, dono_each_n = ?, dono_n = ?, bits_each_n = ?, bits_n = ? where id = ?",
            payload.increase_times.follow, 
            payload.increase_times.sub_t1,
            payload.increase_times.sub_t2,
            payload.increase_times.sub_t3,
            payload.increase_times.dono_each_n,
            payload.increase_times.dono_n,
            payload.increase_times.bits_each_n,
            payload.increase_times.bits_n,
            id
        ).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn get_timer_ids(&self) -> anyhow::Result<Vec<String>> {
        let res = sqlx::query!("select id from timer").fetch_all(&self.pool).await?;
        let ret = res.iter().map(|value| value.id.clone()).collect::<Vec<String>>();

        Ok(ret)
    }

    pub async fn get_user_token_exist(&self) -> anyhow::Result<bool> {
        Ok(self.get_twitch_user_token().await?.is_some())
    }

    pub async fn get_twitch_user_token(&self) -> anyhow::Result<Option<String>> {
        let token = sqlx::query!("select user_token from twitch_data where id = 1;").fetch_one(&self.pool).await?;

        Ok(token.user_token)
    }

    pub async fn insert_twitch_token(&self, token: String, refresh: String) -> anyhow::Result<()> {
        sqlx::query!("INSERT INTO twitch_data (id, user_token, user_refresh)
VALUES (?, ?, ?)
ON CONFLICT (id)
DO UPDATE SET user_token = ?, user_refresh = ?;",1, token, refresh, token, refresh).execute(&self.pool).await?;

        Ok(())
    }

    pub async fn update_user_access_token(&self, res: AuthTokenResponseOk) -> anyhow::Result<()> {
        
        Ok(())
    }

    pub async fn get_refresh_token(&self) -> anyhow::Result<String> {
        let ret = sqlx::query!("select user_refresh from twitch_data where id = 1").fetch_one(&self.pool).await?;

        Ok(ret.user_refresh.unwrap_or("".to_string()))
    }

    /// Gets the OAuth for the bot side related stuff
    ///
    /// # Returns
    /// (token, token_type) -> the token and the type
    pub async fn get_bot_oauth(&self) -> anyhow::Result<(String, String)> {
        let ret = sqlx::query!("select token, token_type from _oauth where id=1").fetch_one(&self.pool).await?;

       Ok((ret.token.unwrap(), ret.token_type.unwrap()))
    }

    pub async fn update_oauth_data(&self, access_token: String, expires_in: DateTime<Utc>, token_type: String) -> anyhow::Result<()> {
        let time = expires_in.timestamp();

        sqlx::query!("
INSERT INTO _oauth (id, token, expires_in, token_type)
VALUES (1, ?, ?, ?)
ON CONFLICT (id)
DO UPDATE SET token = ?, expires_in = ?, token_type = ?;
", access_token, time, token_type, access_token, time, token_type).execute(&self.pool).await?; 
        Ok(())
    }
}
