use std::sync::Arc;

use uuid::Uuid;

use crate::{
    auth::extractor::CurrentUser,
    db::repositories::session_repo,
    dto::session::SessionResponse,
    errors::AppError,
    state::AppState,
};

pub async fn list(
    state: Arc<AppState>,
    current_user: CurrentUser,
) -> Result<
    Vec<SessionResponse>,
    AppError,
> {
    let sessions =
        session_repo::list_by_user(
            &state.db,
            current_user.user_id,
        )
        .await
        .map_err(|_| {
            AppError::Internal
        })?;

    Ok(
        sessions
            .into_iter()
            .map(|s| SessionResponse {
                id: s.id,
                user_agent:
                    s.user_agent,
                ip_address:
                    s.ip_address,
                expires_at:
                    s.expires_at,
                created_at:
                    s.created_at,
            })
            .collect(),
    )
}

pub async fn revoke(
    state: Arc<AppState>,
    current_user: CurrentUser,
    session_id: Uuid,
) -> Result<(), AppError>
{
    let session =
        session_repo::find_by_id(
            &state.db,
            session_id,
        )
        .await
        .map_err(|_| {
            AppError::Internal
        })?
        .ok_or(
            AppError::NotFound,
        )?;

    if session.user_id
        != current_user.user_id
    {
        return Err(
            AppError::Forbidden,
        );
    }

    session_repo::delete(
        &state.db,
        session.id,
    )
    .await
    .map_err(|_| {
        AppError::Internal
    })?;

    Ok(())
}