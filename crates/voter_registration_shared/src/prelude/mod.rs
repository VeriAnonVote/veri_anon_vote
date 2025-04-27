#[cfg(not(target_arch = "wasm32"))]
pub mod diesel_sqlite;

#[cfg(not(target_arch = "wasm32"))]
pub use diesel_sqlite::*;

// #[cfg(target_arch = "wasm32")]
pub use common_core::prelude::*;
pub use utoipa::ToSchema;
// pub use common_core::prelude::*;
pub use alloy_key_ring::prelude::eth::*;
pub use alloy_key_ring::wallet::*;

pub use common_core::local_date_time::*;
// pub use voter_registrar::models::voter::Voter;



// pub use crate::config::OrganizerConfig;
