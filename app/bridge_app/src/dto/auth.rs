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
pub struct LoginRequest {
    pub_key: String,
    is_evm: bool,
}
#[derive(Serialize)]
pub struct LoginResponce {
    token: String,
    expiration: i64,
}
#[derive(Deserialize)]
pub struct RegisterRequest {
    address: String,
}
#[derive(Debug, Clone)]
pub struct CurrentUser {
    id: i64,
    role: RoleType,
}
