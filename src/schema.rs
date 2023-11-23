// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        task -> Varchar,
        done -> Bool,
    }
}
