use yew::{
    Component,
    Context,
    Html, 
    html
};
pub struct Content;

pub enum ContentMsg {
}

impl Component for Content {
    type Message = ContentMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="text-xl whitespace-nowrap dark:text-white">{"Content"}</div>
        }
    }
}