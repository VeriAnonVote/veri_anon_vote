#[cfg(not(target_arch = "wasm32"))]
pub use voter_registration_shared::prelude::diesel_sqlite::*;

// #[cfg(target_arch = "wasm32")]
pub use common_core::prelude::*;
pub use alloy_key_ring::prelude::eth::*;
pub use alloy_key_ring::wallet::*;

// pub use voter_registrar::models::voter::Voter;



// pub use crate::config::OrganizerConfig;
