use std::ops::Add;

use web_sys::{MouseEvent, HtmlElement, console, Element};
use yew::{
    Component,
    Context,
    Html, 
    html, function_component, Properties, Callback, use_node_ref, use_effect, NodeRef
};

use super::icon::Icon;

#[derive(PartialEq, Properties, Clone)]
pub struct SidebarItemProps {
    title: String,
    subtitle: String,
    time: String
}

#[function_component(SidebarItem)]
pub fn sidebar_item(props: &SidebarItemProps) -> Html {
    let SidebarItemProps { title, subtitle, time } = props;
    let btn_ref = use_node_ref();
    let onmouseenter = {
        let btn_ref = btn_ref.clone();
        
        move|_| {
            console::log_1(&format!("Log").into());
            if let Some(btn) = btn_ref.cast::<HtmlElement>() {
                let class_name = btn.class_name().clone();
                btn.set_class_name(&class_name.replace("hidden", ""));
            };
        }
    };

    let onmouseleave = {
        let btn_ref = btn_ref.clone();
        
        move|_| {
            if let Some(btn) = btn_ref.cast::<HtmlElement>() {
                let class_name = btn.class_name().clone();
                btn.set_class_name(&class_name.add(" hidden"));
            };
        }
    };
    html! {
        <li {onmouseenter} {onmouseleave}>
            <a href="#" class="grid gap-2 p-2 text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700">
                <div class="flex items-center whitespace-nowrap font-semibold text-md">
                    <span class="flex-grow">{title}</span>
                    <button ref={btn_ref} class="btn text-red-700 hidden !p-0">
                        <Icon icon={"fa fa-trash"} width={16} height={16} />
                    </button>
                </div>
                <div class="flex gap-2 truncate items-center text-slate-200">
                    <span class="text-sm truncate flex-grow">{subtitle}</span>
                    <span class="text-sm">{time}</span>
                </div>
            </a>
        </li>
    }
}

pub struct Sidebar {
    menu: Vec<SidebarItemProps>,
    list_ref: NodeRef
}

pub enum SidebarMsg {
    Add,
    Del
}

impl Component for Sidebar {
    type Message = SidebarMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            menu: vec![
                SidebarItemProps {
                    title: "Huawei Developer Program.".into(),
                    subtitle: "I need to prepared for Huawei developer program.".into(),
                    time: "35 min ago".into()
                },
                SidebarItemProps {
                    title: "OpenHarmony".into(),
                    subtitle: "I need to prepared for Huawei developer program.".into(),
                    time: "35 min ago".into()
                },
                SidebarItemProps {
                    title: "Apple WWDC 2023 ".into(),
                    subtitle: "I need to prepared for Huawei developer program.".into(),
                    time: "35 min ago".into()
                },
                SidebarItemProps {
                    title: "ArkUI".into(),
                    subtitle: "I need to prepared for Huawei developer program.".into(),
                    time: "35 min ago".into()
                },
            ],
            list_ref: NodeRef::default()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let list = self.menu.iter().map(|item| {

            let SidebarItemProps { title, subtitle, time} = item.clone();
            html!{
                <>
                    <SidebarItem
                        title={title}
                        subtitle={subtitle}
                        time={time}
                    />
                    <hr />
                </>
            }

        }).collect::<Html>();

        let onclick = ctx.link().callback(|_| {
            SidebarMsg::Add
        });

        html! {
            <>
                <ul ref={self.list_ref.clone()} class="space-y-2 font-medium flex-grow overflow-auto px-3">
                    {
                        list
                    }
                </ul>
                <div class="px-3 grid">
                    <button class="btn btn-outline-dash" {onclick}>
                        <Icon
                            icon={"fa fa-plus"}
                            width={24}
                            height={24}
                        />

                        <span class="ml-2">{"Add New Note"}</span>
                    </button>
                </div>
                
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SidebarMsg::Add => {
                let new_note = SidebarItemProps {
                    title: "New Note".into(),
                    subtitle: "Note content".into(),
                    time: "0 min ago".into()
                };
                self.menu.insert(0,new_note);
                if let Some(list) = self.list_ref.cast::<HtmlElement>() {
                    let mut options: web_sys::ScrollIntoViewOptions = web_sys::ScrollIntoViewOptions::new();
                    options.behavior(web_sys::ScrollBehavior::Smooth);
                    list.first_element_child().unwrap().scroll_into_view_with_scroll_into_view_options(&options)
                };
                true
            },
            SidebarMsg::Del => {
                false
            }
        }
    }
}