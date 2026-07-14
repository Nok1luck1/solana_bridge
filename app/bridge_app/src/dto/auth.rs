// use serde::{Deserialize, Serialize};
// pub enum RoleType {
//     Admin,
//     User,
// }
// #[derive(Clone)]
// pub struct AuthConfig {
//     jwt_secret: String,
//     jwt_expiration: i64,
// }
// #[derive(Deserialize)]
// struct LoginRequest {
//     pub_key: String,
//     is_evm: bool,
// }
// #[derive(Serialize)]
// struct LoginResponce {
//     token: String,
//     expiration: i64,
// }
// #[derive(Deserialize)]
// struct RegisterRequest {
//     address: String,
// }
// #[derive(Debug, Clone)]
// struct CurrentUser {
//     id: i64,
//     role: RoleType,
// }
