use crate::routes::auth_routes::auth_routes::{generate_nonce_bytes, login_user, register_user};

use crate::{handlers::auth_routes, state::AppState};
use axum::routing::post;
use axum::Router;
pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .route("/generate_verify", post(generate_nonce_bytes))
}
