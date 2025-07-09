mod config;
mod handlers;
mod models;
mod routes;
mod schema;
mod services;
mod utils;

use axum::{Router, extract::Extension};
use dotenvy::dotenv;
use std::env;

use crate::{
    config::db::init_pool,
    routes::{auth::auth_routes, waypoint::waypoint_routes},
};

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found.");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_UURL is missing.");
    let pool = init_pool(&db_url).expect("Failed to init pool.");

    let app = Router::new()
        .nest("/auth", auth_routes())
        .nest("/waypoints", waypoint_routes())
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
