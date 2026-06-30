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
    State(_state): State<Arc<AppState>>,
    Json(_req): Json<RefreshRequest>,
) -> Result<(), AppError> {
    todo!()
}

pub async fn logout(
    State(_state): State<Arc<AppState>>,
) -> Result<(), AppError> {
    todo!()
}