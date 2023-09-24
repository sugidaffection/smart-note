use yew::{html, html_nested, Component, Context, Html};

use crate::components::{
    icon::Icon,
    list::{List, ListItem},
    search::SearchInput,
};

#[derive(Clone)]
pub struct Note {
    title: String,
    time: String,
}

pub struct Sidebar {
    data: Vec<Note>,
    filtered_data: Vec<Note>,
}

pub enum SidebarMsg {
    Add,
    Delete,
    UpdateFilteredData(Vec<Note>),
}

impl Component for Sidebar {
    type Message = SidebarMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let data = vec![
            Note {
                title: "Huawei Developer Program.".into(),
                time: "35 min ago".into(),
            },
            Note {
                title: "OpenHarmony".into(),
                time: "35 min ago".into(),
            },
            Note {
                title: "Apple WWDC 2023 ".into(),
                time: "35 min ago".into(),
            },
            Note {
                title: "ArkUI".into(),
                time: "35 min ago".into(),
            },
        ];
        let filtered_data = data.clone();
        Self {
            data,
            filtered_data,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let data = self
            .filtered_data
            .iter()
            .map(|item| {
                let Note {title, time} = item;
                html_nested! {
                <ListItem>
                    <a href="#" class="grid gap-2 p-2 text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700">
                        <div class="flex items-center whitespace-nowrap font-semibold text-md">
                            <span class="flex-grow">{title}</span>
                            <button class="btn text-red-700 hidden !p-0">
                                <Icon icon={"fa fa-trash"} width={16} height={16} />
                            </button>
                        </div>
                        <div class="flex gap-2 truncate items-center text-slate-200">
                            <span class="text-sm">{time}</span>
                        </div>
                    </a>
                </ListItem>
                }
            });

        let onclick = ctx.link().callback(|_| SidebarMsg::Add);
        let filtered_data = self.data.clone();
        let onsearch = ctx.link().callback(move |value: String| {
            let filtered_data: Vec<Note> = filtered_data
                .iter()
                .filter(|data| data.title.to_lowercase().contains(&value.to_lowercase()))
                .cloned()
                .collect();
            SidebarMsg::UpdateFilteredData(filtered_data)
        });

        html! {
            <>
                <SearchInput {onsearch} />
                <List>
                    {
                        for data
                    }

                </List>
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
                let new_note = Note {
                    title: "New Note".into(),
                    time: "0 min ago".into(),
                };
                self.data.insert(0, new_note);
                self.filtered_data = self.data.clone();
                // if let Some(list) = self.list_ref.cast::<HtmlElement>() {
                //     let mut options: web_sys::ScrollIntoViewOptions =
                //         web_sys::ScrollIntoViewOptions::new();
                //     options.behavior(web_sys::ScrollBehavior::Smooth);
                //     list.first_element_child()
                //         .unwrap()
                //         .scroll_into_view_with_scroll_into_view_options(&options)
                // };
                true
            }
            SidebarMsg::Delete => false,
            SidebarMsg::UpdateFilteredData(filtered_data) => {
                self.filtered_data = filtered_data;
                true
            }
        }
    }
}
