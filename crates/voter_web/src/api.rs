use crate::prelude::*;
use crate::state::{
    // Module,
    AppError,
    // AppState,
    // get_app_state,
}; // Import AppState and get_app_state
   //
// use reqwest::StatusCode;

#[derive(Deserialize, Debug)]
struct RemoteStrResponse {
    remote_str: String,
}

#[derive(Deserialize, Debug)]
struct UploadSuccessResponse {
    message: String,
    // Add other fields the server might return
}

#[derive(Deserialize, Debug)]
struct ErrorResponse {
    error: String,
}

async fn handle_response<T: for<'de> Deserialize<'de>>(response: reqwest::Response) -> Result<T, AppError> {
    let status = response.status();
    if status.is_success() {
        match response.json::<T>().await {
            Ok(data) => Ok(data),
            Err(e) => {
                // error!("Failed to parse success response: {}", e);
                Err(AppError::Api(format!("Failed to parse success response: {}", e)))
            }
        }
    } else {
        let error_text = response.text().await.unwrap_or("Failed to read error body".to_string());
        // error!("Server returned error {}: {}", status, error_text);
        // Try parsing as known error structure
        if let Ok(err_resp) = serde_json::from_str::<ErrorResponse>(&error_text) {
             Err(AppError::Server(status.as_u16(), err_resp.error))
        } else {
            // Generic server error if parsing fails
             Err(AppError::Server(status.as_u16(), error_text))
        }
    }
}


pub async fn fetch_vote_requirements(domain: &str) -> AResult<VoteRequirements> {
    let url = format!("http://{}/vote_requirements", domain); // Adjust API endpoint
    // info!("Fetching remote string from: {}", url);

    let response = reqwest::Client::new()
        .get(&url)
        // .send().await?.error_for_status()?.json()
        .take_data()
        // .await?
        // .json::<T>()
        .await?;
    Ok(response)
        // .send()
        // .await
        // .map_err(|e| AppError::Network(format!("Failed to send request: {}", e)))?;


    // match handle_response::<RemoteStrResponse>(response).await {
    //     Ok(data) => Ok(data.remote_str),
    //     Err(e) => Err(e),
    // }
}

pub async fn fetch_remote_string(domain: &str) -> Result<String, AppError> {
    let url = format!("http://{}/api/get_derivation_string", domain); // Adjust API endpoint
    // info!("Fetching remote string from: {}", url);

    let response = reqwest::Client::new()
        .get(&url)
        .send()
        .await
        .map_err(|e| AppError::Network(format!("Failed to send request: {}", e)))?;

    match handle_response::<RemoteStrResponse>(response).await {
        Ok(data) => Ok(data.remote_str),
        Err(e) => Err(e),
    }
}

pub async fn upload_signature(
    signature_hex: String, // Use appropriate types
    public_key_display: String,
    nickname: String,
    domain: &str, // Need domain to construct URL
) -> Result<String, AppError> {
    let url = format!("http://{}/api/upload_signature", domain); // Adjust API endpoint
    // info!("Uploading signature to: {}", url);

    #[derive(serde::Serialize)]
    struct UploadPayload<'a> {
        signature: &'a str,
        public_key: &'a str,
        nickname: &'a str,
    }

    let payload = UploadPayload {
        signature: &signature_hex,
        public_key: &public_key_display,
        nickname: &nickname,
    };

    let response = reqwest::Client::new()
        .post(&url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| AppError::Network(format!("Failed to send request: {}", e)))?;

     match handle_response::<UploadSuccessResponse>(response).await {
        Ok(data) => Ok(data.message),
        Err(e) => Err(e),
    }
}
