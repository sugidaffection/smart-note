use yew::{Properties, function_component, Html, html};

#[derive(PartialEq, Properties)]
pub struct IconProps {
    pub icon: String,
    pub width: u8,
    pub height: u8
}

#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    let IconProps { icon, width, height } = props;
    html! {
        <i class={icon} style={format!("width:{}px; height: {}px", width, height)}></i>
    }
}