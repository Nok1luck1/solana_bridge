use axum::http::StatusCode;
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, RedisError};

pub async fn save_registration_data(
    redis: &mut ConnectionManager,
    address: &str,
    nonce: &u64,
    rand_bytes: &[u8; 32],
) -> Result<(), RedisError> {
    redis
        .hset::<_, _, _, ()>(format!("register:{address}"), "nonce", nonce)
        .await?;

    redis
        .hset::<_, _, _, ()>(format!("register:{address}"), "bytes", rand_bytes)
        .await?;
    Ok(())
}
pub async fn get_data_by_address(
    _redis: &mut ConnectionManager,
    _address: &str,
) -> Result<(u64, [u8; 32]), RedisError> {
    let nonce: u64 = _redis.hget(format!("register:{_address}"), "nonce").await?;
    let rand_bytes: [u8; 32] = _redis.hget(format!("register:{_address}"), "bytes").await?;
    Ok((nonce, rand_bytes))
}
pub async fn save_session(
    redis: &mut ConnectionManager,
    jti: &str,
    user_id: i64,
    ttl_secs: u64,
) -> Result<(), RedisError> {
    redis
        .set_ex::<_, _, ()>(format!("session:{jti}"), user_id, ttl_secs)
        .await?;

    Ok(())
}

pub async fn get_session(
    redis: &mut ConnectionManager,
    token: &str,
) -> Result<Option<i64>, RedisError> {
    redis
        .get::<_, Option<i64>>(format!("session:{token}"))
        .await
}
pub async fn delete_session(redis: &mut ConnectionManager, token: &str) -> Result<(), RedisError> {
    redis.del::<_, ()>(format!("session:{token}")).await?;

    Ok(())
}
pub async fn save_jti(
    redis: &mut ConnectionManager,
    jti: &str,
    user_id: i64,
    ttl_secs: u64,
) -> Result<(), RedisError> {
    redis
        .set_ex::<_, _, ()>(format!("session:{jti}"), user_id, ttl_secs)
        .await?;

    Ok(())
}
pub async fn verify_jti(redis: &mut ConnectionManager, jti: &str) -> Result<bool, RedisError> {
    let exists: bool = redis.exists(format!("session:{jti}")).await?;
    Ok(exists)
}
