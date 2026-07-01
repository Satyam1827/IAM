use std::sync::Arc;

use axum::{
    routing::post,
    Router,
};

use crate::{
    handlers::member_role,
    state::AppState,
};

pub fn router()
-> Router<Arc<AppState>>
{
    Router::new()
        .route(
            "/{org_id}/members/{user_id}/roles/{role_id}",
            post(member_role::assign),
        )
}