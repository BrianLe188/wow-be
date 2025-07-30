use axum::{Extension, Json};
use serde_json::{Value, json};

use crate::{
    config::db::{DbPool, get_conn},
    models::user::User,
    services::feature_usage::{get_feature_usage_by_user, give_usage_count_to_user},
    utils::{error_handling::AppError, tsp::nearest_neighbor},
};

pub async fn optimize_waypoints(
    Extension(pool): Extension<DbPool>,
    Extension(current_user): Extension<User>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, AppError> {
    let mut conn = get_conn(pool).await.map_err(AppError::BadRequest)?;

    let feature_usage = get_feature_usage_by_user(&mut conn, &current_user.id.to_string())
        .await
        .map_err(|_| AppError::BadRequest("Failed to calculate usage.".into()))?;

    if feature_usage.route_calculation_count == 0 {
        return Err(AppError::BadRequest("You are reach the limit.".into()));
    }

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

    give_usage_count_to_user(&mut conn, &current_user.id.to_string(), -1)
        .await
        .map_err(|_| AppError::BadRequest("Failed to calculate usage.".into()))?;

    Ok(Json(json!({ "path": path })))
}
