use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};

use crate::handlers::helpers::Role;
use crate::{
    dto::auth::CurrentUser,
    errors::FormatError,
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
