use jsonwebtoken::{DecodingKey, Validation, decode, decode_header};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct AppleClaims {
    sub: String,
    pub email: Option<String>,
    email_verified: Option<String>,
}

///
/// # Examples:
/// {
///   "keys": [
///     {
///       "kty": "RSA",
///       "kid": "E6q83RB15n",
///       "use": "sig",
///       "alg": "RS256",
///       "n": "qD2kjZNSBESRVJksHHnDpMPprhCymecPO8Ji6xlY_fGdUOioVf0nckGaiBwjPGo3xKadAGvbNJ1BjCZOmbLL7lQ5mT8fI6l5HaY8txcz3_PjOUHdiXBuThmQ2eEXtmOtRxi3LNnXaOCpl7QxHgyiPTVgJpJ18Teqz2ESVXg_Lpmw7ot3zBI0p9E56-HVZwxpwS8EoN53nx850fxAlpZj5d1szgV8YzhcRG-8FMOialu-me0OFZWghB-_jCMfdBhWHMWpGkfLPDA1o8eLkr0UByZwMHKCWA--JUvlKvSv3xavDD7ILj8t5PiItonVV9telbza-ToaOWMiG5gZ5QfWDQ",
///       "e": "AQAB"
///     },
///     {
///       "kty": "RSA",
///       "kid": "UaIIFY2fW4",
///       "use": "sig",
///       "alg": "RS256",
///       "n": "sxzLtSWjplO4nMymVwkknn6WrQvK4sz7F1rrIwOKPa3SpltaB719cfxFfoE4UqHfVxHXsoYew82ViYz5whp0CuqgWi2t4HYpSTCQdVCNIXpsMxA8QqTfIlc-EUFNuUMziY-hJXqi4i-woI0HiwPEkO-AhWy86L9-J_1I1yw22-BICacAU7J9UTBBwHu0wkRHiyPe4pHow1wa91v5OM09XHqjHpiFrJD7bOBl6Y3EuBXEWy3VEA-S2IchqVGmvNGNZo6J9WtSHEcL6ussFWPJoIo2GR4BrgHvZGUvhgbHrKjCPrIAhliH0er3pF5_0UTSqW0Xg_Q2iQpxo9TRn-kHpw",
///       "e": "AQAB"
///     },
///     {
///       "kty": "RSA",
///       "kid": "Sf2lFqwkpX",
///       "use": "sig",
///       "alg": "RS256",
///       "n": "oNe3ZKHU5-fnmbjhCamUpBSyLkR4jbQy-PCZU4cr7tyPcFokyZ1CjSGm44sw3EPONWO6bWgKZYBX2UPv7UM3GBIuB8qBkkN0_vu0Kdr8KUWJ-6m9fnKgceDil4K4TsSS8Owe9qnP9XjjmVRK7cCEjew4GYqQ7gRcHUjIQ-PrKkNBOOijxLlwckeQK2IN9WS_CBXVMleXLutfYAHpwr2KoAmt5BQvPFqBegozHaTc2UvarcUPKMrl-sjY_AXobH7NjqfbBLRJLzS2EzE4y865QiBpwwdhlK4ZQ3g1DCV57BDKvoBX0guCDNSFvoPuIjMmTxZEUbwrJ1CQ4Ib5j4VCkQ",
///       "e": "AQAB"
///     }
///   ]
/// }
///
async fn fetch_apple_jwks() -> Result<Value, Box<dyn std::error::Error>> {
    let jwks_url = "https://appleid.apple.com/auth/keys";
    let res = reqwest::get(jwks_url).await?;
    let jwks = res.json().await?;
    Ok(jwks)
}

fn find_apple_key<'a>(jwks: &'a Value, kid: &str) -> Option<&'a Value> {
    jwks["keys"].as_array()?.iter().find(|key| key["kid"] == kid)
}

pub async fn decode_and_verify_identify_token(token: &str) -> Result<AppleClaims, Box<dyn std::error::Error>> {
    let jwks = fetch_apple_jwks().await?;
    let header = decode_header(token)?;
    let kid = header.kid.ok_or("No key ID found in token header")?;
    let key = find_apple_key(&jwks, &kid).ok_or("Matching Apple key not found")?;
    let n = key["n"].as_str().ok_or("Missing key modulus")?;
    let e = key["e"].as_str().ok_or("Missing key exponent")?;

    let decoding_key = DecodingKey::from_rsa_components(n, e)?;

    let validation = Validation::new(jsonwebtoken::Algorithm::RS256);

    let data = decode::<AppleClaims>(token, &decoding_key, &validation)?;

    Ok(data.claims)
}
