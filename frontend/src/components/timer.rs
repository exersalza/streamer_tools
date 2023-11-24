use std::collections::HashMap;
use std::num::ParseIntError;

use futures::StreamExt;
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use log::{debug, error, info};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::{html, Component, Context, Html};
use yew_router::prelude::*;
use serde::{Deserialize, Serialize};

use super::utils::{class, query_parser, Data};
extern crate shared;

const SUB_TIMER: i32 = 6969;

pub enum Msg {
    Tick(String),
    Persistent(Data),
    Update(i32)
}

#[derive(Serialize, Deserialize)]
pub struct Time {
    pub hours: i32,
    pub minutes: i32,
    pub seconds: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Timer {
    pub timer: Time,
    pub id: i32,
    pub browser: bool,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(0)]
    pub hour: i32,

    #[prop_or(15)]
    pub minute: i32,

    #[prop_or(0)]
    pub second: i32,

    #[prop_or(0)]
    pub delta: i32,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or(0)]
    pub timer_id: i32,

    #[prop_or(false)]
    pub browser: bool,

    #[prop_or("dec".to_string())]
    pub ftype: String,

    #[prop_or(false)]
    pub paused: bool,
}

impl Time {
    pub fn new(hours: i32, minutes: i32, seconds: i32) -> Self {
        Self {
            hours,
            minutes,
            seconds,
        }
    }

    /// Construct from seconds
    pub fn from(sec: i32) -> Self {
        let (hours, minutes, seconds) = Time::convert_seconds(sec);

        Self {
            hours,
            minutes,
            seconds,
        }
    }

    /// Like the from the difference is just that you dont get an instance, you just get the values
    /// hours -> minutes -> seconds
    pub fn convert_seconds(sec: i32) -> (i32, i32, i32) {
        let seconds = sec % 60;
        let minutes = (sec / 60) % 60;
        let hours = (sec / 60) / 60;

        (hours, minutes, seconds)
    }

    /// Convert the Time structs elements to seconds
    pub fn to_seconds(&self) -> i32 {
        // conversion from above but reversed and minified
        (self.hours * (60 * 60)) + (self.minutes * 60) + self.seconds
    }

    /// like to_seconds just that you dont have to have instance of the struct to use it
    pub fn convert_time(hours: i32, minutes: i32, seconds: i32) -> i32 {
        (hours * (60 * 60)) + (minutes * 60) + seconds
    }

    pub fn add_seconds(&mut self, sec: i32) {
        let pre_sec = self.to_seconds();
        let ret = Self::from(&pre_sec + &sec);

        self.hours = ret.hours;
        self.minutes = ret.minutes;
        self.seconds = ret.seconds;
    }

    pub fn update(&mut self, sec: i32) {
        let (hours, minutes, seconds) = Self::convert_seconds(sec);
        info!("{sec}");

        self.hours = hours;
        self.minutes = minutes;
        self.seconds = seconds;
    }


    // From time in this case is ex. XX:XX:XX
    pub fn from_time(timer: String) -> Result<Self, ParseIntError> {
        let items: Vec<_> = timer.split(':').collect();

        let hours: i32 = items[0].parse::<i32>()?;
        let minutes: i32 = items[1].parse::<i32>()?;
        let seconds: i32 = items[2].parse::<i32>()?;

        Ok(Self::new(hours, minutes, seconds))
    }

    pub fn inc_by_one(&mut self) {
        self.seconds += 1;

        if self.seconds == 60 {
            self.seconds = 0;
            self.minutes += 1;

            if self.minutes == 60 {
                self.minutes = 0;
                self.hours += 1;
            }
        }
    }

    pub fn dec_by_one(&mut self) {
        if self.seconds > 0 {
            self.seconds -= 1;
        } else if self.minutes > 0 {
            self.minutes -= 1;
            self.seconds = 59;
        } else if self.hours > 0 {
            self.hours -= 1;
            self.minutes = 59;
            self.seconds = 59;
        }
    }
}


impl Timer {
    pub fn new() -> Timer {
        Self {
            id: 0,
            timer: Time::from(0),
            browser: false,
        }
    }

    pub fn convert_and_insert(&mut self, id: i32, hours: i32, minutes: i32, seconds: i32) {
        self.id = id;
        self.timer = Time::new(hours, minutes, seconds);
    }
}

// COMPONENT SECTION
impl Component for Timer {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        let location = ctx.link().location();

        let f = query_parser(location.unwrap().query_str());
        let mut query_params: HashMap<String, i32> = HashMap::new();
        let mut browser = props.browser;
        let mut id = props.timer_id;

        // parse ?=arguments
        for (key, value) in f {
            if key == "browser" {
                browser = value == "true";
                continue;
            }

            let value = match value.parse::<i32>() {
                Ok(v) => {
                    if (key == "minutes" || key == "seconds") && (v > 60 && v < 0) {
                        59
                    } else {
                        v
                    }
                }
                Err(e) => {
                    error!("error while trying to parse int({key}->{value}) -> {e:?}");
                    59
                }
            };

            query_params.insert(key, value);
        }

        let timer: Time = if query_params.get("delta").is_some() || props.delta != 0 {
            let delta = query_params.get("delta").unwrap_or(&props.delta);
            Time::from(*delta)
        } else {
            Time {
                hours: query_params.get("hours").unwrap_or(&props.hour).clone(),
                minutes: query_params.get("minutes").unwrap_or(&props.minute).clone(),
                seconds: query_params.get("seconds").unwrap_or(&props.second).clone(),
            }
        };

        let link = ctx.link().clone();

        // just start an infinite loop, maybe put a tick loop on the server and every timer
        // connects to it via an websocket or something.
        // Interval::new(1000, move || {}).forget();

        let ftype = if id == SUB_TIMER {
            "sub".to_string()
        } else {
            props.ftype.clone()
        };

        let mut ws = WebSocket::open(&*format!("ws://127.0.0.1:8080/ws/{}", ftype)).unwrap();
        let (mut tx, mut rx) = ws.split();

        spawn_local(async move {
            while let Some(msg) = rx.next().await {
                let t: String = match msg {
                    Ok(t) => {
                        match t {
                            Message::Text(msg) => msg,
                            _ => "00".to_string(), // I have no clue why I put 00 here
                        }
                    }
                    Err(_) => "f".to_string(), // f
                };

                link.send_message(Msg::Tick(t.to_string()));
            }
            debug!("websocket closed");
        });

        Self { timer, browser, id }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();

        match msg {
            Msg::Tick(msg) => {
                let mut t = props.ftype.clone();

                if props.paused {
                    return false;
                }

                if props.timer_id == SUB_TIMER {
                    t = String::from("sub");
                }

                match t.as_str() {
                    "sub" => {
                        self.timer.update(msg.parse::<i32>().unwrap());
                    },
                    "inc" => {
                        self.timer.inc_by_one();
                    },
                    _ => { // default decrement
                        self.timer.dec_by_one();
                    }
                }
            }
            Msg::Persistent(data) => info!("{data:?}"),
            Msg::Update(time) => self.timer = Time::from(time),
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let external_class = &ctx.props().class;
        let mut classes: Classes = class("text-3xl text-text select-none text-left w-12");

        classes.extend(external_class.into_iter());
        let time = format!(
            "{:02}:{:02}:{:02}",
            self.timer.hours, self.timer.minutes, self.timer.seconds
        );

        html! {
            if self.browser {
                <div class={class("bg-black w-screen h-screen")}>
                    <p class={classes}> {time}</p>
                </div>
            } else {
                <p class={classes}> {time}</p>
            }
        }
    }
}
