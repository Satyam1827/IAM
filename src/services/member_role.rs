use std::sync::Arc;

use uuid::Uuid;

use crate::{
    auth::extractor::CurrentUser,
    db::repositories::{
        membership_repo,
        role_repo,
    },
    errors::AppError,
    state::AppState,
};

pub async fn assign(
    state: Arc<AppState>,
    current_user: CurrentUser,
    organization_id: Uuid,
    user_id: Uuid,
    role_id: Uuid,
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

    let membership =
        membership_repo::find(
            &state.db,
            user_id,
            organization_id,
        )
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(
            AppError::NotFound,
        )?;

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

    membership_repo::assign_role(
        &state.db,
        membership.id,
        role.id,
    )
    .await
    .map_err(|_| AppError::Internal)?;

    Ok(())
}