use crate::prelude::*;
use crate::state::{
    Module,
    // get_app_state,
    AppState,
    // provide_app_state,
};
use crate::components::{BottomNav, MainKeyView, SubKeyView, SignView, LogView};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
// const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");



#[component]
pub fn App() -> Element {
    use_init_i18n(|| {
    // I18nConfig::new(langid!("en-US")).with_auto_locales(PathBuf::from("../../common_core/i18n/"))
        I18nConfig::new(langid!("en-US"))
            .with_locale((langid!("en-US"), include_str!("../../common_core/i18n/en-US.ftl")))
            .with_locale((
                langid!("zh-CN"),
                include_str!("../../common_core/i18n/zh-CN.ftl"),
            ))
    });
    // Provide the AppState context to all children
    use_context_provider(|| Signal::new(AppState::init()));
    // Get read-only access to the state to determine the current module
    let app_state = use_context::<Signal<AppState>>();
    let current_module = app_state.read().current_module; // Read the current module

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        // Main container structure is handled by index.html and #main div
        // Dioxus renders into the #main div specified in main.rs/lib.rs
        div { class: "content-area", // This div holds the main view content
            match current_module {
                Module::MainKey => rsx! { MainKeyView {} },
                Module::SubKey => rsx! { SubKeyView {} },
                Module::Sign => rsx! { SignView {} },
                Module::Log => rsx! { LogView {} },
            }
        }
        // BottomNav is rendered after the content area
        BottomNav {}
    }
}

