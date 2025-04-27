use crate::prelude::*;
use crate::state::{
    Module,
    // AppError,
    AppState,
};
use crate::crypto_placeholders; // Import placeholder functions

#[derive(PartialEq, Clone, Copy)]
enum MainKeyViewState {
    SelectLang,
    Initial,
    ShowMnemonic,
    InputMnemonic,
}

#[component]
pub fn MainKeyView() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
    let mut view_state = use_signal(|| MainKeyViewState::SelectLang);
    let mut generated_mnemonic = use_signal(|| None::<String>);
    let mut input_mnemonic = use_signal(String::new);
    let mut error_message = use_signal(|| None::<String>);
    let mut is_processing = use_signal(|| false);

    let handle_create = move |_| {
        is_processing.set(true);
        error_message.set(None);
        match WalletSource::new_mnemonic() {
            Ok(WalletSource::Mnemonic(words)) => {
                input_mnemonic.set(words.clone());
                generated_mnemonic.set(Some(words));
                view_state.set(MainKeyViewState::ShowMnemonic);
            },
            Err(e) => {
                let err_msg = format!("Failed to generate keys: {}", e);
                error_message.set(Some(err_msg));
                app_state.write().add_error_log(&e.into());
            }
        }
        is_processing.set(false);
    };

    let handle_recover = move |_| {
        view_state.set(MainKeyViewState::InputMnemonic);
        generated_mnemonic.set(None); // Clear any previously generated mnemonic
        error_message.set(None);
    };

    let handle_confirm_backup = move |_| {
        view_state.set(MainKeyViewState::InputMnemonic);
        // input_mnemonic.set(generated_mnemonic.read().clone().unwrap_or_default()); // Pre-fill if just generated
        generated_mnemonic.set(None); // Clear displayed mnemonic
        error_message.set(None);
    };

    // let mut app_state = use_context::<Signal<AppState>>();
    let handle_confirm_mnemonic = move |_| {
        // is_processing.set(true);
        error_message.set(None);
        let phrase = input_mnemonic.read().clone();
        // spawn_forever(async move {
        spawn(async move {
            match crypto_placeholders::validate_mnemonic(&phrase, &app_state().origin).await {
                Ok(api_client) => {
                    // Validation success!
                    let mut state = app_state.write();
                    state.main_key_mnemonic = Some(phrase);
                    state.api_client = Some(api_client);
                    state.add_log(crate::state::LogLevel::Info, "Main key mnemonic validated and set.".to_string());
                    state.current_module = Module::SubKey; // Navigate
                                                           // Reset local state for this view
                    input_mnemonic.set(String::new());
                    view_state.set(MainKeyViewState::SelectLang);
                }
                Err(e) => {
                    let err_msg = format!("Validation failed: {}", e);
                    error_message.set(Some(err_msg));
                    app_state.write().add_error_log(&e.into());
                }
            }
        });
        // is_processing.set(false);
    };

    rsx! {
        div {
            h1 { { t!("main_key_management") } }

            match view_state() {
                MainKeyViewState::SelectLang => rsx! {
                    // LanguageSelector {}
                    div {
                        h2 { "Select Language / 选择语言" }
                        button {
                            onclick: move |_| {
                                app_state().i18n.set_language(langid!("en-US"));
                                view_state.set(MainKeyViewState::Initial);
                            },
                            "English"
                        }
                        button {
                            onclick: move |_| {
                                app_state().i18n.set_language(langid!("zh-CN"));
                                view_state.set(MainKeyViewState::Initial);
                            },
                            "中文"
                        }
                    }
                },
                MainKeyViewState::Initial => rsx! {
                    p {{ t!("select_action") }}
                    button { onclick: handle_create, disabled: is_processing(), {t!("create_new_keypair")} }
                    button { onclick: handle_recover, disabled: is_processing(), {t!("recover_keypair")} }
                    if let Some(err) = error_message.read().as_ref() {
                        p { class: "error-message", "{err}" }
                    }
                },
                MainKeyViewState::ShowMnemonic => rsx! {
                    p { { t!("backup_mnemonic_prompt") } }
                    if let Some(mnemonic) = generated_mnemonic.read().as_ref() {
                        div { class: "mnemonic-display", "{mnemonic}" }
                    }
                    button { onclick: handle_confirm_backup, { t!("confirm_backup_button")} }
                },
                MainKeyViewState::InputMnemonic => rsx! {
                    p { { t!("enter_mnemonic_prompt") } }
                    textarea {
                        rows: "4",
                        placeholder: "word1 word2 word3...",
                        // value: None::<String>,
                        value: "{input_mnemonic}",
                        oninput: move |evt| input_mnemonic.set(evt.value().clone()),
                    }
                    button {
                        onclick: handle_confirm_mnemonic,
                        disabled: is_processing() || input_mnemonic.read().is_empty(),
                        if is_processing() { { t!("processing")} } else { { t!("confirm_mnemonic_button")} }
                    }
                    if let Some(err) = error_message.read().as_ref() {
                        p { class: "error-message", "{err}" }
                    }
                    // Allow going back to the initial choice
                    button {
                        onclick: move |_| {
                            view_state.set(MainKeyViewState::SelectLang);
                            input_mnemonic.set(String::new());
                            error_message.set(None);
                        },
                        disabled: is_processing(),
                        style: "background-color: grey;", // Style differently
                        { t!("back_button") }
                    }
                },
            }
        }
    }
    }



    // #[component]
    // fn LanguageSelector() -> Element {
    // let app_state = get_app_state();
    // rsx! {
    // }
    // }

