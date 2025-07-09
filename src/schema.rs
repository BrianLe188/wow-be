// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password -> Text,
        created_at -> Timestamp,
    }
}
