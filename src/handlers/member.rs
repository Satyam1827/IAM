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
    dto::member::{
        AddMemberRequest,
        MemberResponse,
    },
    errors::AppError,
    services::member,
    state::AppState,
};

pub async fn add(
    current_user: CurrentUser,
    Path(organization_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    Json(req): Json<AddMemberRequest>,
) -> Result<Json<MemberResponse>, AppError>
{
    let member =
        member::add(
            state,
            current_user,
            organization_id,
            req,
        )
        .await?;

    Ok(Json(member))
}

pub async fn list(
    current_user: CurrentUser,
    Path(organization_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<
    Json<Vec<MemberResponse>>,
    AppError,
> {
    let members =
        member::list(
            state,
            current_user,
            organization_id,
        )
        .await?;

    Ok(Json(members))
}

pub async fn remove(
    current_user: CurrentUser,
    Path((organization_id, user_id)):
        Path<(Uuid, Uuid)>,
    State(state): State<Arc<AppState>>,
) -> Result<(), AppError>
{
    member::remove(
        state,
        current_user,
        organization_id,
        user_id,
    )
    .await
}