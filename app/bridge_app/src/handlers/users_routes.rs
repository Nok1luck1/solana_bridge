use axum::http::StatusCode;
use axum::Json;
use bridge::Order;

use crate::db::database;
use crate::dto::users::{CreateUser, GetUser};
use crate::types::OrderFormatter;

pub async fn orders_sol() -> Json<Vec<Order>> {
    let orders = Vec::new();
    return Json(orders);
}
pub async fn orders_evm() -> Json<Vec<Order>> {
    let orders = Vec::new();
    return Json(orders);
}
pub async fn get_user_orders(
    Json(payload): Json<GetUser>,
) -> (StatusCode, Json<Vec<OrderFormatter>>) {
    let user_orders = database::get_users_made_orders(
        payload.address_evm.to_string(),
        payload.maker,
        payload.is_evm,
        payload.limit,
        payload.offset,
    )
    .await
    .unwrap();
    return (StatusCode::OK, Json(user_orders));
}
pub async fn create_user(Json(_payload): Json<CreateUser>) -> StatusCode {
    return StatusCode::CREATED;
}
