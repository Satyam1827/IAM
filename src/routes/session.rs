use std::sync::Arc;

use axum::{
    routing::{
        delete,
        get,
    },
    Router,
};

use crate::{
    handlers::session,
    state::AppState,
};

pub fn router()
-> Router<Arc<AppState>>
{
    Router::new()
        .route(
            "/sessions",
            get(session::list),
        )
        .route(
            "/sessions/{id}",
            delete(session::revoke),
        )
}