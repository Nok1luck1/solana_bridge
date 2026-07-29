use crate::routes::auth_routes::auth_routes::login_user;
use crate::routes::auth_routes::auth_routes::register_user;
use crate::{handlers::auth_routes, state::AppState};
use axum::routing::post;
use axum::Router;
pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
}
