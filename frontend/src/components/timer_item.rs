use std::collections::HashMap;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{utils::class, icons::Icons};


pub struct TimerItem {
    id: i64,
    title: String,
    states: HashMap<String, bool>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(0)]
    pub id: i64,

    #[prop_or("".to_string())]
    pub title: String,
}

impl Component for TimerItem {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        let id: i64 = props.id;
        let title: String = props.title.clone();
        let states: HashMap<String, bool> = HashMap::new();

        Self { id, title, states }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={class("hover:bg-accent h-max w-full text-gray-500 hover:text-gray-300 select-none transition-all rounded -ml-2 flex flex-row content-center gap-2")}>
                <Icons icon={"clock".to_string()} fill={"#000".to_string()} style={class("ml-4").to_string()} />
                <p class={class("ml-2 w-16 h-max text-xl")}>{&self.id}</p>
                <p class={class("ml-4 h-max text-xl")}>{&self.title}</p>
            </div>
        }
    }
}