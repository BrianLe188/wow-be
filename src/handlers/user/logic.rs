use axum::{Extension, Json, extract::Path};
use axum_valid::Valid;
use bb8_redis::redis::AsyncCommands;
use lettre::SmtpTransport;
use rand::Rng;
use serde_json::{Value, json};
use tokio::task;
use tokio_retry::{
    Retry,
    strategy::{ExponentialBackoff, jitter},
};

use crate::{
    config::{
        cache::{CachePool, get_cache_conn},
        db::{DbPool, get_conn},
        mailer::{mail_template, mailer_send},
        storage::delete_file,
    },
    handlers::user::InvitePayload,
    models::user::User,
    services::{
        action_count::get_action_count_by_user,
        feature_usage::get_feature_usage_by_user,
        mission::do_mission,
        user::{get_user_by_id, update_user_photo},
    },
    utils::{error_handling::AppError, mail_template::invite_user_mail_body},
};

fn generate_pin_code() -> String {
    let mut rng = rand::rng();
    (0..6)
        .map(|_| rng.random_range(0..10).to_string())
        .collect()
}

pub async fn invite(
    Extension(cache_pool): Extension<CachePool>,
    Extension(current_user): Extension<User>,
    Extension(mailer): Extension<SmtpTransport>,
    Valid(Json(payload)): Valid<Json<InvitePayload>>,
) -> Result<Json<Value>, AppError> {
    let to_email = payload.email;

    let code = generate_pin_code();

    let mut cache_conn = get_cache_conn(&cache_pool)
        .await
        .map_err(AppError::BadRequest)?;

    let key = format!("invite:{}", code);
    let expire_seconds = 900;

    let _: () = cache_conn
        .set(&key, current_user.id.to_string())
        .await
        .map_err(|e| AppError::BadRequest(format!("Redis error: {}", e)))?;
    let _: () = cache_conn
        .expire(&key, expire_seconds)
        .await
        .map_err(|e| AppError::BadRequest(format!("Redis error: {}", e)))?;

    task::spawn(async move {
        let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

        let result = Retry::spawn(retry_strategy, || async {
            let invite_mail_body = invite_user_mail_body("")?;

            let mail = mail_template(&to_email, &invite_mail_body)?;

            mailer_send(&mailer, &mail)
        })
        .await;

        if let Err(err) = result {
            eprintln!("Failed after retries: {}", err);
        }
    });

    Ok(Json(json!({})))
}

pub async fn response_invite(
    Extension(pool): Extension<DbPool>,
    Extension(cache_pool): Extension<CachePool>,
    Path(action): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, AppError> {
    let code = payload
        .get("code")
        .ok_or(AppError::BadRequest("Missing code.".into()))?
        .as_str()
        .unwrap();

    let mut cache_conn = get_cache_conn(&cache_pool)
        .await
        .map_err(AppError::BadRequest)?;
    let mut conn = get_conn(pool).await.map_err(AppError::BadRequest)?;

    let key = format!("invite:{}", code);

    let inviter_id: String = cache_conn
        .get(&key)
        .await
        .map_err(|e| AppError::BadRequest(format!("Redis error: {}", e)))?;

    match action.as_str() {
        "accept" => do_mission(&mut conn, &mut cache_conn, &inviter_id, code, None)
            .await
            .map_err(|_| AppError::BadRequest("Failed to accept invite.".into()))?,
        "reject" => {}
        _ => {}
    };

    let _: () = cache_conn
        .del(&key)
        .await
        .map_err(|e| AppError::BadRequest(format!("Redis error: {}", e)))?;

    Ok(Json(json!({})))
}

pub async fn get_profile(
    Extension(pool): Extension<DbPool>,
    Path(user_id): Path<String>,
) -> Result<Json<Value>, AppError> {
    let mut conn = get_conn(pool).await.map_err(AppError::BadRequest)?;

    let mut user = get_user_by_id(&mut conn, &user_id)
        .await
        .map_err(|_| AppError::NotFound("User not found.".into()))?;

    user.password = String::from("");

    let feature_usage = get_feature_usage_by_user(&mut conn, &user.id.to_string())
        .await
        .map_err(|_| AppError::NotFound("User not found.".into()))?;

    let action_count = get_action_count_by_user(&mut conn, user.id)
        .await
        .map_err(|_| AppError::NotFound("User not found.".into()))?;

    Ok(Json(json!({
        "profile":{
    "user": user,
            "feature_usage": feature_usage,
        "action_count": action_count
        }
    })))
}

pub async fn update_photo(
    Extension(pool): Extension<DbPool>,
    Extension(current_user): Extension<User>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, AppError> {
    let field = payload
        .get("field")
        .ok_or(AppError::BadRequest("Missing field.".into()))?
        .as_str()
        .unwrap();
    let photo_url = payload
        .get("photo_url")
        .ok_or(AppError::BadRequest("Missing photo url.".into()))?
        .as_str()
        .unwrap();

    let mut conn = get_conn(pool).await.map_err(AppError::BadRequest)?;

    let user = update_user_photo(&mut conn, &current_user.id.to_string(), field, photo_url)
        .await
        .map_err(|_| AppError::BadRequest("Failed to update.".into()))?;

    let field = field.to_string();

    task::spawn(async move {
        let mut path = String::from("");

        if field == "avatar_url" {
            if let Some(url) = current_user.avatar_url {
                path = url.clone();
            }
        }

        if field == "cover_url" {
            if let Some(url) = current_user.cover_url {
                path = url.clone();
            }
        }

        if !path.is_empty() {
            let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

            let result = Retry::spawn(retry_strategy, || async { delete_file(&path).await }).await;

            if let Err(err) = result {
                eprintln!("Failed after retries: {}", err);
            }
        }
    });

    Ok(Json(json!({
        "user": user
    })))
}

pub async fn check_in(
    Extension(pool): Extension<DbPool>,
    Extension(cache_pool): Extension<CachePool>,
    Extension(current_user): Extension<User>,
) -> Result<Json<Value>, AppError> {
    let mut conn = get_conn(pool).await.map_err(AppError::BadRequest)?;

    let mut cache_conn = get_cache_conn(&cache_pool)
        .await
        .map_err(AppError::BadRequest)?;

    do_mission(
        &mut conn,
        &mut cache_conn,
        &current_user.id.to_string(),
        "DAILY_CHECK_IN",
        None,
    )
    .await
    .map_err(|_| AppError::BadRequest("Failed to check in.".into()))?;

    Ok(Json(json!({})))
}
