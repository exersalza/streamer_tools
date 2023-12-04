use std::collections::HashMap;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::class;

struct TimerItem {
    id: i64,
    states: HashMap<String, bool>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(0)]
    pub id: i64,

    #[prop_or("".to_string())]
    pub title: String

}

impl Component for TimerItem {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let states: HashMap<String, bool> = HashMap::new();

        Self { id, states }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={class("bg-yellow-500 h-16 w-full rounded")}>
                <p class={class("m-2")}>{&self.id}</p>
            </div>
        }
    }
}