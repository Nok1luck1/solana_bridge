pub mod bridge;
pub mod db;
pub mod dto;
pub mod entity;
pub mod errors;
pub mod eth;
pub mod handlers;
pub mod local;
pub mod solana;
pub mod state;
pub mod types;
use crate::types::OrderFormatter;
use crate::solana::utils;
use dotenv::dotenv;
use entity::orders;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tracing::Level;
use tracing_subscriber;
type AppState = Arc<DatabaseConnection>;
#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //  let client = redis::Client::open("redis://127.0.0.1/")?;
    // let mut con = client.get_connection()?;
    // // `set` returns a `()`, so we don't need to specify the return type manually unlike in the previous example.
    // con.set("my_key", 42)?;
    // // `get_int` returns Option<isize>, as the key may not be found.
    // con.get_int("my_key").unwrap();
    dotenv().ok();
    state::init_db_pool().await;
    state::init_redis().await;
    //local::run_evm_local_validator();
    //local::run_solana_local_validator();
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    //let _ = bridge::run_bridge().await;

    let app = axum::Router::new()
        // .route(
        //     "/orders/solana",
        //     get(handlers::users_routes::get_all_orders_sol),
        // )
        // .route(
        //     "/orders/evm",
        //     get(handlers::users_routes::get_all_orders_evm),
        // )
        // .route(
        //     "/user/orders",
        //     post(handlers::users_routes::get_user_orders),
        // )
        // .route("/user/create", post(handlers::users_routes::create_user))
        // .with_state(pool);
;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect(&errors::FormatError::AppError.to_string());
    axum::serve(listener, app).await?;
    Ok(())
}
