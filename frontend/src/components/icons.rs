use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::utils::class;

pub struct Icons {
    icon: String,
    style: String,
    fill: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or("none".to_string())]
    pub icon: String,

    #[prop_or("".to_string())]
    pub style: String,

    #[prop_or("#000000".to_string())]
    pub fill: String,
}

impl Component for Icons {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();

        let icon = props.icon.clone();
        let style = props.style.clone();
        let fill = props.fill.clone();

        Self {
            icon,
            style,
            fill,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let fill = self.fill.clone();
        let style = self.style.clone();

        match self.icon.as_str().as_ref() {
            "clock" => html! {
                <div>
                    <svg xmlns="http://www.w3.org/2000/svg" fill={fill} viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class={class(format!("w-6 h-6 {style}").as_str())}>
                         <path stroke-linecap="round" stroke-linejoin="round" d="M12 6v6h4.5m4.5 0a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                </div>
            },
            _ => html! {}
        }
    }
}


