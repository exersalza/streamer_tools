use yew::prelude::*;

mod components;
use components::utils::class;

struct Streamer {
    name: String,
    links: Vec<Html>
}

struct App {
    streamer: Streamer,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let links = vec![
            ("https://dashboard.twitch.tv/u/{}/home", "dashboard"),
            ("https://twitch.tv/{}", "livestream")
        ];

        let mut streamer = Streamer {
            name: String::from("betrayed"),
            links: vec![]
        };

        for (link, name) in links {
            let _t = link.replace("{}", &streamer.name);
            streamer.links.push(html! {
                <a href={_t} target="_blank" class={class("transition-all text-gray-500 hover:text-gray-300")}>{name}</a>
            });
        }

        Self {streamer}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={class("h-screen w-screen bg-base-light grid grid-container")}>
                // top left
                <div class={class("bg-base-light grid place-items-center")}>
                    <p class={class("text-3xl text-white uppercase")}>{self.streamer.name.clone()}</p>
                    <div class={class("flex gap-2 items-center w-full justify-center")}>
                        {&self.streamer.links}
                    </div>
                    <div class={class("h-1 w-[70%] bg-gray-600 rounded")} ></div>
                </div>
                // top right
                <div class={class("bg-base-light")}></div>
                // bottom left
                <div class={class("bg-base-light")}>

                </div>
                // bottom right
                <div class={class("bg-base rounded-tl-xl")}></div>
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