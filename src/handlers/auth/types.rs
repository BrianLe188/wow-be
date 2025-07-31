use serde::{Deserialize, Serialize};
use validator::Validate;

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
    pub avatar_url: Option<String>,
    pub cover_url: Option<String>,
}

#[derive(Validate, Deserialize)]
pub struct SignInPayload {
    #[validate(email(message = "Please provide a valid email address."))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters."))]
    pub password: String,
}

#[derive(Validate, Deserialize)]
pub struct SignUpPayload {
    #[validate(email(message = "Please provide a valid email address."))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters."))]
    pub password: String,

    pub code: Option<String>,
}

#[derive(Validate, Deserialize)]
pub struct CheckValidUserQuery {
    pub token: String,
}
