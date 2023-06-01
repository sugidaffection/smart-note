use yew::{Component, Context, Html, html, Children, Properties};

pub struct Layout;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub header: Children,
    pub sidebar: Children,
    pub main: Children,
}

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
                    { ctx.props().header.clone() }
                </div>
                <div class="sidebar">
                    { ctx.props().sidebar.clone() }
                </div>
                <div class="main">
                    { ctx.props().main.clone() }
                </div>
            </div>
            
        }
    }
}