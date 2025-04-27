use crate::prelude::*;
use crate::state::{
    // Module,
    AppState,
    LogLevel,
    LogEntry,
    // AppError,
    // SignatureData,
};



#[component]
pub fn LogView() -> Element {
    let app_state = use_context::<Signal<AppState>>();
    let logs = app_state.read().logs.clone(); // Clone Vec<LogEntry> for rendering

    let all_logs = logs.iter().rev().map(|entry| rsx! {
                    LogEntryItem { key: "{entry.id}", entry: entry.clone() } // Use UUID as key
                 });
    rsx! {
        div {
            h1 { { t!("log") } }
            if logs.is_empty() {
                p { { t!("no_logs") } }
            } else {
                { all_logs }
                 // Use reversed iterator to show newest logs first
            }
        }
    }
}

// Sub-component for individual log entry for better structure
#[component]
fn LogEntryItem(entry: LogEntry) -> Element {
    let timestamp_str = entry.timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string();
    let level_class = match entry.level {
        LogLevel::Info => "log-level-info",
        LogLevel::Error => "log-level-error",
        LogLevel::Debug => "log-level-debug", // Add style if needed
    };
    let level_str = format!("{:?}", entry.level).to_uppercase();

    rsx! {
        div { class: "log-entry",
            span { class: "log-timestamp", "{timestamp_str}" }
            span { class: level_class, "[{level_str}]" }
            span { " {entry.message}" }
        }
    }
}
