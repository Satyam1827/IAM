use std::sync::Arc;

use axum::{
    routing::get,
    Router,
};

use crate::handlers::session;
use crate::state::AppState;

pub fn router()
-> Router<Arc<AppState>>
{
    Router::new()
        .route(
            "/",
            get(session::list),
        )
}