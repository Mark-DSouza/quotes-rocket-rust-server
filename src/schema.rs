// @generated automatically by Diesel CLI.

diesel::table! {
    quotes (id) {
        id -> Int4,
        author -> Varchar,
        content -> Varchar,
        category -> Varchar,
    }
}
