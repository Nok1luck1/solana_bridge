use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::handlers::helpers::Role;
use crate::{
    db::redis,
    dto::auth::CurrentUser,
    errors::FormatError,
    handlers::auth_routes::verify_jwt_token,
    state::{self, AppState},
};

pub async fn admin_middleware(req: Request, next: Next) -> Result<Response, FormatError> {
    let user = req
        .extensions()
        .get::<CurrentUser>()
        .ok_or(FormatError::UnauthorizedError)?;

    if user.role != Role::Admin {
        return Err(FormatError::UnauthorizedError);
    }

    Ok(next.run(req).await)
}
