use yew::{Component, Context, Html};

struct Element {
    name: String,
    route: String,
}

struct Elements {
    elements: Vec<Element>,
}

impl Component for Element {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        todo!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        todo!()
    }
}