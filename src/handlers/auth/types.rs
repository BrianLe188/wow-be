use serde::Serialize;

#[derive(Serialize)]
pub struct ReturnFeatureUsage {
    pub route_calculation_count: i32,
}

#[derive(Serialize)]
pub struct ReturnUser {
    pub id: String,
    pub email: String,
    pub feature_usage: ReturnFeatureUsage,
    pub level: Option<i32>,
}
