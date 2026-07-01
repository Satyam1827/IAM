use std::sync::Arc;

use axum::{
    routing::post,
    Router,
};

use crate::{
    handlers::role,
    state::AppState,
};

pub fn router()
-> Router<Arc<AppState>>
{
    Router::new()
        .route(
            "/{id}/roles",
            post(role::create),
        )
}