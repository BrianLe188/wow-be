use axum::{
    Router, middleware,
    routing::{patch, post},
};

use crate::{
    handlers::place::{increase_view, upsert_place},
    middlewares::auth::authorization_middleware,
};

pub fn place_routes() -> Router {
    Router::new()
        .route("/{place_id}/increase-view", patch(increase_view))
        .route("/upsert", post(upsert_place))
        .layer(middleware::from_fn(authorization_middleware))
}
