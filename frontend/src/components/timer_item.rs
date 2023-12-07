use std::collections::HashMap;
use log::info;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{utils::class, icons::Icons};


pub struct TimerItem {
    id: i64,
    title: String,
    active: bool,
    states: HashMap<String, bool>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(0)]
    pub id: i64,

    #[prop_or("".to_string())]
    pub title: String,

    #[prop_or(false)]
    pub active: bool,
}

pub enum Msg {
    FlipActive
}

impl Component for TimerItem {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        let id: i64 = props.id;
        let title: String = props.title.clone();
        let states: HashMap<String, bool> = HashMap::new();
        let active = props.active;

        Self { id, title, states, active }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FlipActive => {
                self.active = !self.active;
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();

        let active: String = if self.active {
            "bg-accent".to_string()
        } else {
            String::new()
        };

        let onclick = Callback::from(move |_| {
            link.send_message(Msg::FlipActive);
        });

        html! {
            <div class={class(format!("hover:bg-accent h-max w-full text-gray-500 hover:text-gray-300 select-none transition-all rounded -ml-2 flex flex-row content-center gap-2 {active}").as_str())}
                    {onclick}
                    id={self.id}>
                <Icons icon={"clock".to_string()} fill={"#000".to_string()} style={class("ml-4").to_string()} />
                <p class={class("w-16 h-max text-xl")}>{&self.id}</p>
                <p class={class("ml-4 h-max text-xl")}>{&self.title}</p>
            </div>
        }
    }
}