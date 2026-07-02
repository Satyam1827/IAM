use std::sync::Arc;

use axum::{
    routing::{post, delete},
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
            post(api_key::create)
                .get(api_key::list),
        )
        .route(
            "/{org_id}/api-keys/{key_id}",
            delete(api_key::revoke),
        )
}