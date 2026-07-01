use std::sync::Arc;

use axum::Router;

use crate::{
    routes,
    state::AppState,
};

use axum::middleware;

use crate::middleware::auth;

pub fn create_router(
    state: Arc<AppState>,
) -> Router {
    Router::new()
    .nest(
        "/auth",
        routes::auth::router(),
    )
    .nest(
        "/users",
        routes::user::router()
            .route_layer(
                middleware::from_fn_with_state(
                    state.clone(),
                    auth::auth,
                ),
            ),
    )
    
    .nest(
        "/sessions",
        routes::session::router()
            .route_layer(
                middleware::from_fn_with_state(
                    state.clone(),
                    auth::auth,
                ),
            ),
    )
    .nest(
    "/auth",
    routes::auth::protected_router()
        .route_layer(
            middleware::from_fn_with_state(
                state.clone(),
                auth::auth,
            ),
        ),
    )
    .with_state(state)
}

