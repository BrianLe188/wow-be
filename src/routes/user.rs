use axum::{
    Router, middleware,
    routing::{get, post, put},
};

use crate::{
    handlers::user::{check_in, get_profile, invite, update_photo},
    middlewares::auth::authorization_middleware,
};

pub fn user_routes() -> Router {
    Router::new()
        .route("/{user_id}", get(get_profile))
        .route("/photo", put(update_photo))
        .route("/check-in", get(check_in))
        .route("/invite", post(invite))
        .layer(middleware::from_fn(authorization_middleware))
}
