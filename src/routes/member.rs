use std::sync::Arc;

use axum::{
    routing::{post, get, delete},
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
        .route(
            "/{id}/members",
            get(member::list),
        )
        .route(
            "/{org_id}/members/{user_id}",
            delete(member::remove),
        )
}