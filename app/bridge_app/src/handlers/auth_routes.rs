use crate::dto::auth;
use crate::handlers::auth_routes::auth::{LoginRequest, RegisterRequest};
use axum::{http::StatusCode, Json};
async fn register(Json(_input): Json<RegisterRequest>) -> Json<StatusCode> {
    Json(StatusCode::ACCEPTED)
}
async fn login(Json(_input): Json<LoginRequest>) -> Json<StatusCode> {
    Json(StatusCode::ACCEPTED)
}
