// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Text,
        title -> Varchar,
        completed -> Bool,
    }
}
