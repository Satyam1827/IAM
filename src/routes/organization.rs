use std::sync::Arc;

use axum::{
    routing::post,
    Router,
};

use crate::{
    handlers::organization,
    state::AppState,
};

pub fn router()
-> Router<Arc<AppState>>
{
    Router::new()
        .route(
            "/",
            post(organization::create),
        )
}