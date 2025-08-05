// @generated automatically by Diesel CLI.

diesel::table! {
    action_count (id) {
        id -> Uuid,
        user_id -> Uuid,
        review_place -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    exp_history (id) {
        id -> Uuid,
        user_id -> Nullable<Uuid>,
        #[max_length = 20]
        source -> Nullable<Varchar>,
        amount -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    feature_usages (id) {
        id -> Uuid,
        route_calculation_count -> Int4,
        created_at -> Timestamp,
        user_id -> Uuid,
    }
}

diesel::table! {
    missions (id) {
        id -> Uuid,
        #[max_length = 20]
        code -> Varchar,
        #[max_length = 50]
        name -> Varchar,
        description -> Nullable<Text>,
        exp_reward -> Int4,
        gift_reward_count -> Nullable<Int4>,
        #[max_length = 20]
        gift_reward_type -> Nullable<Varchar>,
        max_per_day -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    places (id) {
        id -> Uuid,
        place_id -> Text,
        name -> Text,
        formatted_address -> Nullable<Text>,
        formatted_phone_number -> Nullable<Text>,
        business_status -> Nullable<Text>,
        adr_address -> Nullable<Text>,
        icon -> Nullable<Text>,
        #[max_length = 10]
        icon_background_color -> Nullable<Varchar>,
        icon_mask_base_uri -> Nullable<Text>,
        rating -> Nullable<Float8>,
        user_ratings_total -> Nullable<Int4>,
        url -> Nullable<Text>,
        website -> Nullable<Text>,
        vicinity -> Nullable<Text>,
        utc_offset -> Nullable<Text>,
        reference -> Nullable<Text>,
        geometry -> Nullable<Jsonb>,
        types -> Nullable<Array<Nullable<Text>>>,
        address_components -> Nullable<Jsonb>,
        plus_code -> Nullable<Jsonb>,
        created_at -> Timestamp,
        range_time_view_count -> Int4,
    }
}

diesel::table! {
    reviews (id) {
        id -> Uuid,
        user_id -> Nullable<Uuid>,
        place_id -> Uuid,
        author_name -> Nullable<Text>,
        author_url -> Nullable<Text>,
        #[max_length = 2]
        language -> Nullable<Varchar>,
        profile_photo_url -> Nullable<Text>,
        rating -> Float8,
        relative_time_description -> Nullable<Text>,
        text -> Text,
        time -> Nullable<Int4>,
        created_at -> Timestamp,
        medias -> Nullable<Array<Nullable<Jsonb>>>,
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
        level -> Nullable<Int4>,
        exp -> Nullable<Int4>,
        avatar_url -> Nullable<Text>,
        cover_url -> Nullable<Text>,
    }
}

diesel::joinable!(action_count -> users (user_id));
diesel::joinable!(exp_history -> users (user_id));
diesel::joinable!(feature_usages -> users (user_id));
diesel::joinable!(reviews -> places (place_id));
diesel::joinable!(reviews -> users (user_id));
diesel::joinable!(subscriptions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(action_count, exp_history, feature_usages, missions, places, reviews, subscriptions, users,);
