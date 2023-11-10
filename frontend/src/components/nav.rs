use std::collections::HashMap;

use yew::prelude::*;

pub struct NavItem {
    name: String,
}

pub struct Nav {
    items: Vec<NavItem>,
    styles: HashMap<String, Vec<String>>,
}

impl Component for Nav {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let items = vec![NavItem {
            name: String::from("random"),
        }];

        let styles: HashMap<String, Vec<String>> = HashMap::new();
        Self { items, styles }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let items = &self.items;
        let items: Html = items
            .into_iter()
            .map(|item| {
                html! {
                    <p class={classes!("text-green-500")}>{item.name.clone()}</p>
                }
            })
            .collect();

        html! {
            <div class={classes!("bg-blue-500")}>
                { items }
            </div>
        }
    }
}
