use axum::{Extension, Json, extract::Query};
use serde_json::{Value, json};

use crate::{
    config::db::{DbPool, get_conn},
    models::{review::NewReview, user::User},
    services::review::{create_review, get_reviews},
    utils::error_handling::AppError,
};

pub async fn user_review_place(
    Extension(pool): Extension<DbPool>,
    Extension(current_user): Extension<User>,
    Json(mut payload): Json<NewReview>,
) -> Result<Json<Value>, AppError> {
    let user_id = current_user.id;

    payload.user_id = Some(user_id);

    let mut conn = get_conn(pool).await.map_err(AppError::BadRequest)?;

    let new_review = create_review(&mut conn, &payload)
        .await
        .map_err(|_| AppError::BadRequest("Failed to create review.".into()))?;

    Ok(Json(json!({
        "review": new_review
    })))
}

pub async fn search_reviews(
    Extension(pool): Extension<DbPool>,
    Query(query): Query<Value>,
) -> Result<Json<Value>, AppError> {
    let place_id = query
        .get("place_id")
        .ok_or(AppError::BadRequest("Missing place id.".into()))?
        .as_str()
        .unwrap();

    let mut conn = get_conn(pool).await.map_err(AppError::BadRequest)?;

    let reviews = get_reviews(&mut conn, place_id)
        .await
        .map_err(|err| AppError::BadRequest(err.to_string()))?;

    Ok(Json(json!({
        "reviews": reviews
    })))
}
