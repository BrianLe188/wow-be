use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::feature_usages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FeatureUsage {
    pub id: Uuid,
    pub route_calculation_count: i32,
    pub created_at: NaiveDateTime,
    pub user_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::feature_usages)]
pub struct NewFeatureUsage {
    pub route_calculation_count: i32,
    pub user_id: Uuid,
}
