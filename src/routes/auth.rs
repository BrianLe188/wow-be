use axum::{
    Router,
    routing::{get, post},
};

use crate::handlers::auth::{apple_sign_in, check_valid_user, sign_in, sign_up};

pub fn auth_routes() -> Router {
    Router::new()
        .route("/user", get(check_valid_user))
        .route("/sign-up", post(sign_up))
        .route("/sign-in", post(sign_in))
        .route("/sign-in/apple", post(apple_sign_in))
}
