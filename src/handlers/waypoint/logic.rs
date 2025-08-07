use axum::{Extension, Json};
use serde_json::{Value, json};
use tokio::task;
use tokio_retry::{
    Retry,
    strategy::{ExponentialBackoff, jitter},
};

use crate::{
    config::db::{DbPool, get_conn},
    handlers::waypoint::OptimizeWaypointPayload,
    models::user::User,
    services::feature_usage::{get_feature_usage_by_user, give_usage_count_to_user},
    utils::{error_handling::AppError, tsp::nearest_neighbor},
};

pub async fn optimize_waypoints(Extension(pool): Extension<DbPool>, Extension(current_user): Extension<User>, Json(payload): Json<OptimizeWaypointPayload>) -> Result<Json<Value>, AppError> {
    let mut conn = get_conn(&pool).await.map_err(AppError::BadRequest)?;

    let feature_usage = get_feature_usage_by_user(&mut conn, &current_user.id.to_string())
        .await
        .map_err(|_| AppError::BadRequest("Failed to calculate usage.".into()))?;

    if feature_usage.route_calculation_count == 0 {
        return Err(AppError::BadRequest("You are reach the limit.".into()));
    }

    let mut origin = payload.origin;
    let waypoints = payload.waypoints;

    let mut full_path: Vec<[f64; 2]> = Vec::new();

    for group in waypoints {
        let mut group_path = nearest_neighbor(origin, group);

        match group_path.last() {
            Some(point) => origin = *point,
            None => continue,
        }

        full_path.append(&mut group_path);
    }

    let user_id_string = current_user.id.to_string();
    let pool_clone = pool.clone();

    task::spawn(async move {
        let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

        let result = Retry::spawn(retry_strategy, || async {
            let mut conn = match get_conn(&pool_clone).await {
                Ok(conn) => conn,
                Err(err) => {
                    return Err(err.to_string());
                }
            };

            give_usage_count_to_user(&mut conn, &user_id_string, -1).await.map_err(|err| err.to_string())
        })
        .await;

        if let Err(err) = result {
            eprintln!("Failed to calculate usage: {}", err);
        }
    });

    Ok(Json(json!({ "path": full_path })))
}
