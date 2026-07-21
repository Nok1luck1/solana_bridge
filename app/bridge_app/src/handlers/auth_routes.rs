use crate::db::redis;
use crate::dto::auth::{self, RandomNonceReq};
use crate::errors::{self, FormatError};
use crate::handlers::auth_routes::auth::RegisterRequest;
use crate::solana;
use crate::state::AppState;
use crate::{eth, state};
use axum::extract::State;
use axum::{http::StatusCode, Json};

async fn verify_user_message(
    Json(_input): Json<RegisterRequest>,
    State(state): State<AppState>,
) -> Json<StatusCode> {
    let mut redis_connection = state::get_redis();
    let requested_data =
        redis::get_data_by_address(&mut redis_connection, &_input.address.to_string())
            .await
            .expect(&FormatError::RedisError.to_string());
    let is_valid = match _input.address.len() {
        42 => eth::verify_message(_input.message, _input.signature, _input.address)
            .await
            .expect(&FormatError::DecoderError.to_string()),
        44 => solana::verify_message(_input.message, _input.signature, _input.address)
            .await
            .expect(&FormatError::DecoderError.to_string()),
        _ => false,
    };
    //geenrate jwt

    Json(StatusCode::ACCEPTED)
}

async fn generate_nonce_bytes(
    State(_state): State<AppState>,
    Json(_input): Json<RandomNonceReq>,
) -> Json<RandomNonceReq> {
    let nonce = eth::get_address_nonce(_input.address.clone())
        .await
        .expect(&errors::FormatError::BlockchainError.to_string());
    let rand_bytes_arr: [u8; 32] = rand::random();
    let mut redis_connection = state::get_redis();
    redis::save_registration_data(
        &mut redis_connection,
        &_input.address,
        &nonce,
        &rand_bytes_arr,
    );
    Json(RandomNonceReq {
        address: _input.address,
        rand_nonce: nonce,
        rand_bytes_arr: rand_bytes_arr,
    })
}
