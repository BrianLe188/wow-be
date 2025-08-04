mod __test__;
mod config;
mod handlers;
mod middlewares;
mod models;
mod routes;
mod schema;
mod services;
mod utils;

use dotenvy::dotenv;
use tracing::{Level, info};

use crate::config::app::{init_listener, init_production_app};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    dotenv().ok();

    let app = init_production_app().await;

    let (listener, port) = init_listener().await;

    info!(
        "\n\n==============================\n  🚀 Service is running!  \n      🌐 Port: {}\n==============================\n",
        port
    );

    axum::serve(listener, app).await.unwrap();
}
