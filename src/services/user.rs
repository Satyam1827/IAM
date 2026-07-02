use std::sync::Arc;

use crate::{
    auth::extractor::CurrentUser,
    db::repositories::user_repo,
    dto::user::{
        UpdateProfileRequest,
        UserProfileResponse,
    },
    errors::AppError,
    state::AppState,
};

pub async fn me(
    state: Arc<AppState>,
    current_user: CurrentUser,
) -> Result<
    UserProfileResponse,
    AppError,
> {
    let user =
        user_repo::find_by_id(
            &state.db,
            current_user.user_id,
        )
        .await
        .map_err(|_| {
            AppError::Internal
        })?
        .ok_or(
            AppError::NotFound,
        )?;

    Ok(UserProfileResponse {
        id: user.id,
        email: user.email,
        name: user.name,
    })
}

pub async fn update(
    state: Arc<AppState>,
    current_user: CurrentUser,
    req: UpdateProfileRequest,
) -> Result<
    UserProfileResponse,
    AppError,
> {
    let user =
        user_repo::update_name(
            &state.db,
            current_user.user_id,
            &req.name,
        )
        .await
        .map_err(|_| {
            AppError::Internal
        })?;

    Ok(UserProfileResponse {
        id: user.id,
        email: user.email,
        name: user.name,
    })
}