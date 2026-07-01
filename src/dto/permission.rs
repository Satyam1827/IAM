use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct AssignPermissionRequest {
    pub permission_id: Uuid,
}