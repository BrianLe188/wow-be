// @generated automatically by Diesel CLI.

diesel::table! {
    feature_usages (id) {
        id -> Uuid,
        route_calculation_count -> Int4,
        created_at -> Timestamp,
        user_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(feature_usages -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    feature_usages,
    users,
);
