// @generated automatically by Diesel CLI.

diesel::table! {
    verifier (id) {
        id -> Integer,
        name -> Text,
        wallet_address -> Binary,
        max_upload_count -> Integer,
        api_key -> Text,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    voter (id) {
        id -> Integer,
        verifier_id -> Integer,
        proof_type -> Text,
        utc_timestamp -> BigInt,
        offset -> Integer,
        voter_pubkey -> Binary,
        version -> SmallInt,
        verifier_sig -> Binary,
        voter_info -> Nullable<Text>,
    }
}

diesel::joinable!(voter -> verifier (verifier_id));

diesel::allow_tables_to_appear_in_same_query!(
    verifier,
    voter,
);
