use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::action_count)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ActionCount {
    pub id: Uuid,
    pub user_id: Uuid,
    pub review_place: Option<i32>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::action_count)]
pub struct NewActionCount {
    pub user_id: Uuid,
    pub review_place: Option<i32>,
}

pub struct UpdateActionCountPayload {
    pub review_place: Option<i32>,
}

#[derive(AsChangeset, Default)]
#[diesel(table_name = crate::schema::action_count)]
pub struct ActionCountChangeset {
    pub review_place: Option<i32>,
}
