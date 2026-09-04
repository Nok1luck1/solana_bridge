
use crate::state::SharedAppState;
use axum::routing::post;
use axum::Router;

pub fn estate_routes() -> Router<SharedAppState> {
    Router::new()
        .route("/real_estate", get(get_real_estate))
        
}
