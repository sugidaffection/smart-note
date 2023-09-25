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
    debounce_timer: Option<i32>,
}

impl Component for SearchInput {
    type Message = SearchInputMsg;

    type Properties = SearchInputProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            input_ref: NodeRef::default(),
            debounce_timer: None,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SearchInputMsg::Search(_) => {
                if let Some(input) = self.input_ref.cast::<HtmlInputElement>() {
                    if let Some(timeout_id) = self.debounce_timer.take() {
                        gloo_utils::window().clear_timeout_with_handle(timeout_id);
                    }

                    let callback = ctx.props().onsearch.clone();

                    self.debounce_timer = gloo::timers::callback::Timeout::new(500, move || {
                        callback.emit(input.value());
                    })
                    .forget()
                    .as_f64()
                    .map(|t| t as i32);
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
