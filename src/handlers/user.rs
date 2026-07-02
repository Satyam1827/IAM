use std::sync::Arc;

use axum::{
    extract::State,
    Json,
};

use crate::{
    auth::extractor::CurrentUser,
    dto::user::{
        UpdateProfileRequest,
        UserProfileResponse,
    },
    errors::AppError,
    services::user,
    state::AppState,
};

pub async fn me(
    current_user: CurrentUser,
    State(state): State<Arc<AppState>>,
) -> Result<
    Json<UserProfileResponse>,
    AppError,
> {
    let profile =
        user::me(
            state,
            current_user,
        )
        .await?;

    Ok(Json(profile))
}

pub async fn update(
    current_user: CurrentUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<
    Json<UserProfileResponse>,
    AppError,
> {
    let profile =
        user::update(
            state,
            current_user,
            req,
        )
        .await?;

    Ok(Json(profile))
}