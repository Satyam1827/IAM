use std::sync::Arc;

use axum::{
    extract::State,
    http::{
        HeaderMap,
        StatusCode,
    },
    middleware::Next,
    response::Response,
};

use crate::{
    services::api_key_auth,
    state::AppState,
};


// x-api-key header
// ↓
// verify hash
// ↓
// allow request
pub async fn auth(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    request: axum::extract::Request,
    next: Next,
) -> Result<Response, StatusCode>
{
    let Some(raw_key) =
        headers.get("x-api-key")
    else {
        return Err(
            StatusCode::UNAUTHORIZED,
        );
    };

    let Ok(raw_key) =
        raw_key.to_str()
    else {
        return Err(
            StatusCode::UNAUTHORIZED,
        );
    };

    let authenticated =
        api_key_auth::authenticate(
            state,
            raw_key,
        )
        .await;

    if authenticated.is_none() {
        return Err(
            StatusCode::UNAUTHORIZED,
        );
    }

    Ok(
        next.run(request)
            .await,
    )
}