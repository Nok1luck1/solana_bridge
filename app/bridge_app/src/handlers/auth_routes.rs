use crate::db::{database, redis};
use crate::dto::auth::{self, RandomNonceReq};
use crate::errors::{self, FormatError};
use crate::handlers::auth_routes::auth::Claims;
use crate::handlers::auth_routes::auth::RegisterRequest;
use crate::handlers::helpers;
use crate::handlers::helpers::{Network, Role};
use crate::solana;
use crate::state::AppState;
use crate::{eth, state};
use axum::extract::State;
use axum::{http::StatusCode, Json};
use jsonwebtoken::jws::Jws;
use jsonwebtoken::jws::{decode, encode};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use sea_orm::sqlx::types::chrono::Utc;
use std::time::Duration;

async fn verify_user_message(
    Json(_input): Json<RegisterRequest>,
    State(state): State<AppState>,
) -> Json<StatusCode> {
    let mut redis_connection = state::get_redis();
    let connection_type =
        helpers::detect_network(&_input.address).expect(&FormatError::ParseError.to_string());

    let requested_data: (u64, [u8; 32]) =
        redis::get_data_by_address(&mut redis_connection, &_input.address.to_string())
            .await
            .expect(&FormatError::RedisError.to_string());
    //if signed message not same that we give to user then drop
    if _input.message != requested_data.0.to_string()
        && _input.message.as_bytes() != requested_data.1
    {
        return Json(StatusCode::UNAUTHORIZED);
    }
    //verify message corectnes
    match connection_type {
        Network::Ethereum => eth::verify_message(
            requested_data.0.to_string(),
            _input.signature,
            &_input.address,
        )
        .await
        .expect(&FormatError::DecoderError.to_string()),
        Network::Solana => {
            solana::verify_message(requested_data.1, _input.signature, &_input.address)
                .await
                .expect(&FormatError::DecoderError.to_string())
        }
    };
    //check if exists
    let check_id: i64 = if connection_type == Network::Ethereum {
        database::get_user_id_by_address_evm(&state.db, &_input.address)
            .await
            .expect(&FormatError::DBError.to_string())
    } else {
        database::get_user_id_by_address_solana(&state.db, &_input.address)
            .await
            .expect(&FormatError::DBError.to_string())
    };
    //
    let _role: Role = if connection_type == Network::Ethereum {
        eth::check_is_admin(&_input.address)
            .await
            .expect(&FormatError::BlockchainError.to_string());
        Role::Admin
    } else {
        solana::check_exist_admin(&_input.address)
            .await
            .expect(&FormatError::BlockchainError.to_string());
        Role::Admin
    };

    //create if not found
    if connection_type == Network::Ethereum && check_id.eq(&0) {
        let _crt_user = database::create_user(&state.db, _input.address, true)
            .await
            .expect(&FormatError::DBError.to_string());
    } else if connection_type == Network::Solana && check_id.eq(&0) {
        let _crt_user = database::create_user(&state.db, _input.address, false)
            .await
            .expect(&FormatError::DBError.to_string());
    };

    //gen jwt token
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
    )
    .await
    .expect(&FormatError::RedisError.to_string());
    Json(RandomNonceReq {
        address: _input.address,
        rand_nonce: nonce,
        rand_bytes_arr: rand_bytes_arr,
    })
}
fn create_jwt_token(State(_state): State<AppState>, user_id: i64, role: Role) -> Jws<Claims> {
    let expiration = Utc::now() + Duration::from_hours(_state.auth.jwt_expiry_hours as u64);
    let claims = Claims {
        sub: user_id,
        exp: expiration.timestamp() as usize,
        role: role,
    };
    encode(
        &Header::default(),
        Some(&claims),
        &EncodingKey::from_secret(_state.auth.jwt_secret.as_bytes()),
    )
    .expect(&FormatError::JWTokenError.to_string())
}
fn verify_jwt_token(
    State(_state): State<AppState>,
    token: &Jws<Claims>,
) -> std::result::Result<Claims, StatusCode> {
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(_state.auth.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| StatusCode::UNAUTHORIZED)
}
