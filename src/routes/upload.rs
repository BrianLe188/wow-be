use axum::{Router, extract::DefaultBodyLimit, middleware, routing::post};

use crate::{
    handlers::upload::{delete as delete_file, upload},
    middlewares::auth::authorization_middleware,
};

pub fn upload_routes() -> Router {
    Router::new()
        .route("/", post(upload).delete(delete_file))
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024))
        .layer(middleware::from_fn(authorization_middleware))
}
