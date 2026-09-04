use crate::handlers::admin_routes::{
    block_user, get_reserves_evm, get_reserves_sol, get_specific_order,
};
use crate::state::SharedAppState;
use axum::routing::post;
use axum::Router;

pub fn admin_routes() -> Router<SharedAppState> {
    Router::new()
        .route("/get_reserves_evm", post(get_reserves_evm))
        .route("/get_reserves_sol", post(get_reserves_sol))
        .route("/get_order", post(get_specific_order))
        .route("/block_user", post(block_user))
}
