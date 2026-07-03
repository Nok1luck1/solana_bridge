use axum::http::StatusCode;
use axum::Json;
use bridge::Order;

use crate::db::database;
use crate::dto::users::{CreateUser, GetUserOrders};
use crate::types::OrderFormatter;

pub async fn get_all_orders_sol() -> Json<Vec<OrderFormatter>> {
    Json(database::get_all_solana_order(50, 50).await.unwrap())
}
pub async fn get_all_orders_evm() -> Json<Vec<OrderFormatter>> {
    Json(database::get_all_evm_order(50, 50).await.unwrap())
}
pub async fn get_user_orders(
    Json(payload): Json<GetUserOrders>,
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
    let _crt_user = database::create_user(
        _payload.address_evm.to_string(),
        _payload.address_sol.to_string(),
    )
    .await
    .unwrap();
    return StatusCode::CREATED;
}
