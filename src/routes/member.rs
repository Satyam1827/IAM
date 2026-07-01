use std::sync::Arc;

use axum::{
    routing::post,
    Router,
};

use crate::{
    handlers::member,
    state::AppState,
};

pub fn router()
-> Router<Arc<AppState>>
{
    Router::new()
        .route(
            "/{id}/members",
            post(member::add),
        )
}