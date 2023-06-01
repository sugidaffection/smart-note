use yew::prelude::*;


pub mod components;
use components::header::HeaderComponent;
use components::layout::Layout;

#[function_component]
pub fn App() -> Html {
    html! {
        "hello"
    }
}