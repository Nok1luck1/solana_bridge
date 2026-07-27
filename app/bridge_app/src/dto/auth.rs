use serde::{Deserialize, Serialize};

use crate::handlers::helpers::Role;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub message: String,
    pub signature: String,
    pub address: String,
}
#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: i64,
    pub role: Role,
    pub jti: String,
}
#[derive(Deserialize, Serialize)]
pub struct RandomNonceReq {
    pub address: String,
    pub rand_nonce: u64,
    pub rand_bytes_arr: [u8; 32],
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,   // user id
    pub exp: usize, // expiration timestamp
    pub role: Role,
    pub jti: String,
}
#[derive(Clone, Debug)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiry_hours: i64,
}
