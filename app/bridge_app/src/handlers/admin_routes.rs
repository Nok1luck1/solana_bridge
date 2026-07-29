use crate::db::database;
use crate::dto::admin::{BlockUser, GetOrder, GetReservesEvm, GetReservesSol};
use crate::dto::users::User;
use crate::errors::FormatError;
use crate::eth;
use crate::solana;
use crate::state::AppState;
use crate::types::OrderFormatter;

use axum::{extract::State, http::StatusCode, Json};

pub async fn force_execute_evm() {}

pub async fn force_execute_sol() {}

pub async fn get_specific_order(
    State(pool): State<AppState>,
    Json(payload): Json<GetOrder>,
) -> Result<(StatusCode, Json<OrderFormatter>), FormatError> {
    let specific_order = database::get_spicific_order(&pool.db, payload.order_id)
        .await
        .map_err(|err| {
            tracing::error!("{err:?}");
            FormatError::DBError
        })?;

    let order = OrderFormatter::from_db_to_formatet(specific_order);

    Ok((StatusCode::OK, Json(order)))
}

pub async fn get_reserves_evm(
    Json(payload): Json<GetReservesEvm>,
) -> Result<(StatusCode, Json<i64>), FormatError> {
    let token_reserves = eth::check_balance(payload.address_asset)
        .await
        .map_err(|err| {
            tracing::error!("{err:?}");
            FormatError::BlockchainError
        })?
        .to::<i64>();

    Ok((StatusCode::OK, Json(token_reserves)))
}

pub async fn get_reserves_sol(
    Json(payload): Json<GetReservesSol>,
) -> Result<(StatusCode, Json<i64>), FormatError> {
    let token_mint_reserves = solana::get_vault_balance(payload.mint)
        .await
        .map_err(|err| {
            tracing::error!("{err:?}");
            FormatError::BlockchainError
        })?;

    Ok((StatusCode::OK, Json(token_mint_reserves as i64)))
}

pub async fn block_user(
    State(pool): State<AppState>,
    Json(payload): Json<BlockUser>,
) -> Result<(StatusCode, Json<User>), FormatError> {
    let user_id: i64 = if payload.is_evm {
        database::get_user_id_by_address_evm(&pool.db, &payload.address)
            .await
            .map_err(|err| {
                tracing::error!("{err:?}");
                FormatError::DBError
            })?
    } else {
        database::get_user_id_by_address_solana(&pool.db, &payload.address)
            .await
            .map_err(|err| {
                tracing::error!("{err:?}");
                FormatError::DBError
            })?
    };

    let result = database::block_user(&pool.db, user_id as u64)
        .await
        .map_err(|err| {
            tracing::error!("{err:?}");
            FormatError::DBError
        })?;

    let blocked_user = User {
        id: user_id,
        address_sol: result.address_solana,
        address_evm: result.address_evm,
        blocked: true,
    };

    Ok((StatusCode::ACCEPTED, Json(blocked_user)))
}
