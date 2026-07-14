use crate::db::database;
use crate::dto::users::{CreateUser, GetUserOrders};
use crate::errors::FormatError;
use crate::types::OrderFormatter;
use axum::Json;
use axum::{extract::State, http::StatusCode};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

type AppState = Arc<DatabaseConnection>;

pub async fn get_all_orders_sol<S>(State(pool): State<AppState>) -> Json<Vec<OrderFormatter>> {
    Json(
        database::get_all_solana_order(&pool, 50, 50)
            .await
            .expect(&FormatError::DBError.to_string()),
    )
}
pub async fn get_all_orders_evm<S>(
    State(pool): State<AppState>,
) -> Json<(Vec<OrderFormatter>, StatusCode)> {
    Json((
        database::get_all_evm_order(&pool, 50, 50)
            .await
            .expect(&FormatError::DBError.to_string()),
        StatusCode::OK,
    ))
}
pub async fn get_user_orders<S>(
    State(pool): State<AppState>,
    Json(payload): Json<GetUserOrders>,
) -> (StatusCode, Json<Vec<OrderFormatter>>) {
    let user_orders = database::get_users_made_orders(
        &pool,
        payload.address_evm.to_string(),
        payload.maker,
        payload.is_evm,
        payload.limit,
        payload.offset,
    )
    .await
    .expect(&FormatError::DBError.to_string());
    return (StatusCode::OK, Json(user_orders));
}
pub async fn create_user<S>(
    State(pool): State<AppState>,
    Json(_payload): Json<CreateUser>,
) -> StatusCode {
    let _crt_user = database::create_user(&pool, _payload.pub_key.to_string(), _payload.is_evm)
        .await
        .expect(&FormatError::DBError.to_string());

    return StatusCode::CREATED;
}
