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
    subscriptions (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        environment -> Varchar,
        #[max_length = 255]
        orig_tx_id -> Varchar,
        latest_receipt -> Text,
        start_date -> Timestamp,
        end_date -> Timestamp,
        #[max_length = 255]
        app -> Varchar,
        #[max_length = 255]
        product_id -> Varchar,
        is_cancelled -> Bool,
        validation_response -> Text,
        fake -> Bool,
        created_at -> Timestamp,
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
diesel::joinable!(subscriptions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    feature_usages,
    subscriptions,
    users,
);
