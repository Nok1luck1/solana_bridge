use crate::{
    handlers::users_routes::{get_all_orders_evm, get_all_orders_sol, get_user_orders},
    state::SharedAppState,
};
use axum::routing::{get, post};
use axum::Router;

pub fn user_routes() -> Router<SharedAppState> {
    Router::new()
        .route("/user/orders", post(get_user_orders))
        .route("/solana_orders", get(get_all_orders_sol))
        .route("/evm_orders", get(get_all_orders_evm))
}
