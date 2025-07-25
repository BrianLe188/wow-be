use axum::{Router, middleware, routing::get};

use crate::{
    handlers::mission::{create_new_mission, search_missions},
    middlewares::auth::authorization_middleware,
};

pub fn mission_routes() -> Router {
    Router::new()
        .route("/", get(search_missions).post(create_new_mission))
        .layer(middleware::from_fn(authorization_middleware))
}
