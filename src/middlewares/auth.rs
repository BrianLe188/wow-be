use axum::{Extension, body::Body, extract::Request, middleware::Next, response::Response};
use reqwest::header::AUTHORIZATION;

use crate::{
    config::db::{DbPool, get_conn},
    services::user::get_user_by_email,
    utils::{error_handling::AppError, jwt::verify_token},
};

pub async fn authorization_middleware(
    Extension(pool): Extension<DbPool>,
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, AppError> {
    let auth_header = match req.headers_mut().get(AUTHORIZATION) {
        Some(header) => header
            .to_str()
            .map_err(|_| AppError::Unauthorized("Empty header is not allowed.".into()))?,
        None => {
            return Err(AppError::Unauthorized(
                "Please add the JWT token to the header.".into(),
            ));
        }
    };

    let mut header = auth_header.split_whitespace();

    let (_, token) = (header.next(), header.next());

    let decoded_claims = match token {
        Some(token) => verify_token(token).map_err(AppError::Unauthorized)?,
        None => return Err(AppError::Unauthorized("Missing token.".into())),
    };

    let mut conn = get_conn(&pool)
        .await
        .map_err(|_| AppError::Unauthorized("Something wen't wrong.".into()))?;

    let current_user = get_user_by_email(&mut conn, &decoded_claims.claims.email)
        .await
        .map_err(|err| AppError::Unauthorized(err.to_string()))?;

    req.extensions_mut().insert(current_user);

    Ok(next.run(req).await)
}
