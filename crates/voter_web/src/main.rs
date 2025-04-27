// use voter_web::prelude::*;
use voter_web::app::App;
// use crate::prelude::*;

use dioxus_logger::tracing::Level;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
}

// #[component]
// fn App() -> Element {
//     rsx! {
//         App {}
//     }
//     // rsx! {
//     //     document::Link { rel: "icon", href: FAVICON }
//     //     document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
//     //     Hero {}

//     // }
// }

// #[component]
// pub fn Hero() -> Element {
//     rsx! {
//         div {
//             id: "hero",
//             img { src: HEADER_SVG, id: "header" }
//             div { id: "links",
//                 a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
//             }
//         }
//     }
// }
