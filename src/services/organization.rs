use std::sync::Arc;

use crate::{
    auth::extractor::CurrentUser,
    db::repositories::{
        membership_repo,
        organization_repo,
        role_repo,
        permission_repo
    },
    dto::organization::{
        CreateOrganizationRequest,
        OrganizationResponse,
    },
    errors::AppError,
    state::AppState,
};


pub async fn create(
    state: Arc<AppState>,
    current_user: CurrentUser,
    req: CreateOrganizationRequest,
) -> Result<OrganizationResponse, AppError>
{
    let mut tx =
        state
            .db
            .begin()
            .await
            .map_err(|_| {
                AppError::Internal
            })?;

    let organization =
        organization_repo::create_tx(
            &mut tx,
            &req.name,
            &req.slug,
            current_user.user_id,
        )
        .await
        .map_err(|_| {
            AppError::Internal
        })?;

    let membership = membership_repo::create_tx(
        &mut tx,
        current_user.user_id,
        organization.id,
    )
    .await
    .map_err(|_| {
        AppError::Internal
    })?;
    let owner_role =
        role_repo::create_tx(
            &mut tx,
            organization.id,
            "Owner",
            Some(
                "Organization owner"
            ),
        )
        .await
        .map_err(|_| {
            AppError::Internal
        })?;
    membership_repo::assign_role_tx(
        &mut tx,
        membership.id,
        owner_role.id,
    )
    .await
    .map_err(|_| {
        AppError::Internal
    })?;

    let permissions = [
        (
            "org:update",
            "Update organization",
        ),
        (
            "member:add",
            "Add members",
        ),
        (
            "member:remove",
            "Remove members",
        ),
        (
            "role:assign",
            "Assign roles",
        ),
    ];

    for permission_name in [
        "org:update",
        "member:add",
        "member:remove",
        "role:assign",
    ] {
        let permission =
            permission_repo::find_by_name(
                &state.db,
                permission_name,
            )
            .await
            .map_err(|_| AppError::Internal)?
            .ok_or(
                AppError::Internal,
            )?;

        role_repo::assign_permission_tx(
            &mut tx,
            owner_role.id,
            permission.id,
        )
        .await
        .map_err(|_| AppError::Internal)?;
    }
        
    tx.commit()
        .await
        .map_err(|_| {
            AppError::Internal
        })?;
    
    crate::services::audit::log(
        state.clone(),
        Some(organization.id),
        Some(current_user.user_id),
        "organization.create",
        Some("organization"),
        Some(organization.id),
    )
    .await;

    

    Ok(OrganizationResponse {
        id: organization.id,
        name: organization.name,
        slug: organization.slug,
    })
}