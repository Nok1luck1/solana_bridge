


// static REDIS_POOL: OnceCell<Arc<Client>> = OnceCell::const_new();

// pub async fn init_redis() {
//     let client = redis::Client::open(std::env::var("REDIS_URL").expect("REDIX_URL not set"))
//         .expect(&errors::FormatError::InitError.to_string());
//     REDIS_POOL
//         .set(Arc::new(client))
//         .expect(&errors::FormatError::InitError.to_string());
// }
// pub fn get_pool() -> Arc<Connection> {
//     return REDIS_POOL
//         .get()
//         .expect(&errors::FormatError::RedisError.to_string())
//         .clone()
//         .get_connection()
//         .expect(&errors::FormatError::RedisError.to_string())
//         .into();
// }
