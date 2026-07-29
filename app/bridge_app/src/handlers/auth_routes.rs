use crate::db::{database, redis};
use crate::dto::auth::{self, AuthConfig, RandomNonceReq};
use crate::errors::{self, FormatError};
use crate::handlers::auth_routes::auth::Claims;
use crate::handlers::auth_routes::auth::RegisterRequest;
use crate::handlers::helpers;
use crate::handlers::helpers::{Network, Role};
use crate::solana;
use crate::state::AppState;
use crate::{eth, state};
use ::redis::aio::ConnectionManager;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{http::StatusCode, Json};
use jsonwebtoken::jws::Jws;
use jsonwebtoken::jws::{decode, encode};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use sea_orm::sqlx::types::chrono::Utc;
use std::time::Duration;
use uuid::Uuid;

pub async fn login_user(
    State(state): State<AppState>,
    Json(input): Json<RegisterRequest>,
) -> Result<StatusCode, StatusCode> {
    let mut reds = state::get_redis();
    let (network, role) = verify_wallet(&mut reds, &input).await?;

    let user_id = if network == Network::Ethereum {
        database::get_user_id_by_address_evm(&state.db, &input.address)
            .await
            .map_err(|err| {
                tracing::error!("{err:?}");
                FormatError::DBError.into_response().status()
            })?
    } else {
        database::get_user_id_by_address_solana(&state.db, &input.address)
            .await
            .map_err(|err| {
                tracing::error!("{err:?}");
                FormatError::DBError.into_response().status()
            })?
    };

    if user_id == 0 {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let expiration = Utc::now() + Duration::from_hours(state.auth.jwt_expiry_hours as u64);
    let token = create_jwt_token(&state.auth, user_id, role, expiration.timestamp() as usize);
    let mut redis_c = state::get_redis();
    redis::save_session(
        &mut redis_c,
        &token,
        user_id,
        expiration.timestamp_millis().try_into().unwrap(),
    );

    Ok(StatusCode::OK)
}
pub async fn register_user(
    State(state): State<AppState>,
    Json(input): Json<RegisterRequest>,
) -> Result<StatusCode, StatusCode> {
    let mut reds = state::get_redis();
    let (network, role) = verify_wallet(&mut reds, &input).await?;

    let user_id = if network == Network::Ethereum {
        database::get_user_id_by_address_evm(&state.db, &input.address)
            .await
            .map_err(|err| {
                tracing::error!("{err:?}");
                FormatError::DBError.into_response().status()
            })?
    } else {
        database::get_user_id_by_address_solana(&state.db, &input.address)
            .await
            .map_err(|err| {
                tracing::error!("{err:?}");
                FormatError::DBError.into_response().status()
            })?
    };
    if user_id != 0 {
        return Err(StatusCode::CONFLICT);
    }
    let _new_user = database::create_user(
        &state.db,
        input.address.clone(),
        network == Network::Ethereum,
    )
    .await
    .map_err(|err| {
        tracing::error!("{err:?}");
        FormatError::DBError.into_response().status()
    })?;
    let expiration = Utc::now() + Duration::from_hours(state.auth.jwt_expiry_hours as u64);
    let token = create_jwt_token(&state.auth, user_id, role, expiration.timestamp() as usize);
    let mut redis_c = state::get_redis();
    redis::save_session(
        &mut redis_c,
        &token,
        user_id,
        expiration.timestamp_millis().try_into().unwrap(),
    );

    Ok(StatusCode::CREATED)
}

async fn verify_wallet(
    redis: &mut ConnectionManager,
    input: &RegisterRequest,
) -> Result<(Network, Role), StatusCode> {
    let connection_type =
        helpers::detect_network(&input.address).expect(&FormatError::AppError.to_string());
    let requested_data = redis::get_data_by_address(redis, &input.address)
        .await
        .map_err(|err| {
            tracing::error!("{err:?}");
            FormatError::RedisError.into_response().status()
        })?;

    if input.message != requested_data.0.to_string() && input.message.as_bytes() != requested_data.1
    {
        return Err(StatusCode::UNAUTHORIZED);
    }
    match connection_type {
        Network::Ethereum => {
            eth::verify_message(
                requested_data.0.to_string(),
                input.signature.clone(),
                &input.address,
            )
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;
        }
        Network::Solana => {
            solana::verify_message(requested_data.1, input.signature.clone(), &input.address)
                .await
                .map_err(|_| StatusCode::UNAUTHORIZED)?;
        }
    }

    let role = match connection_type {
        Network::Ethereum => {
            if eth::check_is_admin(&input.address)
                .await
                .map_err(|_| StatusCode::UNAUTHORIZED)?
            {
                Role::Admin
            } else {
                Role::User
            }
        }
        Network::Solana => {
            if solana::check_exist_admin(&input.address)
                .await
                .map_err(|_| StatusCode::UNAUTHORIZED)?
            {
                Role::Admin
            } else {
                Role::User
            }
        }
    };

    Ok((connection_type, role))
}
async fn generate_nonce_bytes(
    State(_state): State<AppState>,
    Json(_input): Json<RandomNonceReq>,
) -> Json<RandomNonceReq> {
    let nonce = eth::get_address_nonce(_input.address.clone())
        .await
        .map_err(|err| {
            tracing::error!("{err:?}");
            FormatError::BlockchainError
        })
        .unwrap();
    let rand_bytes_arr: [u8; 32] = rand::random();
    let mut redis_connection = state::get_redis();
    let _ = redis::save_registration_data(
        &mut redis_connection,
        &_input.address,
        &nonce,
        &rand_bytes_arr,
    )
    .await
    .map_err(|err| {
        tracing::error!("{err:?}");
        FormatError::RedisError
    });
    Json(RandomNonceReq {
        address: _input.address,
        rand_nonce: nonce,
        rand_bytes_arr: rand_bytes_arr,
    })
}
fn create_jwt_token(config: &AuthConfig, user_id: i64, role: Role, expiration: usize) -> String {
    let jti = Uuid::new_v4().to_string();
    let claims = Claims {
        sub: user_id,
        exp: expiration,
        role,
        jti,
    };
    let jws = encode(
        &Header::default(),
        Some(&claims),
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(|err| {
        tracing::error!("{err:?}");
        FormatError::JWTokenError
    });

    serde_json::to_string(&jws).expect(&FormatError::JWTokenError.to_string())
}

pub fn verify_jwt_token(
    config: &AuthConfig,
    token: &str,
) -> std::result::Result<Claims, StatusCode> {
    let jws: Jws<Claims> = serde_json::from_str(token).map_err(|_| StatusCode::UNAUTHORIZED)?;

    decode::<Claims>(
        &jws,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| StatusCode::UNAUTHORIZED)
}
