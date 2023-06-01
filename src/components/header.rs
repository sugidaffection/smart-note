use yew::{
    Component,
    Context,
    Html, 
    html, Callback, use_state
};

use super::search::SearchInput;

pub struct Header {
    is_search: bool
}

pub enum HeaderMsg {
    OnSearch(bool)
}

impl Component for Header {
    type Message = HeaderMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            is_search: false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_search = ctx.link().callback(|value| {
            HeaderMsg::OnSearch(value)
        });
        html! {
            <div class="flex items-center dark:text-white gap-2">
                if !self.is_search {
                    <div class="text-xl font-semibold whitespace-nowrap dark:text-white flex-grow">{"SmartNote"}</div>
                }
                <SearchInput {on_search} is_search={self.is_search} />
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