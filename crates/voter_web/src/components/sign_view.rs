use crate::prelude::*;
// use crate::state::get_app_state;
use crate::state::AppState;

#[derive(PartialEq, Clone)]
enum SignViewState {
    InputNickname,
    Signing,
    Uploading,
    ShowResult, // Can display success or error status_message
}


#[component]
pub fn SignView() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
    let mut view_state = use_signal(|| SignViewState::InputNickname);
    let mut vote_choice_input = use_signal(String::new);
    let pub_ring = use_resource(move || async move {
        app_state().api_client
            .unwrap()
            .get_pub_ring().await
            .unwrap()
        });

    let mut status_message = use_signal(|| None::<String>); // General status/result status_message
    let mut is_error = use_signal(|| false); // Flag to style the status_message as error
                                             // Need domain for upload API call - assuming it's derivable or stored somewhere
                                             // For simplicity, let's assume we can get it from context or a fixed config for now.
                                             // A better approach might be storing it in AppState when fetched in SubKeyView.
    // let server_domain = use_signal(|| "your-server.com".to_string()); // Placeholder - FIX THIS

    let upload_coroutine = use_coroutine(move |mut rx: UnboundedReceiver<String>| {
        // Clone necessary state signals for the async block
        // let mut app_state = get_app_state();
        let mut view_state = view_state;
        let mut status_message = status_message;
        let mut is_error = is_error;
        // let server_domain = server_domain.clone(); // Capture domain

        async move {
            let api_client = app_state().api_client.unwrap();
            while let Some(vote_choice_input) = rx.next().await {
                let pub_ring = match pub_ring() {
                    Some(ring) => ring,
                    None => app_state()
                        .api_client
                        .unwrap()
                        .get_pub_ring()
                        .await
                        .unwrap(),
                };
                // let pub_ring = pub_ring().unwrap_or_else(|| async {
                //     app_state().api_client
                //     .unwrap()
                //     .get_pub_ring().await
                //     .unwrap()
                // });
                let new_vote = NewVoteRecord::new(&vote_choice_input)
                    .sign(api_client.ring_sk, pub_ring);

                view_state.set(SignViewState::Uploading);
                status_message.set(Some(t!("uploading_signature")));
                is_error.set(false);

                // let domain = server_domain.read().clone(); // Read domain inside async task

                match api_client.insert_vote_record(&new_vote).await {
                    Ok(record_id) => {
                        let msg = format!("id = {record_id}");
                        status_message.set(Some(t!("upload_success", msg: &msg)));
                        is_error.set(false);
                        app_state.write().add_log(crate::state::LogLevel::Info, format!("Signature uploaded successfully: {msg}"));
                    }
                    Err(e) => {
                        status_message.set(Some(t!("upload_failed", error: e.to_string())));
                        is_error.set(true);
                        app_state.write().add_error_log(&e.into());
                    }
                }
                view_state.set(SignViewState::ShowResult);
            }
        }
    });

    // Prerequisite check
    let subkey_check = app_state().api_client.clone(); // Clone Option<SubKeyData>
    if subkey_check.is_none() {
        return rsx! {
            div {
                h1 { { t!("sign_and_upload_title") } }
                p { class: "error-status_message", { t!("error_sub_key_needed") } }
            }
        };
    }
    // If we were storing the actual key object, we'd read it here.
    // Using the placeholder identifier for now.
    // let placeholder_subkey_ref = subkey_check.as_ref().map(|k| k.internal_key_ref.clone());
    // let placeholder_pubkey_display = subkey_check.as_ref().map(|k| k.public_key_display.clone());


    let handle_sign_and_upload = move |_| {
        // Need to get the actual SubKey object for signing.
        // This requires either storing it (with security risks) or re-deriving it.
        // For this example, we'll use the placeholder function with just the ref.
        // **IN A REAL APP: Replace this with secure key handling.**
        // let Some(ref internal_ref) = placeholder_subkey_ref else {
        //     status_message.set(Some("内部错误：子密钥引用丢失".to_string()));
        //     is_error.set(true);
        //     view_state.set(SignViewState::ShowResult);
        //     return;
        // };
        // let Some(ref pubkey_display) = placeholder_pubkey_display else {
        //     status_message.set(Some("内部错误：子密钥公钥丢失".to_string()));
        //     is_error.set(true);
        //     view_state.set(SignViewState::ShowResult);
        //     return;
        // };

        view_state.set(SignViewState::Signing);
        status_message.set(Some(t!("generating_signature")));
        is_error.set(false);

        upload_coroutine.send(vote_choice_input());
        // let nickname = vote_choice_input.read().clone();
        // let timestamp = Utc::now().to_rfc3339(); // ISO 8601 format timestamp
        // let new_str = format!("{}::{}", nickname, timestamp);

        // // --- Replace with actual key retrieval and signing ---
        // let dummy_key_for_signing = crypto_placeholders::YourSubKey {
        //     internal_ref: internal_ref.clone(),
        //     public_key_display: pubkey_display.clone(),
        // };
        // match crypto_placeholders::sign_message(&dummy_key_for_signing, &new_str) {
        //     // --- End Replace ---
        //     Ok(signature_placeholder) => {
        //         let signature_data: SignatureData = signature_placeholder.into();
        //         app_state.write().add_log(crate::state::LogLevel::Debug, format!("Generated signature for '{}'", new_str));
        //         // Send signature, pubkey, nickname to the upload coroutine
        //         upload_coroutine.send((signature_data, pubkey_display.clone(), nickname));
        //     }
        //     Err(e) => {
        //         // error!("Error signing status_message: {}", e);
        //         status_message.set(Some(format!("签名失败: {}", e)));
        //         is_error.set(true);
        //         app_state.write().add_error_log(&e);
        //         view_state.set(SignViewState::ShowResult);
        //     }
        // }
    };

    rsx! {
        div {
            h1 { { t!("sign_and_upload_title") } }

            // Display input fields unless showing final result temporarily
            if *view_state.read() != SignViewState::ShowResult {
                p { { t!("sign_and_upload_title") } }
                input {
                    r#type: "text",
                    placeholder: "XiJinPig",
                    value: "{vote_choice_input}",
                    disabled: *view_state.read() != SignViewState::InputNickname,
                    oninput: move |evt| vote_choice_input.set(evt.value().clone()),
                }
                button {
                    onclick: handle_sign_and_upload,
                    disabled: *view_state.read() != SignViewState::InputNickname || vote_choice_input.read().is_empty(),
                    { t!("confirm_sign_upload_button") }
                }
            }

            // Display status/result status_message
            if let Some(msg) = status_message.read().as_ref() {
                p {
                    class: if *is_error.read() { "error-status_message" } else { "success-status_message" },
                    "{msg}"
                }
            }

            // If showing result, provide a button to go back to input
            if *view_state.read() == SignViewState::ShowResult {
                button {
                    onclick: move |_| {
                        status_message.set(None);
                        is_error.set(false);
                        view_state.set(SignViewState::InputNickname);
                    },
                    { t!("back_button") }
                }
            } else if *view_state.read() == SignViewState::Signing || *view_state.read() == SignViewState::Uploading {
                // Optional: Show a spinner or more detailed progress
                p { { t!("processing") } }
            }
        }
    }
}
