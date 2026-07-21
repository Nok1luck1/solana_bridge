use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub enum RoleType {
    Admin,
    User,
}
#[derive(Clone)]
pub struct AuthConfig {
    jwt_secret: String,
    jwt_expiration: i64,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub message: String,
    pub signature: String,
    pub address: String,
}
#[derive(Debug, Clone)]
pub struct CurrentUser {
    id: i64,
    role: RoleType,
}
#[derive(Deserialize, Serialize)]
pub struct RandomNonceReq {
    pub address: String,
    pub rand_nonce: u64,
    pub rand_bytes_arr: [u8; 32],
}
