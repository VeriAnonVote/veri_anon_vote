pub use common::prelude::*;
pub use common::prelude::server::*;
// pub use common::prelude::client::*;
// pub use common_core::prelude::http_client::*;
pub use alloy_key_ring::prelude::eth::*;
pub use alloy_key_ring::wallet::*;

pub use voter_registration_shared::models::{
    voter::*,
    verifier::*,
};



pub use election_shared;
pub use election_shared::config::OrganizerConfig;
// pub use election_shared::config::*;

pub use voter_registrar::{
    self,
    config::RegistrarConfig,
    handlers::{
        admin::{
            // self,
            AdminDoc,
            // manual_hello,
            upsert_verifier,
            delete_verifier,
            get_all_verifiers_details,
            toggle_registration_status,
        },
        verifier::{
            insert_voter,
            // hello,
        },
        public::{
            // echo,
            get_registration_closed_status,
            get_one_voter,
            get_all_voters,
            get_all_verifiers,
            get_voter_requirements,
        },
    },
};

pub use election_organizer::handlers::{
    admin::{
        // self,
        toggle_election_status,
    },
    public::{
        insert_vote_record,
        get_all_vote_records,
        get_pub_ring,
        get_one_vote_record,
        get_vote_requirements,
    },
};
