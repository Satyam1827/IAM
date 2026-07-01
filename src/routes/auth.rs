use axum::{
    routing::post,
    Router,
};

use crate::handlers::auth;
use std::sync::Arc;
use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .route("/refresh", post(auth::refresh))
}

pub fn protected_router()
-> Router<Arc<AppState>>
{
    Router::new()
        .route(
            "/logout",
            post(auth::logout),
        )
}