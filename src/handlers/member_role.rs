use std::sync::Arc;

use axum::extract::{
    Path,
    State,
};
use uuid::Uuid;

use crate::{
    auth::extractor::CurrentUser,
    errors::AppError,
    services::member_role,
    state::AppState,
};

pub async fn assign(
    current_user: CurrentUser,
    Path((organization_id, user_id, role_id)):
        Path<(Uuid, Uuid, Uuid)>,
    State(state): State<Arc<AppState>>,
) -> Result<(), AppError>
{
    member_role::assign(
        state,
        current_user,
        organization_id,
        user_id,
        role_id,
    )
    .await
}