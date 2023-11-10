use log::info;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use super::utils::get;

#[derive(Debug, PartialEq)]
pub struct Data {
    data: String,
}

pub enum Msg {
    FutureOut(Data),
}

pub struct SideApp {
    local_data: Data,
}

impl Component for SideApp {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let local_data = Data {
            data: String::new(),
        };
        let link = ctx.link().clone();

        info!("some random info");

        spawn_local(async move {
            match get("http://localhost:8080/api/hello").await {
                Ok(response_data) => link.send_message(Msg::FutureOut(Data {
                    data: response_data,
                })),
                Err(e) => {
                    eprintln!("{e}");
                }
            };
        });

        Self { local_data }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FutureOut(data) => self.local_data.data = data.data,
        };
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let display = self.local_data.data.clone();

        html! {
            <div>
                <p>{ display }</p>
            </div>
        }
    }
}
