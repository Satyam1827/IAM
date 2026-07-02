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
    auth::extractor::CurrentUser,
    dto::api_key::{
        CreateApiKeyRequest,
        CreateApiKeyResponse,
    },
    errors::AppError,
    services::api_key,
    state::AppState,
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
