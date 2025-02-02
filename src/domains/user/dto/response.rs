use serde::Serialize;
use crate::domains::user::model::user::User;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub deleted_at: String,
    pub created_at: String,
    pub updated_at: String,
}

impl UserResponse {
    pub fn new(model: User) -> Self {
        Self {
            id: model.id.to_string(),
            name: model.name,
            email: model.email,
            username: model.username,
            password: model.password,
            deleted_at: model.deleted_at.map(|d| d.to_string()).unwrap_or_default(),
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}
