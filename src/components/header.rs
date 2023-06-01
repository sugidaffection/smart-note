use yew::{
    Component,
    Context,
    Html, 
    html
};
pub struct HeaderComponent;

pub enum HeaderComponentMsg {
}

impl Component for HeaderComponent {
    type Message = HeaderComponentMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container p-2 bg-slate-100">{"This is Header World"}</div>
        }
    }
}