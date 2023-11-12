use std::collections::HashMap;
use yew::prelude::*;
use yew::{BaseComponent, Component, Context, Html, html, };
use gloo_timers::callback::Interval;
use log::{debug, error, info};
use wasm_bindgen_futures::spawn_local;
use yew_router::prelude::*;

use super::utils::{class, get, Data, query_parser};

struct Time {
    hours: u64,
    minutes: u64,
    seconds: u64
}

pub struct Timer {
    timer: Time,
    browser: bool
}

pub enum Msg {
    Tick,
    Persistent(Data)
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(8)]
    pub hour: u64,

    #[prop_or(0)]
    pub minute: u64,

    #[prop_or(0)]
    pub second: u64,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or(false)]
    pub persistent: bool,

    #[prop_or(0)]
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
        let mut query_params: HashMap<String, u64> = HashMap::new();

        for (key, value) in f {
            let value = match value.parse::<u64>() {
                Ok(v) => {
                    debug!("{key} {}", (key == "minutes" || key == "seconds"));
                    if (key == "minutes" || key == "seconds") && (v > 60 && v < 0) {
                        59
                    } else {
                        v
                    }
                },
                Err(e) => {error!("error while trying to parse int({key}->{value}) -> {e:?}"); 59},
            };

            query_params.insert(key, value);
        }

        if props.persistent {
            let link = ctx.link().clone();

            spawn_local(async move {
                match get("http://localhost:8080/api/subathon_timer/-1").await {
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
            seconds: query_params.get("seconds").unwrap_or(&&props.second).clone()
        };

        let browser = props.browser;


        let link = ctx.link().clone();

        Interval::new(1000, move || {
           link.callback(|_| Msg::Tick).emit(());
        }).forget();

        Self {
            timer,
            browser
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                debug!("time tick");
                if self.timer.seconds > 0 {
                    self.timer.seconds -= 1;
                }
                else if self.timer.minutes > 0 {
                    self.timer.minutes -= 1;
                    self.timer.seconds = 59;
                }
                else if self.timer.hours > 0 {
                    self.timer.hours -= 1;
                    self.timer.minutes = 59;
                    self.timer.seconds = 59;
                }
            },
            Msg::Persistent(data) => debug!("{data:?}"),
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let external_class = &ctx.props().class;
        let mut classes: Classes = class("text-3xl text-text select-none text-left w-12");

        classes.extend(external_class.into_iter());
        let some_condition = true;
        let time = format!("{:02}:{:02}:{:02}", self.timer.hours, self.timer.minutes, self.timer.seconds);

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