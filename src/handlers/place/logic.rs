use axum::{Extension, Json, extract::Path};
use diesel::result::Error::NotFound;
use serde_json::{Value, json};

use crate::{
    config::db::{DbConn, DbPool, get_conn},
    handlers::place::UpsertPlacePayload,
    models::{
        place::{NewPlace, Place},
        review::NewReview,
    },
    services::{
        place::{create_place, get_place_by_place_id, increase_place_view},
        review::create_review,
    },
    utils::error_handling::AppError,
};

async fn create_place_with_defaults(
    conn: &mut DbConn,
    place: &NewPlace,
    reviews: &Vec<NewReview>,
) -> Result<Place, AppError> {
    let place = create_place(conn, place)
        .await
        .map_err(|_| AppError::BadRequest("Failed to create new place.".into()))?;

    for review in reviews {
        let review_payload = NewReview {
            user_id: None,
            place_id: place.id,
            author_name: review.author_name.clone(),
            author_url: review.author_url.clone(),
            language: review.language.clone(),
            profile_photo_url: review.profile_photo_url.clone(),
            rating: review.rating,
            relative_time_description: review.relative_time_description.clone(),
            text: review.text.clone(),
            time: review.time,
            medias: review.medias.clone(),
        };
        if let Err(err) = create_review(conn, &review_payload).await {
            eprintln!("Failed to create review while creating place: {}", err);
        }
    }

    Ok(place)
}

pub async fn upsert_place(
    Extension(pool): Extension<DbPool>,
    Json(payload): Json<UpsertPlacePayload>,
) -> Result<Json<Value>, AppError> {
    let mut conn = get_conn(pool).await.map_err(AppError::BadRequest)?;

    let place = &payload.place;
    let reviews = &payload.reviews;

    let place_id = &place.place_id;

    let existing_place = match get_place_by_place_id(&mut conn, place_id).await {
        Ok(place) => place,
        Err(NotFound) => create_place_with_defaults(&mut conn, place, reviews).await?,
        Err(err) => {
            return Err(AppError::BadRequest(err.to_string()));
        }
    };

    Ok(Json(json!({
        "place": existing_place
    })))
}

pub async fn increase_view(
    Extension(pool): Extension<DbPool>,
    Path(place_id): Path<String>,
) -> Result<Json<Value>, AppError> {
    let mut conn = get_conn(pool).await.map_err(AppError::BadRequest)?;

    let place = match increase_place_view(&mut conn, place_id.as_str()).await {
        Ok(place) => place,
        Err(NotFound) => return Err(AppError::NotFound("Place not found.".into())),
        Err(_) => return Err(AppError::BadRequest("Failed to increase view.".into())),
    };

    Ok(Json(json!({
        "place": place
    })))
}
