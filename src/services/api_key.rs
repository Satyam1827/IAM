use std::sync::Arc;

use crate::{
    auth::{
        api_key, extractor::CurrentUser, password,
    }, db::repositories::{
        api_key_repo,
        role_repo,
    }, dto::api_key::{
        ApiKeyResponse, CreateApiKeyRequest, CreateApiKeyResponse,
    }, errors::AppError, state::AppState,
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

pub async fn list(
    state: Arc<AppState>,
    current_user: CurrentUser,
    organization_id: Uuid,
) -> Result<
    Vec<ApiKeyResponse>,
    AppError,
>
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

    let keys =
        api_key_repo::list_by_organization(
            &state.db,
            organization_id,
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(
        keys.into_iter()
            .map(|k| ApiKeyResponse {
                id: k.id,
                name: k.name,
                created_at: k.created_at,
                expires_at: k.expires_at,
                last_used_at: k.last_used_at,
            })
            .collect(),
    )
}

pub async fn revoke(
    state: Arc<AppState>,
    current_user: CurrentUser,
    organization_id: Uuid,
    key_id: Uuid,
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

    let key =
        api_key_repo::find_by_id(
            &state.db,
            key_id,
        )
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(
            AppError::NotFound,
        )?;

    if key.organization_id
        != organization_id
    {
        return Err(
            AppError::BadRequest(
                "api key does not belong to organization"
                    .into(),
            ),
        );
    }

    api_key_repo::delete(
        &state.db,
        key.id,
    )
    .await
    .map_err(|_| AppError::Internal)?;

    Ok(())
}