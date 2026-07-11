use crate::dto::auth;
use axum::{http::StatusCode, Json};
async fn register(Json(input): Json<RegisterRequest>) -> Json<StatusCode> {
    Json(StatusCode::ACCEPTED)
}
async fn login(Json(input): Json<LoginRequest>) -> Json<StatusCode> {
    Json(StatusCode::ACCEPTED)
}
