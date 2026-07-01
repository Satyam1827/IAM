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
    dto::permission::AssignPermissionRequest,
    errors::AppError,
    services::role_permission,
    state::AppState,
};

pub async fn assign(
    current_user: CurrentUser,
    Path((organization_id, role_id)):
        Path<(Uuid, Uuid)>,
    State(state): State<Arc<AppState>>,
    Json(req): Json<AssignPermissionRequest>,
) -> Result<(), AppError>
{
    role_permission::assign(
        state,
        current_user,
        organization_id,
        role_id,
        req,
    )
    .await
}