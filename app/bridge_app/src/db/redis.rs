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
