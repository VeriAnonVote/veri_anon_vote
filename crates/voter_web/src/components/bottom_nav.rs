use crate::prelude::*;
use crate::state::{
    Module,
    AppState,
};

#[component]
pub fn BottomNav() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();

    let get_style = |button_module: Module| {
        if app_state.read().current_module == button_module {
            "nav-button nav-button-active"
        } else {
            "nav-button"
        }
    };

    rsx! {
        div { class: "bottom-nav",
            button {
                class: get_style(Module::MainKey),
                onclick: move |_| app_state.write().current_module = Module::MainKey,
                { t!("main_key") }
            }
            button {
                class: get_style(Module::SubKey),
                onclick: move |_| app_state.write().current_module = Module::SubKey,
                { t!("sub_key") }
             }
            button {
                class: get_style(Module::Sign),
                onclick: move |_| app_state.write().current_module = Module::Sign,
                { t!("sign") }
            }
            button {
                class: get_style(Module::Log),
                onclick: move |_| app_state.write().current_module = Module::Log,
                { t!("log") }
            }
        }
    }
}
