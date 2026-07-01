use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateOrganizationRequest {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize)]
pub struct OrganizationResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
}