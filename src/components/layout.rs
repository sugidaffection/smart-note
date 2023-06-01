use yew::{Component, Context, Html, html, Children, Properties};

pub struct Layout;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Children
}

impl Component for Layout {
    type Message = ();
    type Properties = LayoutProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id="layout">
                {ctx.props().children.clone() }
            </div>
            
        }
    }
}