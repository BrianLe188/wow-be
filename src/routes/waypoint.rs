use crate::handlers::waypoint::optimize_waypoints;
use axum::{Router, routing::post};

pub fn waypoint_routes() -> Router {
    Router::new().route("/", post(optimize_waypoints))
}
