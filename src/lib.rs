use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use yew::prelude::*;

pub mod components;
use components::content::Content;
use components::header::Header;
use components::layout::Layout;
use components::sidebar::Sidebar;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = document)]
    fn getElementById(s: &str) -> HtmlElement;
}

#[derive(Serialize, Deserialize)]
pub struct Options {
    pub theme: String,
}

#[wasm_bindgen]
extern "C" {
    type Quill;

    #[wasm_bindgen(js_name = namespace)]
    static DEFAULTS: JsValue;

    #[wasm_bindgen(constructor)]
    fn new(container: HtmlElement, options: JsValue) -> Quill;
}

#[function_component]
pub fn App() -> Html {
    html! {
        <Layout />
    }
}
