use axum::{Router, middleware, routing::get};

use crate::{
    handlers::review::{search_reviews, user_review_place},
    middlewares::auth::authorization_middleware,
};

pub fn review_routes() -> Router {
    Router::new()
        .route("/", get(search_reviews).post(user_review_place))
        .layer(middleware::from_fn(authorization_middleware))
}
