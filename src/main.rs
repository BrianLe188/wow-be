mod __test__;
mod config;
mod handlers;
mod middlewares;
mod models;
mod routes;
mod schema;
mod services;
mod utils;

use axum::{Router, extract::Extension};
use dotenvy::dotenv;
use std::env;
use tower_http::trace::{DefaultMakeSpan, DefaultOnFailure, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::{
    config::{cache::init_cache_pool, db::init_pool, mailer::init_mailer},
    routes::{
        auth::auth_routes, iap::iap_routes, mission::mission_routes, place::place_routes,
        review::review_routes, upload::upload_routes, user::user_routes, waypoint::waypoint_routes,
    },
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    dotenv().expect(".env file not found.");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is missing.");
    let pool = init_pool(&db_url).expect("Failed to init pool.");

    let cache_url = env::var("CACHE_URL").expect("CACHE_URL is missing.");
    let cache_pool = init_cache_pool(&cache_url)
        .await
        .expect("Failed to init cache pool.");

    let mailer_username = env::var("MAILER_USERNAME").expect("MAILER_USERNAME is missing.");
    let mailer_password = env::var("MAILER_PASSWORD").expect("MAILER_PASSWORD is missing.");
    let mailer_relay_mail = env::var("MAILER_RELAY_MAIL").expect("MAILER_RELAY_MAIL is missing.");
    let mailer = init_mailer(&mailer_username, &mailer_password, &mailer_relay_mail);

    let app = Router::new()
        .nest("/auth", auth_routes())
        .nest("/waypoints", waypoint_routes())
        .nest("/iap", iap_routes())
        .nest("/places", place_routes())
        .nest("/reviews", review_routes())
        .nest("/missions", mission_routes())
        .nest("/users", user_routes())
        .nest("/uploads", upload_routes())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
                .on_failure(DefaultOnFailure::new().level(Level::ERROR)),
        )
        .layer(Extension(pool))
        .layer(Extension(cache_pool))
        .layer(Extension(mailer));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
