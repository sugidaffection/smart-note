use yew::{Properties, function_component, Html, html, Callback, use_state, use_node_ref, use_effect, use_effect_with_deps, use_callback,};
use web_sys::{HtmlInputElement, console};
use super::icon::Icon;

#[derive(PartialEq, Properties)]
pub struct SearchInputProps {
    pub on_search: Callback<bool>,
    pub is_search: bool
}

#[function_component(SearchInput)]
pub fn search_input(props: &SearchInputProps) -> Html {
    let is_search = use_state(|| props.is_search);
    let input_ref = use_node_ref();
    let onclick = {
        let is_search = is_search.clone();
        let on_search = props.on_search.clone();
        
        let input_ref = input_ref.clone();

        use_effect(move || {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                input.focus();
            };
        });

        
        
        move |_| {
            let value = *is_search;
            is_search.set(!value);
            on_search.emit(!*is_search);
        }
    };

    html! {
        if *is_search {
            <div class="flex-grow text-black">
                <input ref={input_ref.clone()} class="w-full rounded p-1 outline-none" placeholder="Search Note" type="text" />
            </div>
            <button class="btn !p-0 w-8 h-8" {onclick}>
                <Icon
                    icon={"fa fa-close"}
                    width={16}
                    height={16}                
                />
            </button>
        } else {
            <button class="btn !p-0 w-8 h-8" {onclick}>
                <Icon
                    icon={"fa fa-search"}
                    width={16}
                    height={16}                
                />
            </button>
        }
    }
}