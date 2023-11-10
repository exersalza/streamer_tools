use crate::components::nav::Nav;
use yew::prelude::*;

mod components;

struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("h-screen", "w-screen", "bg-accent", "text-white", )}>
                <p class={classes!("text-white")}> {"test"}</p>
                <Nav />
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
