use axum::{Extension, Json, extract::Path};
use diesel::result::Error::NotFound;
use redis::AsyncCommands;
use serde_json::{Value, json};
use tokio::task;
use uuid::Uuid;

use crate::{
    config::{
        cache::{CacheConn, CachePool, get_cache_conn},
        db::{DbConn, DbPool, get_conn},
    },
    handlers::place::UpsertPlacePayload,
    models::{
        place::{NewPlace, Place},
        review::NewReview,
        user::User,
        user_place_access::NewUserPlaceAccess,
    },
    services::{
        place::{create_place, get_place_by_place_id, increase_place_view},
        review::create_review,
        user_place_access::create_user_place_access,
    },
    utils::{
        error_handling::AppError,
        time::{get_seconds_to_midnight, get_today},
    },
};

const IGNORED_TYPES: &[&str] = &[
    "establishment",
    "point_of_interest",
    "premise",
    "geocode",
    "political",
    "colloquial_area",
    "locality",
    "sublocality",
    "sublocality_level_1",
    "sublocality_level_2",
    "sublocality_level_3",
    "sublocality_level_4",
    "sublocality_level_5",
    "administrative_area_level_1",
    "administrative_area_level_2",
    "administrative_area_level_3",
    "administrative_area_level_4",
    "administrative_area_level_5",
    "country",
    "postal_code",
    "postal_code_prefix",
    "postal_code_suffix",
    "postal_town",
    "neighborhood",
    "route",
    "intersection",
    "street_address",
    "street_number",
    "floor",
    "room",
    "plus_code",
    "natural_feature",
    "airport",
    "place_of_worship",
];

async fn create_user_place_access_today<'a>(conn: &mut DbConn, cache_conn: &mut CacheConn<'a>, user_id: Uuid, place_id: Uuid, types: Option<Vec<Option<String>>>) {
    let today = get_today();

    let cache_key = format!("access:{}:{}:{}", user_id, today, place_id);

    let current_access: Option<bool> = match cache_conn.get(&cache_key).await {
        Ok(access) => access,
        Err(err) => {
            eprintln!("Failed to read access from cache: {}", err);
            return;
        }
    };

    if current_access.is_none() {
        let type_ = types
            .as_ref()
            .and_then(|vec| vec.iter().flatten().find(|s| !IGNORED_TYPES.contains(&s.as_str())))
            .cloned()
            .unwrap_or_default();

        let new_user_place_access = NewUserPlaceAccess { user_id, place_id, type_ };

        if (create_user_place_access(conn, &new_user_place_access).await).is_err() {
            eprintln!("Failed to create user place access.");
        }
    } else {
        let expire_time = get_seconds_to_midnight();

        if let Err(err) = cache_conn.set_ex::<&str, bool, u64>(&cache_key, true, expire_time as u64).await {
            eprintln!("Failed to cache access: {}", err);
        }
    }
}

async fn create_place_with_defaults(conn: &mut DbConn, place: &NewPlace, reviews: &Vec<NewReview>) -> Result<Place, AppError> {
    let place = create_place(conn, place).await.map_err(|_| AppError::BadRequest("Failed to create new place.".into()))?;

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
    Extension(cache_pool): Extension<CachePool>,
    Extension(current_user): Extension<User>,
    Json(payload): Json<UpsertPlacePayload>,
) -> Result<Json<Value>, AppError> {
    let mut conn = get_conn(&pool).await.map_err(AppError::BadRequest)?;

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

    let place_types_clone = existing_place.types.clone();
    let place_id = existing_place.id;
    let user_id = current_user.id;
    let cache_pool_clone = cache_pool.clone();
    let pool_clone = pool.clone();

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

        create_user_place_access_today(&mut conn, &mut cache_conn, user_id, place_id, place_types_clone).await;
    });

    Ok(Json(json!({
        "place": existing_place
    })))
}

pub async fn increase_view(
    Extension(pool): Extension<DbPool>,
    Extension(cache_pool): Extension<CachePool>,
    Extension(current_user): Extension<User>,
    Path(place_id): Path<String>,
) -> Result<Json<Value>, AppError> {
    let mut conn = get_conn(&pool).await.map_err(AppError::BadRequest)?;

    let mut cache_conn = get_cache_conn(&cache_pool).await.map_err(AppError::BadRequest)?;

    let today = get_today();

    let place = {
        let cache_key = format!("view:{}:{}:{}", &current_user.id.to_string(), today, &place_id);

        let current_view: Option<bool> = cache_conn.get(&cache_key).await.map_err(|err| AppError::BadRequest(err.to_string()))?;

        if current_view.is_some() {
            get_place_by_place_id(&mut conn, &place_id).await.map_err(|_| AppError::NotFound("Place not found.".into()))?
        } else {
            let expire_time = 900; // 15 minutes

            if let Err(err) = cache_conn.set_ex::<&str, bool, i32>(&cache_key, true, expire_time).await {
                eprintln!("Failed to cache view: {}", err);
            }

            match increase_place_view(&mut conn, place_id.as_str()).await {
                Ok(place) => place,
                Err(NotFound) => return Err(AppError::NotFound("Place not found.".into())),
                Err(_) => return Err(AppError::BadRequest("Failed to increase view.".into())),
            }
        }
    };

    let user_id = current_user.id;
    let place_id = place.id;
    let place_types_clone = place.types.clone();
    let cache_pool_clone = cache_pool.clone();
    let pool_clone = pool.clone();

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

        create_user_place_access_today(&mut conn, &mut cache_conn, user_id, place_id, place_types_clone).await;
    });

    Ok(Json(json!({
        "place": place
    })))
}
