use crate::prelude::*;



// --- Placeholder Types (Replace) ---
// Use your actual key/signature types here
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubKeyData {
    pub public_key_display: String, // For display purposes
    // IMPORTANT: Avoid storing raw private key material directly in state
    // if possible. This is a simplification. Consider secure storage or
    // keeping it scoped within function calls if feasible.
    // If you MUST store it, be extremely careful.
    #[serde(skip)] // Don't serialize sensitive parts if state is ever persisted
    pub internal_key_ref: String, // Example: A path or identifier, not the key itself
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignatureData {
    pub signature_hex: String, // Example representation
}
// --- End Placeholder Types ---

// #[derive(thiserror::Error, Debug, Clone, PartialEq, derive_more::Display)]
#[derive(thiserror::Error, Debug, Clone, PartialEq, )]
pub enum AppError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("API error: {0}")]
    Api(String),
    #[error("Server error ({0}): {1}")]
    Server(u16, String),
    #[error("Crypto error: {0}")]
    Crypto(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Input error: {0}")]
    Input(String),
    #[error("Operation failed: {0}")]
    Operation(String), // General failure
}

// impl<E> From<E> for AppError
// where
//     E: core::error::Error + Send + Sync + 'static,
// {
//     fn from(error: E) -> Self {
//         AppError::Operation(error.to_string())
//     }
// }
impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        AppError::Operation(error.to_string())
    }
}

#[derive(Clone, Debug, PartialEq, Copy, Serialize, Deserialize)]
pub enum Module {
    MainKey,
    SubKey,
    Sign,
    Log,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LogLevel {
    Info,
    Error,
    Debug,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: String, // Use uuid for unique key in lists
    pub timestamp: chrono::DateTime<Utc>,
    pub message: String,
    pub level: LogLevel,
}

// Shared application state provided via context
#[derive(Clone)]
pub struct AppState {
    pub current_module: Module,
    // Store the mnemonic securely if needed, or re-derive keys on demand
    // Storing it in signal memory is okay for this example, but be aware of risks.
    pub main_key_mnemonic: Option<String>,
    pub origin: String,
    pub sub_key_data: Option<SubKeyData>,
    pub api_client: Option<VoterApiClient>,
    pub i18n: I18n,
    pub logs: Vec<LogEntry>,
}

impl AppState {
    pub fn init() -> Self {
        let origin = if cfg!(debug_assertions) {
            "http://localhost:38081".to_string()
        } else {
            web_sys::window()
                .unwrap()
                .location().origin()
                .unwrap()
        };

        let mut i18n = i18n();
        i18n.set_language(langid!("en-US"));
        Self {
            current_module: Module::MainKey,
            main_key_mnemonic: None,
            origin,
            sub_key_data: None,
            api_client: None,
            i18n,
            logs: Vec::new(),
        }
    }

    pub fn add_log(&mut self, level: LogLevel, message: String) {
        // log::info!("Adding log ({:?}): {}", level, message); // Also log to console
        let id = Uuid::new_v4().to_string();
        self.logs.push(LogEntry {
            id,
            timestamp: Utc::now(),
            message,
            level,
        });
        // Optional: Limit log size
        if self.logs.len() > 100 {
            self.logs.remove(0);
        }
    }

    pub fn add_error_log(&mut self, error: &AppError) {
        self.add_log(LogLevel::Error, error.to_string());
    }
}
