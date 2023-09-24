use web_sys::{HtmlInputElement, InputEvent};
use yew::{html, Callback, Component, Html, NodeRef, Properties};

#[derive(PartialEq, Properties)]
pub struct SearchInputProps {
    pub onsearch: Callback<String>,
}

pub enum SearchInputMsg {
    Search(InputEvent),
}

pub struct SearchInput {
    input_ref: NodeRef,
}

impl Component for SearchInput {
    type Message = SearchInputMsg;

    type Properties = SearchInputProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            input_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SearchInputMsg::Search(_) => {
                if let Some(input) = self.input_ref.cast::<HtmlInputElement>() {
                    ctx.props().onsearch.emit(input.value());
                }
                false
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let oninput = ctx.link().callback(SearchInputMsg::Search);
        html! {
            <>
                <div class="search-input">
                    <input
                        ref={self.input_ref.clone()}
                        oninput={oninput}
                        class="search"
                        placeholder="Search Note"
                        type="text"
                     />
                </div>
            </>
        }
    }
}
