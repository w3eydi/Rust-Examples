// @generated automatically by Diesel CLI.

diesel::table! {
    tweets (id) {
        id -> Int4,
        message -> Varchar,
        created_at -> Timestamp,
    }
}
