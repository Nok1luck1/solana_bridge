use crate::db::database;
use crate::dto::admin::{BlockUser, GetOrder, GetReservesEvm, GetReservesSol};
use crate::dto::users::User;
use crate::errors::FormatError;
use crate::eth;
use crate::solana;
use crate::types::OrderFormatter;
use alloy::primitives::Address;
use anchor_lang::prelude::Pubkey;
use axum::{http::StatusCode, Json};

pub async fn force_execute_evm() {}
pub async fn force_execute_sol() {}

pub async fn get_spicific_order(
    Json(payload): Json<GetOrder>,
) -> (StatusCode, Json<OrderFormatter>) {
    let specific_order = database::get_spicific_order(payload.order_id as i32)
        .await
        .expect("Cant get Specific order");
    let order = OrderFormatter::from_db_to_formatet(specific_order);
    (StatusCode::OK, Json(order))
}
pub async fn get_reserves_evm(Json(payload): Json<GetReservesEvm>) -> (StatusCode, i64) {
    let token_reserves = eth::check_balance(payload.address_asset)
        .await
        .expect(&FormatError::BlockchainError.to_string())
        .to::<i64>();
    (StatusCode::FOUND, token_reserves)
}
pub async fn get_reserves_sol(Json(payload): Json<GetReservesSol>) -> (StatusCode, i64) {
    let token_mint_reserves = solana::get_vault_balance(payload.mint)
        .await
        .expect(&FormatError::BlockchainError.to_string());
    (StatusCode::FOUND, token_mint_reserves as i64)
}

pub async fn block_user(Json(payload): Json<BlockUser>) -> (StatusCode, Json<User>) {
    let user_id: i64 = if payload.is_evm {
        database::get_user_id_by_address_evm(payload.address)
            .await
            .expect(&FormatError::BlockchainError.to_string())
    } else {
        database::get_user_id_by_address_solana(payload.address)
            .await
            .expect(&FormatError::BlockchainError.to_string())
    };
    let result = database::block_user(user_id as u64)
        .await
        .expect(&FormatError::DBError.to_string());
    let blocked_user = User {
        id: user_id,
        address_sol: result.address_solana,
        address_evm: result.address_evm,
        blocked: true,
    };
    (StatusCode::ACCEPTED, Json(blocked_user))
}
