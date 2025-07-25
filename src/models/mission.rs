use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Clone, Serialize, Debug)]
#[diesel(table_name = crate::schema::missions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Mission {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub exp_reward: i32,
    pub gift_reward_count: Option<i32>,
    pub gift_reward_type: Option<String>,
    pub max_per_day: Option<i32>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = crate::schema::missions)]
pub struct NewMission {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub exp_reward: i32,
    pub gift_reward_count: Option<i32>,
    pub gift_reward_type: Option<String>,
    pub max_per_day: Option<i32>,
}
