use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};

use crate::{
    db::redis,
    dto::auth::CurrentUser,
    errors::FormatError,
    handlers::auth_routes::verify_jwt_token,
    state::AppState,
};

pub async fn auth_middleware(
    State(config): State<AppState>,
    mut reqv: Request,
    next: Next,
) -> Result<Response, FormatError> {
    let auth = reqv
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or(FormatError::UnauthorizedError)?
        .to_str()
        .map_err(|_| FormatError::UnauthorizedError)?;
    let token = auth
        .strip_prefix("Bearer ")
        .ok_or(FormatError::UnauthorizedError)?;
    let claims = verify_jwt_token(&config.auth, token).map_err(|_| FormatError::UnauthorizedError)?;
    let mut redis_con = crate::state::get_redis();
    let exists = redis::get_session(&mut redis_con, &claims.jti)
        .await
        .map_err(|_| FormatError::RedisError)?;
    if exists.is_none() || exists == Some(0) {
        return Err(FormatError::JWTokenError);
    }
    reqv.extensions_mut().insert(CurrentUser {
        id: claims.sub,
        role: claims.role,
        jti: claims.jti,
    });
    Ok(next.run(reqv).await)
}
