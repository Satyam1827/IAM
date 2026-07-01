use std::sync::Arc;

use axum::{
    routing::post,
    Router,
};

use crate::{
    handlers::role_permission,
    state::AppState,
};

pub fn router()
-> Router<Arc<AppState>>
{
    Router::new()
        .route(
            "/{org_id}/roles/{role_id}/permissions",
            post(role_permission::assign),
        )
}