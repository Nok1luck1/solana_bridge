use crate::db::database;
use crate::dto::users::{CreateUser, GetUserOrders};
use crate::handlers::helpers::Network;
use crate::state::SharedAppState;
use crate::types::OrderFormatter;
use axum::Json;
use axum::{extract::State, http::StatusCode};
pub async fn get_all_orders_sol(State(pool): State<SharedAppState>) -> Result<Json<Vec<OrderFormatter>>, StatusCode> {
    let orders = database::get_all_solana_order(&pool.db, 50, 50)
        .await
        .map_err(|err| {
            tracing::error!("{err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(orders))
}
pub async fn get_all_orders_evm(State(pool): State<SharedAppState>) -> Result<Json<Vec<OrderFormatter>>, StatusCode> {
    let orders = database::get_all_evm_order(&pool.db, 50, 50)
        .await
        .map_err(|err| {
            tracing::error!("{err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(orders))
}
pub async fn get_user_orders(
    State(pool): State<SharedAppState>,
    Json(payload): Json<GetUserOrders>,
) -> Result<(StatusCode, Json<Vec<OrderFormatter>>), StatusCode> {
    let is_evm = payload.network == Network::Ethereum;
    let user_orders = database::get_users_made_orders(
        &pool.db,
        &payload.address_evm.to_string(),
        payload.maker,
        is_evm,
        payload.limit,
        payload.offset,
    )
    .await
    .map_err(|err| {
        tracing::error!("{err:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok((StatusCode::OK, Json(user_orders)))
}
pub async fn create_user(State(_pool): State<SharedAppState>, _payload: CreateUser) -> StatusCode {
    StatusCode::CREATED
}
