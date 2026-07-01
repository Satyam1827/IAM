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
    dto::role::{
        CreateRoleRequest,
        RoleResponse,
    },
    errors::AppError,
    services::role,
    state::AppState,
};

pub async fn create(
    current_user: CurrentUser,
    Path(organization_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateRoleRequest>,
) -> Result<Json<RoleResponse>, AppError>
{
    let role =
        role::create(
            state,
            current_user,
            organization_id,
            req,
        )
        .await?;

    Ok(Json(role))
}