use yew::prelude::*;

pub struct NavItem {
    name: String
}

pub struct Nav {
    items: Vec<NavItem>
}

impl Component for Nav {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut items = vec![NavItem { name: String::from("random") }];

        Self {
            items
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("")}>

            </div>
        }
    }
}
