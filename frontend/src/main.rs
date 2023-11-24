use log::info;
use yew::prelude::*;
use yew_router::prelude::*;

use components::{timer::Timer, utils::class};

pub mod components;

struct Streamer {
    name: String,
    links: Vec<Html>,
}

struct Base {
    streamer: Streamer,
    paused: bool
}

enum Msg {
    ButtonClick
}

impl Component for Base {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let links = vec![
            ("https://dashboard.twitch.tv/u/{}/home", "dashboard"),
            ("https://twitch.tv/{}", "livestream"),
        ];

        let mut streamer = Streamer {
            name: String::from("betrayedval"),
            links: vec![],
        };

        for (link, name) in links {
            let _t = link.replace("{}", &streamer.name);
            streamer.links.push(html! {
                <a href={_t} target="_blank" class={class("transition-all text-gray-500 hover:text-gray-300")}>{name}</a>
            });
        }

        let paused = false;

        Self { streamer, paused }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ButtonClick => self.paused = !self.paused
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::ButtonClick);

        html! {
            <div>
                // find the center of the screen
                // <div class={class("absolute h-screen w-screen flex justify-center")}>
                //    <div class={class("h-full bg-white w-1")}> </div>
                    // <p class={class("mt-6 text-3xl text-text select-none")}> {"00:17:23"}</p>

                // </div>

                <div class={class("h-screen w-screen bg-base-light grid grid-container")}>
                    // top left
                    <div class={class("bg-base-light grid place-items-center select-none")}>
                        <p class={class("text-3xl text-white uppercase")}>{self.streamer.name.clone()}</p>
                        <div class={class("flex gap-2 items-center w-full justify-center")}>
                            // links to dashboard and the stream
                            {&self.streamer.links}
                        </div>
                        // little bar to close the header section, might be removed in further iterations
                        <div class={class("h-1 w-[70%] bg-gray-600 rounded")}></div>
                    </div>
                    // top right / nav??
                    <div class={class("bg-base-light flex")}>
                        <Timer paused={&self.paused} hour=5 minute=2 class={class("relative left-[33%] top-4")} />
                        <Timer ftype="sub" /> // we do a little bit of trolling here
                    </div>
                    // bottom left / item list
                    <div class={class("bg-base-light")}>

                    </div>
                    // bottom right / body shows first item when created
                    <div class={class("bg-base rounded-tl-xl")}>
                        <button class={class("text-text")} onclick={onclick}>{"Pause timer"}</button>
                    </div>
                </div>
            </div>
        }
    }
}

struct Root {}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,

    #[at("/timer")]
    TimerClean,

    #[at("/timer/:id")]
    Timer { id: i32 },

    #[not_found]
    #[at("/404")] // I hate it when you go outside and someone randomly throws a fridge at you
    NotFound,
}

fn switch(routes: Route) -> Html {
    let paused: Callback<bool> = Callback::from(move |_| {info!("paused")});
    match routes {
        Route::Home => html! {<Base />},
        Route::Timer { id } => html! {<Timer timer_id={id} />},
        Route::TimerClean => html! {<Timer />},
        Route::NotFound => html! {
        <p class={class("bg-base-light grid place-items-center h-screen w-screen text-text")}>{"404 not found"}</p>},
    }
}

impl Component for Root {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        }
    }
}

fn main() {
    let dev = env!("PROD");

    // check if prod is on and if set log level to info
    let log_level: log::Level = match dev.parse::<bool>() {
        Ok(x) => {
            if x {
                log::Level::Info
            } else {
                log::Level::Debug
            }
        }
        Err(_) => log::Level::Debug,
    };

    wasm_logger::init(wasm_logger::Config::new(log_level));
    console_error_panic_hook::set_once();
    yew::Renderer::<Root>::new().render();
}
