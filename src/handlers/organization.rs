use std::sync::Arc;

use axum::{
    extract::State,
    Json,
};

use crate::{
    auth::extractor::CurrentUser,
    dto::organization::{
        CreateOrganizationRequest,
        OrganizationResponse,
    },
    errors::AppError,
    services::organization,
    state::AppState,
};

pub async fn create(
    current_user: CurrentUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateOrganizationRequest>,
) -> Result<Json<OrganizationResponse>, AppError>
{
    let organization =
        organization::create(
            state,
            current_user,
            req,
        )
        .await?;

    Ok(Json(organization))
}