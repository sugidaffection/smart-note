use gloo_utils::format::{JsValueSerdeExt};
use wasm_bindgen::{JsValue};
use web_sys::{HtmlElement};
use yew::{
    Component,
    Context,
    Html, 
    html, NodeRef
};

use crate::{Quill, Options};

use super::icon::Icon;
pub struct Content {
    textarea: NodeRef
}

pub enum ContentMsg {
    Bold
}

impl Component for Content {
    type Message = ContentMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            textarea: NodeRef::default()
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="flex flex-col h-full border-l border-gray-700">
                <div class="hidden flex items-center divide-x whitespace-nowrap dark:text-white px-3 py-2 border-b border-gray-700">
                    <div class="flex px-2">

                        <button class="w-8 h-8 active:bg-gray-700 rounded">
                            <Icon
                                icon={"fa fa-bold"}
                                width={16}
                                height={16}
                            />
                        </button>

                        <button class="w-8 h-8 active:bg-gray-700 rounded">
                            <Icon
                                icon={"fa fa-underline"}
                                width={16}
                                height={16}
                            />
                        </button>
                    </div>

                    <div class="flex px-2">
                        <button class="w-8 h-8 active:bg-gray-700 rounded">
                            <Icon
                                icon={"fa fa-indent"}
                                width={16}
                                height={16}
                            />
                        </button>

                        <button class="w-8 h-8 active:bg-gray-700 rounded">
                            <Icon
                                icon={"fa fa-outdent"}
                                width={16}
                                height={16}
                            />
                        </button>
                    </div>
                    <div class="flex px-2">
                        <button class="w-8 h-8 active:bg-gray-700 rounded">
                            <Icon
                                icon={"fa fa-align-left"}
                                width={16}
                                height={16}
                            />
                        </button>


                        <button class="w-8 h-8 active:bg-gray-700 rounded">
                            <Icon
                                icon={"fa fa-align-center"}
                                width={16}
                                height={16}
                            />
                        </button>


                        <button class="w-8 h-8 active:bg-gray-700 rounded">
                            <Icon
                                icon={"fa fa-align-right"}
                                width={16}
                                height={16}
                            />
                        </button>

                        <button class="w-8 h-8 active:bg-gray-700 rounded">
                            <Icon
                                icon={"fa fa-align-justify"}
                                width={16}
                                height={16}
                            />
                        </button>
                    </div>
                    <div class="px-2">
                        <button class="w-8 h-8 active:bg-gray-700 rounded">
                            <Icon
                                icon={"fa fa-list"}
                                width={16}
                                height={16}
                            />
                        </button>
                        <button class="w-8 h-8 active:bg-gray-700 rounded">
                            <Icon
                                icon={"fa fa-heading"}
                                width={16}
                                height={16}
                            />
                        </button>
                    </div>
                </div>
                <div class="p-8 h-full dark:bg-gray-800 overflow-hidden">
                    <div ref={self.textarea.clone()}  id="editor"></div>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        let container = self.textarea.cast::<HtmlElement>().unwrap();
        let options = Options { theme: "snow".into() };
        Quill::new(container, JsValue::from_serde(&options).unwrap());
    }
}