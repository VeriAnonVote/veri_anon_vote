pub mod server;
// pub mod client;
// pub mod eth;
pub mod tui;

pub use common_core::prelude::*;

// #[cfg(feature = "scalar")]
// pub use common_core::prelude::scalar::*;


pub use tokio;
pub use tokio::sync::OnceCell;
pub use tracing::{
    instrument,
    info,
    debug,
    trace,
};
pub use tracing::level_filters::LevelFilter;
pub use tracing_subscriber::{
    self,
    EnvFilter,
};
