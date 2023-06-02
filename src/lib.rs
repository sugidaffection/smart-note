use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement};
use serde::{Serialize, Deserialize};

pub mod components;
use components::header::Header;
use components::sidebar::Sidebar;
use components::content::Content;
use components::layout::Layout;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = document)]
    fn getElementById(s: &str) -> HtmlElement;
}

#[derive(Serialize, Deserialize)]
pub struct Options {
    pub theme: String
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
        <Layout
            header={
                html!{ <Header /> }
            }

            sidebar={
                html!{ <Sidebar /> }
            }

            main={
                html!{ <Content /> }
            }
        />
    }
}