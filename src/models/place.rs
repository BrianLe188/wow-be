use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Queryable, Selectable, Clone, Serialize, Debug)]
#[diesel(table_name = crate::schema::places)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Place {
    pub id: Uuid,
    pub place_id: String,
    pub name: String,
    pub formatted_address: Option<String>,
    pub formatted_phone_number: Option<String>,
    pub business_status: Option<String>,
    pub adr_address: Option<String>,
    pub icon: Option<String>,
    pub icon_background_color: Option<String>,
    pub icon_mask_base_uri: Option<String>,
    pub rating: Option<f64>,
    pub user_ratings_total: Option<i32>,
    pub url: Option<String>,
    pub website: Option<String>,
    pub vicinity: Option<String>,
    pub utc_offset: Option<String>,
    pub reference: Option<String>,
    pub geometry: Option<Value>,
    pub types: Option<Vec<Option<String>>>,
    pub address_components: Option<Value>,
    pub plus_code: Option<Value>,
    pub created_at: NaiveDateTime,
    pub range_time_view_count: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Geometry {
    pub location: Location,
    pub viewport: Viewport,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Viewport {
    pub northeast: Location,
    pub southwest: Location,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Location {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = crate::schema::places)]
pub struct NewPlace {
    pub place_id: String,
    pub name: String,
    pub formatted_address: Option<String>,
    pub formatted_phone_number: Option<String>,
    pub business_status: Option<String>,
    pub adr_address: Option<String>,
    pub icon: Option<String>,
    pub icon_background_color: Option<String>,
    pub icon_mask_base_uri: Option<String>,
    pub rating: Option<f64>,
    pub user_ratings_total: Option<i32>,
    pub url: Option<String>,
    pub website: Option<String>,
    pub vicinity: Option<String>,
    pub utc_offset: Option<String>,
    pub reference: Option<String>,
    pub geometry: Option<Value>,
    pub types: Option<Vec<Option<String>>>,
    pub address_components: Option<Value>,
    pub plus_code: Option<Value>,
}
