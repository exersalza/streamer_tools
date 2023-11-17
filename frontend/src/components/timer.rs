use futures::{SinkExt, StreamExt};
use gloo_net::websocket;
use gloo_net::websocket::futures::WebSocket;
use gloo_timers::callback::Interval;
use log::{debug, error};
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::{html, Component, Context, Html};
use yew_router::prelude::*;

use super::utils::{class, get, query_parser, Data};

struct Time {
    hours: i32,
    minutes: i32,
    seconds: i32,
}

pub struct Timer {
    timer: Time,
    browser: bool,
}

pub enum Msg {
    Tick,
    Persistent(Data),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(0)]
    pub hour: i32,

    #[prop_or(15)]
    pub minute: i32,

    #[prop_or(0)]
    pub second: i32,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or(false)]
    pub persistent: bool,

    #[prop_or(-1)]
    pub timer_id: i32,

    #[prop_or(false)]
    pub browser: bool,
}

impl Component for Timer {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        let location = ctx.link().location();

        let f = query_parser(location.unwrap().query_str());
        let mut query_params: HashMap<String, i32> = HashMap::new();
        let mut browser = props.browser;

        for (key, value) in f {
            if key == "browser" {
                browser = true;
                continue;
            }

            let value = match value.parse::<i32>() {
                Ok(v) => {
                    debug!("{key} {}", (key == "minutes" || key == "seconds"));
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

        if props.persistent {
            let link = ctx.link().clone();

            spawn_local(async move {
                match get("http://localhost:8080/api/subathon_timer").await {
                    Ok(response_data) => link.send_message(Msg::Persistent(Data {
                        data: response_data,
                    })),
                    Err(e) => {
                        error!("{e}")
                    }
                };
            });
        }

        let timer = Time {
            hours: query_params.get("hours").unwrap_or(&props.hour).clone(),
            minutes: query_params.get("minutes").unwrap_or(&props.minute).clone(),
            seconds: query_params
                .get("seconds")
                .unwrap_or(&&props.second)
                .clone(),
        };

        let link = ctx.link().clone();

        // just start an infinite loop, maybe put a tick loop on the server and every timer
        // connects to it via an websocket or something.
        // Interval::new(1000, move || {}).forget();

        let mut ws = WebSocket::open("ws://127.0.0.1:8080/ws").unwrap();
        let (mut tx, mut rx) = ws.split();

        spawn_local(async move {
            while let Some(msg) = rx.next().await {
                log::info!("{msg:?}");
            }
            log::info!("websocket closed");
        });

        Self { timer, browser }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                debug!("time tick");
                if self.timer.seconds > 0 {
                    self.timer.seconds -= 1;
                } else if self.timer.minutes > 0 {
                    self.timer.minutes -= 1;
                    self.timer.seconds = 59;
                } else if self.timer.hours > 0 {
                    self.timer.hours -= 1;
                    self.timer.minutes = 59;
                    self.timer.seconds = 59;
                }
            }
            Msg::Persistent(data) => debug!("{data:?}"),
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
