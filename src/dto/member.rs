use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct AddMemberRequest {
    pub user_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct MemberResponse {
    pub user_id: Uuid,
    pub organization_id: Uuid,
}

