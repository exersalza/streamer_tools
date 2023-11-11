use yew::prelude::*;
use yew::{BaseComponent, Component, Context, Html, html, };
use gloo_timers::callback::Interval;
use log::{error, info};
use wasm_bindgen_futures::spawn_local;

use super::utils::{class, get, Data};

struct Time {
    hours: u64,
    minutes: u64,
    seconds: u64
}

pub struct Timer {
    timer: Time,
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
    pub persistent: bool
}


impl Component for Timer {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();

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
            hours: props.hour,
            minutes: props.minute,
            seconds: props.second
        };


        let link = ctx.link().clone();

        Interval::new(1000, move || {
           link.callback(|_| Msg::Tick).emit(());
        }).forget();

        Self {
            timer
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("interval");
        match msg {
            Msg::Tick => {
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
            Msg::Persistent(data) => info!("{data:?}"),
        };

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let external_class = &ctx.props().class;
        let mut classes: Classes = class("mt-6 text-3xl text-text select-none text-left w-12");
        classes.extend(external_class.into_iter());

        html! {
            <p class={classes}>
                {format!("{:02}:{:02}:{:02}", self.timer.hours, self.timer.minutes, self.timer.seconds)}
            </p>
        }
    }
}