use std::sync::Arc;

use axum::{
    middleware,
    routing::get,
    Router,
};

use crate::{
    handlers::user,
    middleware::auth,
    state::AppState,
};

pub fn router()
-> Router<Arc<AppState>>
{
    Router::new()
        .route(
            "/me",
            get(user::me),
        )
}