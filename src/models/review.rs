use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::reviews)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Review {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub place_id: Uuid,
    pub author_name: Option<String>,
    pub author_url: Option<String>,
    pub language: Option<String>,
    pub profile_photo_url: Option<String>,
    pub rating: f64,
    pub relative_time_description: Option<String>,
    pub text: String,
    pub time: Option<i32>,
    pub created_at: NaiveDateTime,
    pub medias: Option<Vec<Option<Value>>>,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = crate::schema::reviews)]
pub struct NewReview {
    pub user_id: Option<Uuid>,
    pub place_id: Uuid,
    pub author_name: Option<String>,
    pub author_url: Option<String>,
    pub language: Option<String>,
    pub profile_photo_url: Option<String>,
    pub rating: f64,
    pub relative_time_description: Option<String>,
    pub text: String,
    pub time: Option<i32>,
    pub medias: Option<Vec<Option<Value>>>,
}
