use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::{
    handlers::iap::{get_user_subscription, save_receipt},
    middlewares::auth::authorization_middleware,
};

pub fn iap_routes() -> Router {
    Router::new()
        .route("/save-receipt", post(save_receipt))
        .route("/user-subscription/{app_type}", get(get_user_subscription))
        .layer(middleware::from_fn(authorization_middleware))
}
