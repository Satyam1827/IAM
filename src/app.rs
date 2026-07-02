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
    "/organizations",
    routes::organization::router()
        .route_layer(
            middleware::from_fn_with_state(
                state.clone(),
                auth::auth,
            ),
        ),
    )
    .nest(
        "/organizations",
        routes::member::router()
            .route_layer(
                middleware::from_fn_with_state(
                    state.clone(),
                    auth::auth,
                ),
            ),
    )

    .nest(
        "/organizations",
        routes::role::router()
            .route_layer(
                middleware::from_fn_with_state(
                    state.clone(),
                    auth::auth,
                ),
            ),
    )
    .nest(
        "/organizations",
        routes::member_role::router()
            .route_layer(
                middleware::from_fn_with_state(
                    state.clone(),
                    auth::auth,
                ),
            ),
    )

    .nest(
        "/organizations",
        routes::role_permission::router()
            .route_layer(
                middleware::from_fn_with_state(
                    state.clone(),
                    auth::auth,
                ),
            ),
    )

    .nest(
        "/organizations",
        routes::api_key::router()
            .route_layer(
                middleware::from_fn_with_state(
                    state.clone(),
                    auth::auth,
                ),
            ),
    )

    .merge(
        routes::test::router()
            .route_layer(
                middleware::from_fn_with_state(
                    state.clone(),
                    crate::middleware::api_key::auth,
                ),
            )
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

    .nest(
        "/auth",
        routes::session::router()
            .route_layer(
                middleware::from_fn_with_state(
                    state.clone(),
                    auth::auth,
                ),
            ),
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
    .with_state(state)

    
}