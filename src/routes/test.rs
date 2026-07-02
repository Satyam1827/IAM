use std::sync::Arc;

use axum::{
    routing::get,
    Router,
};

use crate::{
    handlers::test,
    state::AppState,
};

pub fn router()
-> Router<Arc<AppState>>
{
    Router::new()
        .route(
            "/test-api-key",
            get(test::protected),
        )
}