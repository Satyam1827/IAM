use std::sync::Arc;

use uuid::Uuid;

use crate::{
    db::repositories::audit_repo,
    state::AppState,
};

pub async fn log(
    state: Arc<AppState>,
    organization_id: Option<Uuid>,
    actor_id: Option<Uuid>,
    action: &str,
    resource_type: Option<&str>,
    resource_id: Option<Uuid>,
) {
        if let Err(e) = audit_repo::create(
            &state.db,
            organization_id,
            actor_id,
            action,
            resource_type,
            resource_id,
        )
        .await {
            println!("AUDIT ERROR: {e:?}");
        }
}