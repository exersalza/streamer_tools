use crate::components::nav::Nav;
use yew::prelude::*;
use log::{info, warn, debug};

mod components;
mod utils;

use utils::class;

struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        debug!("created App");
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={class("h-screen w-screen bg-accent text-white")}>
                <p class={class("bg-blue-300 text-green-400")}> {"test"}</p>
                <Nav />
            </div>
        }
    }
}


fn main() {
    let dev = env!("PROD");
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
