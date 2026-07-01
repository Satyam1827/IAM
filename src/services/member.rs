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

    Ok(MemberResponse {
        user_id:
            membership.user_id,
        organization_id:
            membership.organization_id,
    })
}