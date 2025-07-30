use crate::{handlers::waypoint::optimize_waypoints, middlewares::auth::authorization_middleware};
use axum::{Router, middleware, routing::post};

pub fn waypoint_routes() -> Router {
    Router::new()
        .route("/", post(optimize_waypoints))
        .layer(middleware::from_fn(authorization_middleware))
}
