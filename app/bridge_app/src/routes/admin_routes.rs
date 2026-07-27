use crate::handlers::admin_routes::{
    block_user, get_reserves_evm, get_reserves_sol, get_specific_order,
};
use crate::AppState;
use axum::routing::{delete, get, post};
use axum::Router;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/get_reserves_evm", post(get_reserves_evm))
        .route("/get_reserves_sol", post(get_reserves_sol))
        .route("/get_order", post(get_specific_order))
        .route("/block_user", post(block_user))
}
