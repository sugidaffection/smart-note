use yew::prelude::*;


pub mod components;
use components::header::Header;
use components::sidebar::Sidebar;
use components::content::Content;
use components::layout::Layout;

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