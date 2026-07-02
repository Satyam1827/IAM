use std::sync::Arc;

use axum::{
    extract::{
        Path,
        State,
    },
    Json,
};
use uuid::Uuid;

use crate::{
    auth::extractor::CurrentUser, dto::api_key::{
        ApiKeyResponse, CreateApiKeyRequest, CreateApiKeyResponse,
    }, errors::AppError, services::api_key, state::AppState,
};

pub async fn create(
    current_user: CurrentUser,
    Path(organization_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateApiKeyRequest>,
) -> Result<
    Json<CreateApiKeyResponse>,
    AppError,
> {
    let api_key =
        api_key::create(
            state,
            current_user,
            organization_id,
            req,
        )
        .await?;

    Ok(Json(api_key))
}

pub async fn list(
    current_user: CurrentUser,
    Path(organization_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<
    Json<Vec<ApiKeyResponse>>,
    AppError,
> {
    let keys =
        api_key::list(
            state,
            current_user,
            organization_id,
        )
        .await?;

    Ok(Json(keys))
}

pub async fn revoke(
    current_user: CurrentUser,
    Path((organization_id, key_id)):
        Path<(Uuid, Uuid)>,
    State(state): State<Arc<AppState>>,
) -> Result<(), AppError>
{
    api_key::revoke(
        state,
        current_user,
        organization_id,
        key_id,
    )
    .await
}