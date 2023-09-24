use yew::{html, use_state, Callback, Component, Context, Html};

pub struct Header {
    is_search: bool,
}

pub enum HeaderMsg {
    OnSearch(bool),
}

impl Component for Header {
    type Message = HeaderMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { is_search: false }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="flex items-center dark:text-white gap-2">
                <div class="text-xl font-semibold whitespace-nowrap dark:text-white flex-grow">{"SmartNote"}</div>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HeaderMsg::OnSearch(value) => {
                self.is_search = value;
                true
            }
        }
    }
}
