use crate::middleware::admin_middleware::admin_middleware;
use crate::middleware::auth_middleware::auth_middleware;
use crate::state::AppState;
use axum::{middleware::from_fn_with_state, Router};
pub mod admin_routes;
pub mod auth_routes;
pub mod user_routes;
pub fn all_routes(state: AppState) -> Router {
    Router::new()
        .nest("/auth", auth_routes::auth_routes())
        .nest(
            "/user",
            user_routes::user_routes()
                .route_layer(from_fn_with_state(state.clone(), auth_middleware)),
        )
        .nest(
            "/admin",
            admin_routes::admin_routes()
                .route_layer(from_fn_with_state(state.clone(), auth_middleware))
                .route_layer(from_fn_with_state(state.clone(), admin_middleware)),
        )
        .with_state(state)
}
