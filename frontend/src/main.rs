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
                <Nav />
            </div>
        }
    }
}


fn main() {
    let dev = env!("PROD");

    // check if prod is on and if set log level to info
    let log_level: log::Level = match dev.parse::<bool>() {
        Ok(x) => {if x {
            log::Level::Info
        } else {
            log::Level::Debug
        }},
        Err(_) => log::Level::Debug,
    };

    wasm_logger::init(wasm_logger::Config::new(log_level));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
