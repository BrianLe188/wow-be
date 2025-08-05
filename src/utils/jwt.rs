use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
}

pub fn sign_token(sub: String, email: String) -> Result<String, String> {
    let exp = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|err| err.to_string())?.as_secs() + 3600;

    let claims = Claims {
        sub,
        email,
        exp: exp.try_into().unwrap(),
    };

    let secret_key = env::var("JWT_SECRET_KEY").map_err(|err| err.to_string())?;

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_ref())).map_err(|err| err.to_string())
}

pub fn verify_token(token: &str) -> Result<TokenData<Claims>, String> {
    let secret_key = env::var("JWT_SECRET_KEY").map_err(|err| err.to_string())?;

    decode::<Claims>(&token, &DecodingKey::from_secret(secret_key.as_ref()), &Validation::default()).map_err(|err| err.to_string())
}
