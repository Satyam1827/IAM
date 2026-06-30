use std::sync::Arc;

use axum::{
    extract::State,
    Json,
};

use crate::{
    auth::extractor::CurrentUser,
    db::repositories::session_repo,
    dto::session::SessionResponse,
    errors::AppError,
    state::AppState,
};

pub async fn list(
    current_user: CurrentUser,
    State(state): State<Arc<AppState>>,
) -> Result<
    Json<Vec<SessionResponse>>,
    AppError,
> {
    let sessions =
        session_repo::find_by_user(
            &state.db,
            current_user.user_id,
        )
        .await
        .map_err(|_| {
            AppError::Internal
        })?;

    let response =
        sessions
            .into_iter()
            .map(|s| SessionResponse {
                id: s.id,
                user_agent: s.user_agent,
                ip_address: s.ip_address,
                expires_at: s.expires_at,
                created_at: s.created_at,
            })
            .collect();

    Ok(Json(response))
}