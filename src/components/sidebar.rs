use yew::{
    Component,
    Context,
    Html, 
    html, function_component, Properties
};

#[derive(PartialEq, Properties)]
pub struct SidebarItemProps {
    name: String,
}

#[function_component(SidebarItem)]
pub fn sidebar_item(props: &SidebarItemProps) -> Html {
    let SidebarItemProps { name } = props;
    html! {
        <li>
            <a href="#" class="flex items-center p-2 text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700">
                <span class="flex-1 ml-3 whitespace-nowrap">{name}</span>
            </a>
        </li>
    }
}

pub struct Sidebar {
    menu: Vec<SidebarItemProps>,
}

impl Component for Sidebar {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            menu: vec![
                SidebarItemProps {
                    name: "Note 1".into(),
                },
                SidebarItemProps {
                    name: "Note 2".into(),
                },
                SidebarItemProps {
                    name: "Note 3".into(),
                }
            ]
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let menu = self.menu.iter().map(|item| html!{
            <SidebarItem
                name={item.name.clone()}
            ></SidebarItem>
        });
        html! {
            <ul class="space-y-2 font-medium">
                {menu.collect::<Html>()}
            </ul>
        }
    }
}