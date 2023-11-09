use log::info;
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

async fn get(url: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let text = response.text().await?;
    Ok(text)
}

#[derive(Debug, PartialEq)]
pub struct Data {
    data: String,
}

pub enum Msg {
    FuturerOut(Data),
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
                Ok(response_data) => link.send_message(Msg::FuturerOut(Data {
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
            Msg::FuturerOut(data) => self.local_data.data = data.data,
        };
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // spawn_local(async move {
        //     match get("http://localhost:8080/api/hello").await {
        //         Ok(data) => link.send_message(Msg::FuturerOut(Data { data })),
        //         Err(e) => {
        //             eprintln!("{e}");
        //         }
        //     };
        // });
        let mut display = self.local_data.data.clone();

        if display.is_empty() {
            display = String::from("some random string");
        }

        html! {
            <div>
                <p>{ display }</p>
            </div>
        }
    }
}
