mod utils;

use axum::{Json, Router, routing::post};
use serde_json::{Value, json};

use crate::utils::nearest_neighbor;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/waypoints", post(optimize_waypoints));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn optimize_waypoints(Json(payload): Json<Value>) -> Json<Value> {
    let origin_array = payload.get("origin").unwrap().as_array().unwrap();
    let origin = [
        origin_array[0].as_f64().unwrap(),
        origin_array[1].as_f64().unwrap(),
    ];

    let waypoints_json = payload.get("waypoints").unwrap().as_array().unwrap();
    let waypoints: Vec<[f64; 2]> = waypoints_json
        .iter()
        .map(|pt| {
            let arr = pt.as_array().unwrap();
            [arr[0].as_f64().unwrap(), arr[1].as_f64().unwrap()]
        })
        .collect();

    let path = nearest_neighbor(origin, waypoints);

    Json(json!({ "path": path }))
}
