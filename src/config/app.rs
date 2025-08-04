use std::env;

use crate::routes::{
    auth::auth_routes, iap::iap_routes, mission::mission_routes, place::place_routes,
    review::review_routes, upload::upload_routes, user::user_routes, waypoint::waypoint_routes,
};
use axum::{Extension, Router};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnFailure, DefaultOnResponse, TraceLayer};
use tracing::Level;

use super::{cache::init_cache_pool, db::init_pool, mailer::init_mailer};

async fn init_app(
    db_url: &str,
    cache_url: &str,
    mailer_username: &str,
    mailer_password: &str,
    mailer_relay_mail: &str,
) -> Router {
    let pool = init_pool(db_url).expect("Failed to init pool.");

    let cache_pool = init_cache_pool(cache_url)
        .await
        .expect("Failed to init cache pool.");

    let mailer = init_mailer(mailer_username, mailer_password, mailer_relay_mail);

    Router::new()
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
        .layer(Extension(mailer))
}

pub async fn init_production_app() -> Router {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is missing.");
    let cache_url = env::var("CACHE_URL").expect("CACHE_URL is missing.");
    let mailer_username = env::var("MAILER_USERNAME").expect("MAILER_USERNAME is missing.");
    let mailer_password = env::var("MAILER_PASSWORD").expect("MAILER_PASSWORD is missing.");
    let mailer_relay_mail = env::var("MAILER_RELAY_MAIL").expect("MAILER_RELAY_MAIL is missing.");

    init_app(
        &db_url,
        &cache_url,
        &mailer_username,
        &mailer_password,
        &mailer_relay_mail,
    )
    .await
}

pub async fn init_test_app() -> Router {
    let db_url = env::var("DATABASE_URL_TEST").expect("DATABASE_URL_TEST is missing.");
    let cache_url = env::var("CACHE_URL_TEST").expect("CACHE_URL is missing.");
    let mailer_username = env::var("MAILER_USERNAME_TEST").expect("MAILER_USERNAME is missing.");
    let mailer_password = env::var("MAILER_PASSWORD_TEST").expect("MAILER_PASSWORD is missing.");
    let mailer_relay_mail =
        env::var("MAILER_RELAY_MAIL_TEST").expect("MAILER_RELAY_MAIL is missing.");

    init_app(
        &db_url,
        &cache_url,
        &mailer_username,
        &mailer_password,
        &mailer_relay_mail,
    )
    .await
}

pub async fn init_listener() -> (TcpListener, String) {
    let port = env::var("PORT").unwrap_or("8080".to_string());

    (
        TcpListener::bind(format!("0.0.0.0:{}", port))
            .await
            .unwrap(),
        port,
    )
}
