use std::sync::Arc;

use axum::Router;

use crate::{
    routes,
    state::AppState,
};

pub fn create_router(
    state: Arc<AppState>,
) -> Router {
    Router::new()
        .nest(
            "/auth",
            routes::auth::router(),
        )
        .with_state(state)
}