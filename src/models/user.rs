use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Selectable, Clone, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub level: Option<i32>,
    pub exp: Option<i32>,
    pub created_at: NaiveDateTime,
    pub avatar_url: Option<String>,
    pub cover_url: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub avatar_url: Option<String>,
    pub cover_url: Option<String>,
}

#[derive(AsChangeset, Default)]
#[diesel(table_name = crate::schema::users)]
pub struct UserPhotoChangeset {
    pub avatar_url: Option<String>,
    pub cover_url: Option<String>,
}
