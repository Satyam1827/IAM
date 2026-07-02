use std::sync::Arc;

use axum::{
    routing::post,
    Router,
};

use crate::{
    handlers::api_key,
    state::AppState,
};

pub fn router()
-> Router<Arc<AppState>>
{
    Router::new()
        .route(
            "/{id}/api-keys",
            post(api_key::create),
        )
}