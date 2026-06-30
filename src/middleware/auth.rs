use std::sync::Arc;

use axum::{
    extract::{Request, State, FromRequestParts},
    http::{
        request::Parts,
        header::AUTHORIZATION,
        StatusCode,
    },
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::{
    auth::{
        extractor::CurrentUser,
        jwt,
    },
    state::AppState,
};

impl<S> FromRequestParts<S>
for CurrentUser
where
    S: Send + Sync,
{
    type Rejection =
        StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<CurrentUser>()
            .cloned()
            .ok_or(
                StatusCode::UNAUTHORIZED,
            )
    }
}

pub async fn auth(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth =
        request
            .headers()
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or(
                StatusCode::UNAUTHORIZED,
            )?;

    let token =
        auth
            .strip_prefix("Bearer ")
            .ok_or(
                StatusCode::UNAUTHORIZED,
            )?;

    let claims =
        jwt::verify_access_token(
            token,
            &state.config.jwt_secret,
        )
        .map_err(|_| {
            StatusCode::UNAUTHORIZED
        })?;

    let current_user = CurrentUser {
        user_id:
            Uuid::parse_str(
                &claims.sub,
            )
            .map_err(|_| {
                StatusCode::UNAUTHORIZED
            })?,

        session_id:
            Uuid::parse_str(
                &claims.session_id,
            )
            .map_err(|_| {
                StatusCode::UNAUTHORIZED
            })?,
    };

    request
        .extensions_mut()
        .insert(current_user);

    Ok(next.run(request).await)
}