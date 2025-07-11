use axum::{
    Json,
    extract::{Extension, Query},
};
use diesel::result::Error::NotFound;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::{
    config::db::{DbConn, DbPool, get_conn},
    handlers::auth::{ReturnFeatureUsage, ReturnUser},
    models::{
        feature_usage::NewFeatureUsage,
        user::{NewUser, User},
    },
    services::{
        feature_usage::{create_feature_usage, get_feature_usage_by_user},
        user::{create_user, get_user_by_email},
    },
    utils::{
        apple::decode_and_verify_identify_token,
        error_handling::AppError,
        hash::{hash_password, verify_password},
        jwt::{sign_token, verify_token},
    },
};

pub async fn create_user_with_defaults(
    conn: &mut DbConn,
    email: &str,
    hashed_password: &str,
) -> Result<User, AppError> {
    let payload = NewUser {
        email,
        password: hashed_password,
    };

    let new_user = create_user(conn, &payload)
        .await
        .map_err(|_| AppError::BadRequest("Failed to create new user.".into()))?;

    let new_feature_usage_payload = NewFeatureUsage {
        route_calculation_count: 5,
        user_id: new_user.id,
    };

    create_feature_usage(conn, &new_feature_usage_payload)
        .await
        .map_err(|_| AppError::BadRequest("Failed to create new user.".into()))?;

    Ok(new_user)
}

pub async fn sign_up(
    Extension(pool): Extension<DbPool>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, AppError> {
    let email = payload
        .get("email")
        .ok_or(AppError::BadRequest("Missing email.".into()))?
        .as_str()
        .unwrap();
    let password = payload
        .get("password")
        .ok_or(AppError::BadRequest("Missing password.".into()))?
        .as_str()
        .unwrap();

    let mut conn = get_conn(pool).await.map_err(AppError::BadRequest)?;

    if get_user_by_email(&mut conn, email).await.is_ok() {
        return Err(AppError::BadRequest("User already existing.".into()));
    }

    let hashed_password = hash_password(password.to_string()).map_err(AppError::BadRequest)?;

    let new_user = create_user_with_defaults(&mut conn, email, &hashed_password).await?;

    let access_token =
        sign_token(new_user.id.to_string(), new_user.email).map_err(AppError::BadRequest)?;

    Ok(Json(json!({"access_token": access_token})))
}

pub async fn sign_in(
    Extension(pool): Extension<DbPool>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, AppError> {
    let email = payload
        .get("email")
        .ok_or(AppError::BadRequest("Missing email.".into()))?
        .as_str()
        .unwrap();
    let password = payload
        .get("password")
        .ok_or(AppError::BadRequest("Missing password.".into()))?
        .as_str()
        .unwrap();

    let mut conn = get_conn(pool).await.map_err(AppError::BadRequest)?;

    let user = match get_user_by_email(&mut conn, email).await {
        Ok(user) => user,
        Err(NotFound) => return Err(AppError::NotFound("User not found.".into())),
        Err(err) => {
            return Err(AppError::BadRequest(err.to_string()));
        }
    };

    let is_match_password = verify_password(password.to_string(), user.password.as_ref())
        .map_err(AppError::BadRequest)?;

    if !is_match_password {
        return Err(AppError::BadRequest("User not found.".into()));
    }

    let access_token = sign_token(user.id.to_string(), user.email).map_err(AppError::BadRequest)?;

    Ok(Json(json!({ "access_token": access_token })))
}

pub async fn apple_sign_in(
    Extension(pool): Extension<DbPool>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, AppError> {
    let token = payload
        .get("token")
        .ok_or(AppError::BadRequest("Missing token".into()))?;

    let apple_claims = decode_and_verify_identify_token(token.as_str().unwrap())
        .await
        .map_err(|err| AppError::BadRequest(err.to_string()))?;

    let user_email = apple_claims.email;

    if let Some(email) = user_email {
        let mut conn = get_conn(pool).await.map_err(AppError::BadRequest)?;

        let user = match get_user_by_email(&mut conn, &email).await {
            Ok(user) => user,
            Err(NotFound) => {
                let hashed_password =
                    hash_password(Uuid::new_v4().to_string()).map_err(AppError::BadRequest)?;

                create_user_with_defaults(&mut conn, &email, &hashed_password).await?
            }
            Err(err) => {
                return Err(AppError::BadRequest(err.to_string()));
            }
        };

        let access_token =
            sign_token(user.id.to_string(), user.email).map_err(AppError::BadRequest)?;

        Ok(Json(json!({ "access_token": access_token })))
    } else {
        Err(AppError::BadRequest("Email not found.".into()))
    }
}

pub async fn check_valid_user(
    Extension(pool): Extension<DbPool>,
    Query(query): Query<Value>,
) -> Result<Json<Value>, AppError> {
    let token = query
        .get("token")
        .ok_or(AppError::BadRequest("Missing token.".into()))?
        .as_str()
        .unwrap();

    let decoded_claims = verify_token(token.to_string()).map_err(AppError::BadRequest)?;

    let mut conn = get_conn(pool).await.map_err(AppError::BadRequest)?;

    let user = get_user_by_email(&mut conn, &decoded_claims.claims.email)
        .await
        .map_err(|err| AppError::BadRequest(err.to_string()))?;

    let feature_usage = get_feature_usage_by_user(&mut conn, user.id)
        .await
        .map_err(|err| AppError::BadRequest(err.to_string()))?;

    let return_user = ReturnUser {
        id: user.id.to_string(),
        email: user.email,
        feature_usage: ReturnFeatureUsage {
            route_calculation_count: feature_usage.route_calculation_count,
        },
    };

    Ok(Json(json!({
        "user": return_user
    })))
}
