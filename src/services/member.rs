use std::sync::Arc;

use uuid::Uuid;

use crate::{
    auth::extractor::CurrentUser,
    db::repositories::{
        membership_repo,
        role_repo,
    },
    dto::member::{
        AddMemberRequest,
        MemberResponse,
    },
    errors::AppError,
    state::AppState,
};

// Adding a member
pub async fn add(
    state: Arc<AppState>,
    current_user: CurrentUser,
    organization_id: Uuid,
    req: AddMemberRequest,
) -> Result<MemberResponse, AppError>
{
    let allowed =
        role_repo::has_permission(
            &state.db,
            current_user.user_id,
            organization_id,
            "member:add",
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

    let existing =
        membership_repo::find(
            &state.db,
            req.user_id,
            organization_id,
        )
        .await
        .map_err(|_| {
            AppError::Internal
        })?;

    if existing.is_some() {
        return Err(
            AppError::Conflict(
                "user is already a member"
                    .into(),
            ),
        );
    }

    let membership =
        membership_repo::create(
            &state.db,
            req.user_id,
            organization_id,
        )
        .await
        .map_err(|_| {
            AppError::Internal
        })?;
    
    crate::services::audit::log(
        state.clone(),
        Some(organization_id),
        Some(current_user.user_id),
        "member.add",
        Some("user"),
        Some(req.user_id),
    )
    .await;

    Ok(MemberResponse {
        user_id:
            membership.user_id,
        organization_id:
            membership.organization_id,
    })
}

// List members of the organisation
pub async fn list(
    state: Arc<AppState>,
    current_user: CurrentUser,
    organization_id: Uuid,
) -> Result<Vec<MemberResponse>, AppError>
{
    let allowed =
        role_repo::has_permission(
            &state.db,
            current_user.user_id,
            organization_id,
            "member:add",
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

    let memberships =
        membership_repo::list_members(
            &state.db,
            organization_id,
        )
        .await
        .map_err(|_| {
            AppError::Internal
        })?;

    Ok(
        memberships
            .into_iter()
            .map(|m| MemberResponse {
                user_id: m.user_id,
                organization_id:
                    m.organization_id,
            })
            .collect(),
    )
}

pub async fn remove(
    state: Arc<AppState>,
    current_user: CurrentUser,
    organization_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError>
{
    if user_id == current_user.user_id {
        return Err(
            AppError::BadRequest(
                "cannot remove yourself".into(),
            ),
        );
    }
    
    let allowed =
        role_repo::has_permission(
            &state.db,
            current_user.user_id,
            organization_id,
            "member:remove",
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
        .map_err(|_| AppError::Internal)?;

    if membership.is_none() {
        return Err(
            AppError::NotFound,
        );
    }

    membership_repo::delete(
        &state.db,
        user_id,
        organization_id,
    )
    .await
    .map_err(|_| AppError::Internal)?;

    crate::services::audit::log(
        state.clone(),
        Some(organization_id),
        Some(current_user.user_id),
        "member.remove",
        Some("user"),
        Some(user_id),
    )
    .await;

    Ok(())
}