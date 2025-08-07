use axum::{Extension, Json, extract::Query};
use chrono::Utc;
use serde_json::{Value, json};
use tokio::task;

use crate::{
    config::{
        cache::{CachePool, get_cache_conn},
        db::{DbPool, get_conn},
    },
    models::{action_count::UpdateActionCountPayload, review::NewReview, user::User},
    services::{
        action_count::increase_action_count_by_user,
        mission::do_mission,
        review::{create_review, get_reviews},
    },
    utils::error_handling::AppError,
};

pub async fn user_review_place(
    Extension(pool): Extension<DbPool>,
    Extension(cache_pool): Extension<CachePool>,
    Extension(current_user): Extension<User>,
    Json(mut payload): Json<NewReview>,
) -> Result<Json<Value>, AppError> {
    let user_id = current_user.id;

    payload.user_id = Some(user_id);
    payload.time = Some(Utc::now().timestamp_millis() as i32);
    payload.author_name = Some(current_user.email);

    let mut conn = get_conn(&pool).await.map_err(AppError::BadRequest)?;

    let new_review = create_review(&mut conn, &payload).await.map_err(|_| AppError::BadRequest("Failed to create review.".into()))?;

    let cache_pool_clone = cache_pool.clone();
    let pool_clone = pool.clone();
    let user_id_string = current_user.id.to_string();
    let medias = payload.medias;

    task::spawn(async move {
        let mut conn = match get_conn(&pool_clone).await {
            Ok(conn) => conn,
            Err(err) => {
                eprintln!("{}", err);
                return;
            }
        };

        let mut cache_conn = match get_cache_conn(&cache_pool_clone).await {
            Ok(conn) => conn,
            Err(err) => {
                eprintln!("{}", err);
                return;
            }
        };

        let media_count = medias.iter().len() as i32;

        if media_count > 0 {
            let upload_photo_code = "UPLOAD_PHOTO";

            if let Err(err) = do_mission(&mut conn, &mut cache_conn, &user_id_string, upload_photo_code, Some(media_count)).await {
                eprintln!("Failed to do mission: {} - {}", upload_photo_code, err)
            };
        }

        let review_code = "REVIEW_PLACE";

        if let Err(err) = do_mission(&mut conn, &mut cache_conn, &user_id_string, review_code, None).await {
            eprintln!("Failed to do mission: {} - {}", review_code, err)
        }

        let increase_payload = UpdateActionCountPayload { review_place: Some(1) };

        if let Err(err) = increase_action_count_by_user(&mut conn, &user_id_string, &increase_payload).await {
            eprintln!("Failed to increase action count: {} - {}", review_code, err)
        }
    });

    Ok(Json(json!({
        "review": new_review
    })))
}

pub async fn search_reviews(Extension(pool): Extension<DbPool>, Query(query): Query<Value>) -> Result<Json<Value>, AppError> {
    let place_id = query.get("place_id").ok_or(AppError::BadRequest("Missing place id.".into()))?.as_str().unwrap();

    let mut conn = get_conn(&pool).await.map_err(AppError::BadRequest)?;

    let reviews = get_reviews(&mut conn, place_id).await.map_err(|err| AppError::BadRequest(err.to_string()))?;

    Ok(Json(json!({
        "reviews": reviews
    })))
}
