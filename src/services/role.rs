use std::sync::Arc;

use uuid::Uuid;

use crate::{
    auth::extractor::CurrentUser,
    db::repositories::role_repo,
    dto::role::{
        CreateRoleRequest,
        RoleResponse,
    },
    errors::AppError,
    state::AppState,
};

// Create custom role
pub async fn create(
    state: Arc<AppState>,
    current_user: CurrentUser,
    organization_id: Uuid,
    req: CreateRoleRequest,
) -> Result<RoleResponse, AppError>
{
    let allowed =
        role_repo::has_permission(
            &state.db,
            current_user.user_id,
            organization_id,
            "role:assign",
        )
        .await
        .map_err(|_| {
            AppError::Internal
        })?;

    if !allowed {
        return Err(
            AppError::Forbidden,
        );
    }

    let role =
        role_repo::create(
            &state.db,
            organization_id,
            &req.name,
            req.description.as_deref(),
        )
        .await
        .map_err(|_| {
            AppError::Internal
        })?;

    crate::services::audit::log(
        state.clone(),
        Some(organization_id),
        Some(current_user.user_id),
        "role.create",
        Some("role"),
        Some(role.id),
    )
    .await;

    Ok(RoleResponse {
        id: role.id,
        name: role.name,
        description:
            role.description,
    })
}