use std::sync::Arc;

use crate::{
    auth::password,
    db::{
        models::api_key::ApiKey,
        repositories::api_key_repo,
    },
    state::AppState,
};

pub async fn authenticate(
    state: Arc<AppState>,
    raw_key: &str,
) -> Option<ApiKey> {
    let keys =
        api_key_repo::all(
            &state.db,
        )
        .await
        .ok()?;

    for key in keys {
        let valid =
            password::verify_password(
                raw_key,
                &key.key_hash,
            )
            .ok()?;

        if valid {
            return Some(key);
        }
    }

    None
}