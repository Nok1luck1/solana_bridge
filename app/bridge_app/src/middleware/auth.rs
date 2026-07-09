use axum::http::StatusCode;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use jsonwebtoken::{decode, encode};
use sea_orm::sqlx::types::chrono::Utc;
use serde::{Deserialize, Serialize};
use std::time::Duration;
#[derive(Clone)]
struct AuthConfig {
    jwt_secret: String,
    jwr_expiry_time: i64,
    jwt_expiry_hours: i64,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    wallet: String,
    is_evm: bool,
    password: String,
}
#[derive(Serialize)]
pub struct LoginResponce {
    token: String,
    expires_in: i64,
}
#[derive(Deserialize)]
pub struct RegisterRequest {
    wallet_evm: String,
    wallet_sol: String,
    password: String,
}
#[derive(Serialize, Deserialize)]
pub struct Claim {
    sub: String,
    exp: u64,
    role: String,
}
#[derive(Debug, Clone)]
struct CurrentUser {
    id: String,
    role: String,
}
fn verify_token(config: &AuthConfig, token: &str) -> Result<Claim, StatusCode> {
    decode::<Claim>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| StatusCode::UNAUTHORIZED)
}
fn create_token(config: &AuthConfig, user_id: &str, role: &str) -> Result<String, StatusCode> {
    let expire = Utc::now() + Duration::from_hours(config.jwt_expiry_hours as u64);
    let claims = Claim {
        sub: user_id.to_string(),
        exp: expire.timestamp() as u64,
        role: role.to_string(),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
