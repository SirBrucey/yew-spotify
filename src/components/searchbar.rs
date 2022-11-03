use std::rc::Rc;

use web_sys::{Event, FocusEvent};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Routes;

#[function_component(SearchBar)]
pub fn search_bar() -> Html {
    let search_term = Rc::new(use_state(|| "".to_string()));
    let history = use_history().unwrap();

    let search_value = search_term.clone();
    let handle_submit = Callback::from(move |e: FocusEvent| {
        e.prevent_default();
        history.push(Routes::Search {
            search: search_value.to_string(),
        });
    });

    let search_value = search_term.clone();
    html! {
        <form onsubmit={handle_submit} autoComplete="off" class={classes!("p-2", "text-gray-400", "focus-within:text-gray-600")}>
            <label for="search-field" class={classes!("sr-only")}>
                { "Search all files" }
            </label>
            <div class={classes!("flex", "flex-row", "justify-start", "items-center")}>
                <input
                 name="search-field" autoComplete="off" id="search-field"
                 class={classes!("flex-1", "bg-transparent", "border-none", "placeholder-gray-500", "outline-none", "text-base", "text-white", "p-4")}
                 placeholder="Search" type="search" value={ format!("{}", **search_value) }
                 onchange={Callback::from(move |e: Event| search_term.set(e.as_string().unwrap_or("".to_string())))}
        />
            </div>
        </form>
    }
}
