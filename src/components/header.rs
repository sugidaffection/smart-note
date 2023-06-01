use yew::{
    Component,
    Context,
    Html, 
    html
};
pub struct Header;

pub enum HeaderMsg {
}

impl Component for Header {
    type Message = HeaderMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="text-xl font-semibold whitespace-nowrap dark:text-white">{"SmartNote"}</div>
        }
    }
}