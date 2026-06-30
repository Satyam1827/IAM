use std::sync::Arc;

use axum::{
    extract::State,
    Json,
};

use crate::{
    auth::extractor::CurrentUser,
    db::repositories::user_repo,
    dto::user::UserResponse,
    errors::AppError,
    state::AppState,
};

pub async fn me(
    current_user: CurrentUser,
    State(state): State<Arc<AppState>>,
) -> Result<Json<UserResponse>, AppError>
{
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

    Ok(Json(UserResponse {
        id: user.id,
        email: user.email,
        name: user.name,
    }))
}