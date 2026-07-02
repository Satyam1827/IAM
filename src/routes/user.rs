use std::sync::Arc;

use axum::{
    routing::get,
    Router,
};

use crate::{
    handlers::user,
    state::AppState,
};

pub fn router()
-> Router<Arc<AppState>>
{
    Router::new()
        .route(
            "/me",
            get(user::me)
                .patch(user::update),
        )
}