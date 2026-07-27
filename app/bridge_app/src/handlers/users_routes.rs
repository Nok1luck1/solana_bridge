use crate::db::database;
use crate::dto::users::{CreateUser, GetUserOrders};
use crate::errors::FormatError;
use crate::handlers::helpers::Network;
use crate::state::AppState;
use crate::types::OrderFormatter;
use axum::Json;
use axum::{extract::State, http::StatusCode};
pub async fn get_all_orders_sol(State(pool): State<AppState>) -> Json<Vec<OrderFormatter>> {
    Json(
        database::get_all_solana_order(&pool.db, 50, 50)
            .await
            .expect(&FormatError::DBError.to_string()),
    )
}
pub async fn get_all_orders_evm(State(pool): State<AppState>) -> Json<Vec<OrderFormatter>> {
    Json(
        database::get_all_evm_order(&pool.db, 50, 50)
            .await
            .expect(&FormatError::DBError.to_string()),
    )
}
pub async fn get_user_orders(
    State(pool): State<AppState>,
    Json(payload): Json<GetUserOrders>,
) -> (StatusCode, Json<Vec<OrderFormatter>>) {
    let is_evm = if payload.network == Network::Ethereum {
        true
    } else {
        false
    };
    let user_orders = database::get_users_made_orders(
        &pool.db,
        &payload.address_evm.to_string(),
        payload.maker,
        is_evm,
        payload.limit,
        payload.offset,
    )
    .await
    .expect(&FormatError::DBError.to_string());
    return (StatusCode::OK, Json(user_orders));
}
pub async fn create_user(State(_pool): State<AppState>, _payload: CreateUser) -> StatusCode {
    return StatusCode::CREATED;
}
