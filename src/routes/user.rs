use axum::{Router, middleware, routing::post};

use crate::{
    handlers::user::{invite, response_invite},
    middlewares::auth::authorization_middleware,
};

pub fn user_routes() -> Router {
    Router::new()
        .route("/invite", post(invite))
        .route("/invite/{action}", post(response_invite))
        .layer(middleware::from_fn(authorization_middleware))
}
