use std::sync::Arc;

use axum::{
    extract::{
        Path,
        State,
    },
    Json,
};
use uuid::Uuid;

use crate::{
    auth::extractor::CurrentUser,
    dto::session::SessionResponse,
    errors::AppError,
    services::session,
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
        session::list(
            state,
            current_user,
        )
        .await?;

    Ok(Json(sessions))
}


pub async fn revoke(
    current_user: CurrentUser,
    Path(session_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<(), AppError>
{
    session::revoke(
        state,
        current_user,
        session_id,
    )
    .await
}