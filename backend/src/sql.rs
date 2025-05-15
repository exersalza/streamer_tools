use std::str::FromStr;

use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use sqlx::{query, Executor, SqlitePool};
use uuid::Uuid;

use crate::{
    routes::{running_timer, AuthTokenResponseOk, HistoryItem, Settings, TimerCustom},
    twitch::User,
};

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
}

#[derive(Serialize, Debug, Deserialize)]
pub struct StrippedTimer {
    pub id: Uuid,
    pub timer: Option<i64>, // when the timer is supposed to end
    pub name: String,       // name of the timer duh
    pub main: bool,
    pub is_active: bool,
    pub color: String,
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

    pub async fn get_timer(&self, id: String) -> Result<Vec<Timer>> {
        let res = sqlx::query!("SELECT s.id, s.name, s.time, s.main, s.is_active, s.color, t.follow, t.sub_t1, t.sub_t2, t.sub_t3, t.dono_each_n, t.dono_n, t.bits_each_n, t.bits_n FROM timer AS s LEFT OUTER JOIN timer_go_down_by AS t ON s.id = t.id where t.id = ?", id)
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
                color: item.color.clone().unwrap_or("#000".into()),
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

    pub async fn get_all_timer(&self) -> Result<Vec<StrippedTimer>> {
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

    pub async fn post_create_timer(&self, payload: Timer) -> Result<()> {
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
    pub async fn post_update_timer(&self, payload: Timer) -> Result<()> {
        let id = payload.id.to_string();

        let _ = sqlx::query!(
            r#"update timer set name = ?, time = ?, main = ? where id = ?"#,
            payload.name,
            payload.timer,
            payload.main,
            id
        )
        .execute(&self.pool)
        .await?;

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

    pub async fn get_timer_ids(&self) -> Result<Vec<String>> {
        let res = sqlx::query!("select id from timer")
            .fetch_all(&self.pool)
            .await?;
        let ret = res
            .iter()
            .map(|value| value.id.clone())
            .collect::<Vec<String>>();

        Ok(ret)
    }

    pub async fn get_user_token_exist(&self) -> Result<bool> {
        Ok(self.get_twitch_user_token().await?.is_some())
    }

    pub async fn get_twitch_user_token(&self) -> Result<Option<String>> {
        let token = sqlx::query!("select user_token from twitch_data where id = 1;")
            .fetch_one(&self.pool)
            .await?;

        Ok(token.user_token)
    }

    pub async fn insert_twitch_token(&self, token: String, refresh: String) -> Result<()> {
        sqlx::query!(
            "INSERT INTO twitch_data (id, user_token, user_refresh)
VALUES (?, ?, ?)
ON CONFLICT (id)
DO UPDATE SET user_token = ?, user_refresh = ?;",
            1,
            token,
            refresh,
            token,
            refresh
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_user_access_token(&self, res: AuthTokenResponseOk) -> Result<()> {
        sqlx::query!(
            "INSERT INTO twitch_data (id, user_token, user_refresh, token_type, expires_in)
             VALUES (1, ?, ?, ?, ?)
             ON CONFLICT (id) DO UPDATE
             SET user_token = EXCLUDED.user_token,
                 user_refresh = EXCLUDED.user_refresh,
                 token_type = EXCLUDED.token_type,
                 expires_in = EXCLUDED.expires_in",
            res.access_token,
            res.refresh_token,
            res.token_type,
            res.expires_in
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn remove_token_data(&self) -> Result<u64> {
        Ok(sqlx::query!("delete from twitch_data where id=1")
            .execute(&self.pool)
            .await?
            .rows_affected())
    }

    pub async fn get_refresh_token(&self) -> Result<String> {
        let ret = sqlx::query!("select user_refresh from twitch_data where id = 1")
            .fetch_one(&self.pool)
            .await?;

        Ok(ret.user_refresh.unwrap_or("".to_string()))
    }

    pub async fn get_expires_in_oauth(&self) -> Result<Option<i64>> {
        let ret = sqlx::query!("select expires_in from _oauth where id=1")
            .fetch_one(&self.pool)
            .await?;
        Ok(ret.expires_in)
    }

    /// Gets the OAuth for the bot side related stuff
    ///
    /// # Returns
    /// (token, token_type) -> the token and the type
    pub async fn get_bot_oauth(&self) -> Result<(String, String)> {
        let ret = sqlx::query!("select token, token_type from _oauth where id=1")
            .fetch_one(&self.pool)
            .await?;

        Ok((ret.token.unwrap(), ret.token_type.unwrap()))
    }

    pub async fn update_oauth_data(
        &self,
        access_token: String,
        expires_in: DateTime<Utc>,
        token_type: String,
    ) -> Result<()> {
        let time = expires_in.timestamp();

        sqlx::query!(
            "
INSERT INTO _oauth (id, token, expires_in, token_type)
VALUES (1, ?, ?, ?)
ON CONFLICT (id)
DO UPDATE SET token = ?, expires_in = ?, token_type = ?;
",
            access_token,
            time,
            token_type,
            access_token,
            time,
            token_type
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_user(&self, user: &User) -> Result<()> {
        let f = self.get_user().await?;
        if f.is_some() {
            dbg!(&f, &user);
            match sqlx::query!("update user_data set username = ?, display_name = ?, profile_pic = ?, broadcaster_type = ?, id = ?", 
                user.login,
                user.display_name,
                user.profile_image_url,
                user.broadcaster_type,
                user.id).execute(&self.pool).await {
                Ok(v) => println!("{}", v.rows_affected()),
                Err(e) => println!("{}", e),
            }

            return Ok(());
        }

        sqlx::query!("insert into user_data (id, username, display_name, profile_pic, broadcaster_type) values (?, ?, ?, ?, ?)", user.id,
            user.login,
            user.display_name,
            user.profile_image_url,
            user.broadcaster_type).execute(&self.pool).await?;

        Ok(())
    }

    pub async fn update_username(&self, username: String) -> Result<()> {
        let f = self.get_user().await?;

        if f.is_some() {
            // :tf:
            // this will surely be ok Clueless
            sqlx::query!("update user_data set username = ?", username)
                .execute(&self.pool)
                .await?;
            return Ok(());
        }

        sqlx::query!("insert into user_data (username) values (?)", username)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// get the user from the db
    pub async fn get_user(&self) -> Result<Option<User>> {
        // we just hope that there is only one
        let f = sqlx::query!(
            "select id, username, display_name, profile_pic, broadcaster_type from user_data"
        )
        .fetch_all(&self.pool)
        .await?;

        if let Some(user) = f.last() {
            return Ok(Some(User {
                // godspeed 47
                id: user.id.clone().unwrap_or_default().to_string(),
                login: user.username.clone().unwrap_or_default(),
                display_name: user.display_name.clone().unwrap_or_default(),
                broadcaster_type: user.broadcaster_type.clone().unwrap_or_default(),
                profile_image_url: user.profile_pic.clone().unwrap_or_default(),
            }));
        }
        Ok(None)
    }

    pub async fn dec_timer(&self, id: String) -> Result<u64> {
        Ok(
            sqlx::query!("update timer set time = time - 1 where id = ?", id)
                .execute(&self.pool)
                .await?
                .rows_affected(),
        )
    }

    pub async fn dec_all_timer(&self) -> Result<()> {
        // types
        // 0 -> normal decrementing timer
        // 1 -> incrementing timer

        #[allow(non_snake_case)]
        let mut special_query_thingy_idk_how_to_caLL_this = vec![];

        {
            let lock = running_timer.lock();
            let mut ret = vec![];

            for (key, value) in lock.clone().into_iter() {
                if value <= 0 {
                    continue;
                }

                ret.push(key);
            }

            for i in ret {
                special_query_thingy_idk_how_to_caLL_this.push(i);
            }
        }

        let query = format!(
            "update timer set time = time - 1 where is_active = 1 and type = 0 and id in ({})",
            special_query_thingy_idk_how_to_caLL_this
                .clone()
                .iter()
                .map(|_| "?")
                .collect::<Vec<_>>()
                .join(",")
        );

        let mut thingy = sqlx::query(&query);

        for i in &special_query_thingy_idk_how_to_caLL_this {
            thingy = thingy.bind(i);
        }

        thingy.execute(&self.pool).await?;

        Ok(())
    }

    pub async fn toggle_timer_active(&self, id: String) -> Result<()> {
        sqlx::query!("update timer set is_active = ~is_active where id = ?", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn set_timer_active(&self, id: String, active: bool) -> Result<()> {
        sqlx::query!("update timer set is_active = ? where id = ?", active, id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_active_timer(&self) -> Result<Vec<(String, String)>> {
        let mut ret = vec![];
        for row in sqlx::query!("select id, time from timer where is_active = 1")
            .fetch_all(&self.pool)
            .await?
        {
            ret.push((row.id, row.time.unwrap_or(0).to_string()))
        }

        Ok(ret)
    }

    pub async fn add_time_to_timer(&self, id: String, time_to_add: i32) -> Result<()> {
        sqlx::query!(
            "update timer set time = max(time + ?, 0) where id = ? ",
            time_to_add,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn set_timer(&self, id: String, time: i32) -> Result<()> {
        sqlx::query!("update timer set time = max(?, 0) where id = ? ", time, id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // SETTINGS
    pub async fn update_setting(&self, payload: Settings) -> Result<()> {
        sqlx::query!(
            "INSERT INTO settings (id, show_emotes)
             VALUES (1, ?)
             ON CONFLICT (id) DO UPDATE
             SET show_emotes = EXCLUDED.show_emotes;",
            payload.show_emotes
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_settings(&self) -> Result<Settings> {
        let res = sqlx::query!("select show_emotes from settings where id = 1")
            .fetch_one(&self.pool)
            .await?;

        Ok(Settings {
            show_emotes: res.show_emotes.unwrap_or(0) == 1,
        })
    }

    pub async fn get_timer_hist(&self, uuid: String) -> Result<Vec<HistoryItem>> {
        let q = sqlx::query!(
            "select uuid, event_type, amount, user, extra, timestamp, time_added from history where uuid = ? order by timestamp", uuid
        ).fetch_all(&self.pool).await?;

        let mut ret = vec![];

        // TODO: refactor into one liner, its just a POC for now
        for i in q {
            ret.push(HistoryItem {
                uuid: i.uuid.unwrap_or_default(),
                event_type: i.event_type.unwrap_or("subscription".to_string()),
                amount: i.amount.unwrap_or_default(),
                user: i.user.unwrap_or("Anonymous".to_string()),
                extra: i.extra.unwrap_or_default(),
                timestamp: i.timestamp.unwrap_or_default(),
                time_added: i.time_added.unwrap_or_default(),
            });
        }

        Ok(ret)
    }

    // TIMER CUSTOM THIGNS
    pub async fn get_timer_custom_data(&self, id: String) -> Result<TimerCustom> {
        todo!()
    }
}
