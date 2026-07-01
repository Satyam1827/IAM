use std::sync::Arc;

use uuid::Uuid;

use crate::{
    auth::extractor::CurrentUser,
    db::repositories::{
        permission_repo,
        role_repo,
    },
    dto::permission::AssignPermissionRequest,
    errors::AppError,
    state::AppState,
};

//Assign Permission
pub async fn assign(
    state: Arc<AppState>,
    current_user: CurrentUser,
    organization_id: Uuid,
    role_id: Uuid,
    req: AssignPermissionRequest,
) -> Result<(), AppError>
{
    let allowed =
        role_repo::has_permission(
            &state.db,
            current_user.user_id,
            organization_id,
            "role:assign",
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if !allowed {
        return Err(
            AppError::Forbidden,
        );
    }

    let role =
        role_repo::find_by_id(
            &state.db,
            role_id,
        )
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(
            AppError::NotFound,
        )?;

    if role.organization_id
        != organization_id
    {
        return Err(
            AppError::BadRequest(
                "role does not belong to organization"
                    .into(),
            ),
        );
    }

    permission_repo::find_by_id(
        &state.db,
        req.permission_id,
    )
    .await
    .map_err(|_| AppError::Internal)?
    .ok_or(
        AppError::NotFound,
    )?;

    role_repo::assign_permission(
        &state.db,
        role_id,
        req.permission_id,
    )
    .await
    .map_err(|_| AppError::Internal)?;

    Ok(())
}