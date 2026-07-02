use std::sync::Arc;

use crate::{
    auth::{
        api_key,
        password,
        extractor::CurrentUser,
    },
    db::repositories::{
        api_key_repo,
        role_repo,
    },
    dto::api_key::{
        CreateApiKeyRequest,
        CreateApiKeyResponse,
    },
    errors::AppError,
    state::AppState,
};

use uuid::Uuid;

//create api keys
pub async fn create(
    state: Arc<AppState>,
    current_user: CurrentUser,
    organization_id: Uuid,
    req: CreateApiKeyRequest,
) -> Result<CreateApiKeyResponse, AppError>
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

    let raw_key =
        api_key::generate();

    let hash =
        password::hash_password(
            &raw_key,
        )
        .map_err(|_| {
            AppError::Internal
        })?;

    let api_key =
        api_key_repo::create(
            &state.db,
            organization_id,
            &req.name,
            &hash,
            current_user.user_id,
        )
        .await
        .map_err(|_| {
            AppError::Internal
        })?;

    Ok(
        CreateApiKeyResponse {
            id: api_key.id,
            name: api_key.name,
            api_key: raw_key,
        },
    )
}