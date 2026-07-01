use std::sync::Arc;

use crate::{
    auth::extractor::CurrentUser,
    db::repositories::{
        membership_repo,
        organization_repo,
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

    membership_repo::create_tx(
        &mut tx,
        current_user.user_id,
        organization.id,
    )
    .await
    .map_err(|_| {
        AppError::Internal
    })?;

    tx.commit()
        .await
        .map_err(|_| {
            AppError::Internal
        })?;

    Ok(OrganizationResponse {
        id: organization.id,
        name: organization.name,
        slug: organization.slug,
    })
}