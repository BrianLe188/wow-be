use axum::Json;
use serde_json::{Value, json};

use crate::utils::tsp::nearest_neighbor;

pub async fn optimize_waypoints(Json(payload): Json<Value>) -> Json<Value> {
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
