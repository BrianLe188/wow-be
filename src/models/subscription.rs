use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Clone)]
#[diesel(table_name = crate::schema::subscriptions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Subscription {
    pub id: Uuid,
    pub user_id: Uuid,
    pub environment: String,
    pub orig_tx_id: String,
    pub latest_receipt: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub app: String,
    pub product_id: String,
    pub is_cancelled: bool,
    pub validation_response: String,
    pub fake: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::subscriptions)]
pub struct NewSubscription<'a> {
    pub user_id: Uuid,
    pub environment: &'a str,
    pub orig_tx_id: &'a str,
    pub latest_receipt: &'a str,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub app: &'a str,
    pub product_id: &'a str,
    pub is_cancelled: bool,
    pub validation_response: &'a str,
    pub fake: bool,
}
