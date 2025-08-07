use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::user_place_access)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserPlaceAccess {
    pub id: Uuid,
    pub user_id: Uuid,
    pub place_id: Uuid,
    pub type_: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::user_place_access)]
pub struct NewUserPlaceAccess {
    pub user_id: Uuid,
    pub place_id: Uuid,
    pub type_: String,
}
