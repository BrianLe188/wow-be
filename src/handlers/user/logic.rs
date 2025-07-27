use axum::{Extension, Json, extract::Path};
use bb8_redis::redis::AsyncCommands;
use lettre::SmtpTransport;
use rand::Rng;
use serde_json::{Value, json};

use crate::{
    config::{
        cache::{CachePool, get_cache_conn},
        db::{DbPool, get_conn},
        mailer::{mail_template, mailer_send},
    },
    models::user::User,
    services::mission::do_mission,
    utils::error_handling::AppError,
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
    Json(payload): Json<Value>,
) -> Result<Json<Value>, AppError> {
    let to_email = payload
        .get("email")
        .ok_or(AppError::BadRequest("Missing email.".into()))?
        .as_str()
        .unwrap();

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

    if let Ok(mail) = mail_template(to_email, "") {
        mailer_send(&mailer, &mail);
    }

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
