use crate::prelude::*;
// use crate::state::get_app_state;
use crate::state::AppState;



#[component]
pub fn SubKeyView() -> Element {
    let app_state = use_context::<Signal<AppState>>();
    if app_state().api_client.is_none() {
         return rsx! {
             div {
                 h1 { { t!("not_exist_subkey") } }
                 div { class: "error-message", { t!("error_main_key_needed") } }
             }
         };
    }

    let api_client = app_state().api_client.unwrap();
    let required_identity = api_client.vote_requirements.unwrap().required_identity;
    let pubkey = api_client.ring_sk.compute_pubkey();
    let pubkey_str = pubkey.to_base58();

    match data_to_qr_png(pubkey_str.as_bytes()) {
        Ok(data_url) => rsx! {
            div {
                p { { t!("generated_auto", required_identity: required_identity) } }
                p {
                    style: "overflow-wrap: break-word;",
                    { t!("public_key_label", pubkey_str: pubkey_str) },
                }
                img {
                    style: "width: 100%; height: auto;",
                    src: "{data_url}",
                }
            }
        },
        Err(err) => rsx! {
            h1 { { t!("qr_generation_failed") } }
            p { class: "error-message", "{err}" }
        }
    }
}

