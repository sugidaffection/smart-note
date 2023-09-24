use yew::{html, Children, ChildrenWithProps, Component, Properties};

pub struct ListItem {}
pub enum ListItemMsg {}
#[derive(Properties, PartialEq, Clone)]
pub struct ListItemProps {
    pub children: Children,
}

impl Component for ListItem {
    type Message = ListItemMsg;

    type Properties = ListItemProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
        <>
            <div class="list-item">
                {ctx.props().children.clone()}
            </div>
        </>
        }
    }
}

pub struct List {}
pub enum ListMsg {
    AddItem(ListItem),
}
#[derive(Properties, PartialEq, Clone)]
pub struct ListProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<ListItem>,
}

impl Component for List {
    type Message = ListMsg;

    type Properties = ListProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let child_len = ctx.props().children.len();
        html! {
            <div class="list">
                {for ctx.props().children.iter().enumerate().map(|(index, child)| {
                    html!{
                    <>
                        if index > 0 && index < child_len {
                            <hr />
                        }
                        {child}
                    </>
                    }
                })}
            </div>
        }
    }
}
