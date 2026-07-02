use std::sync::Arc;

use axum::{
    extract::State,
    Json,
};

use crate::{
    dto::auth::{
        LoginRequest,
        RefreshRequest,
        RegisterRequest,
        TokenResponse,
    },
    errors::AppError,
    services::auth,
    state::AppState,
};

use crate::auth::extractor::CurrentUser;

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> Result<(), AppError> {
    auth::register(state, req).await
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    let tokens =
        auth::login(state, req).await?;

    Ok(Json(tokens))
}

pub async fn refresh(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RefreshRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    let response =
        auth::refresh(
            state,
            req.session_id,
            req.refresh_token,
        )
        .await?;

    Ok(Json(response))
}

pub async fn logout(
    current_user: CurrentUser,
    State(state): State<Arc<AppState>>,
) -> Result<(), AppError>
{
    auth::logout(
        state,
        current_user.session_id,
    )
    .await
}
