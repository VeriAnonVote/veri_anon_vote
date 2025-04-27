// @generated automatically by Diesel CLI.

diesel::table! {
    vote_record (id) {
        id -> Integer,
        vote_choice -> Text,
        ring_sig -> Binary,
    }
}
