use axum::{Extension, Json};
use serde_json::{Value, json};

use crate::{
    config::db::{DbPool, get_conn},
    models::mission::NewMission,
    services::mission::{create_mission, get_missions},
    utils::error_handling::AppError,
};

pub async fn search_missions(Extension(pool): Extension<DbPool>) -> Result<Json<Value>, AppError> {
    let mut conn = get_conn(pool).await.map_err(AppError::BadRequest)?;

    let missions = get_missions(&mut conn)
        .await
        .map_err(|err| AppError::BadRequest(err.to_string()))?;

    Ok(Json(json!({
        "missions": missions
    })))
}

pub async fn create_new_mission(
    Extension(pool): Extension<DbPool>,
    Json(payload): Json<NewMission>,
) -> Result<Json<Value>, AppError> {
    let mut conn = get_conn(pool).await.map_err(AppError::BadRequest)?;

    let mission = create_mission(&mut conn, &payload)
        .await
        .map_err(|err| AppError::BadRequest(format!("Failed to create new mission. {}", err)))?;

    Ok(Json(json!({
        "mission": mission
    })))
}
