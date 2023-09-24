use yew::{html, virtual_dom::VNode, Children, Component, Context, Html, Properties};

use crate::components::{content::Content, header::Header, sidebar::Sidebar};

pub struct Layout;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {}

impl Component for Layout {
    type Message = ();
    type Properties = LayoutProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div class="layout">
                <div class="header">
                    <Header />
                </div>
                <div class="sidebar">
                    <Sidebar />
                </div>
                <div class="main">
                    <Content />
                </div>
            </div>
        }
    }
}
