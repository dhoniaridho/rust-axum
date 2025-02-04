use std::sync::Arc;

use uuid::Uuid;

use crate::{
    domains::user::{
        dto::request::GetUserListRequest, model::user::User,
        repository::user_repository::UserRepository,
    },
    shared::dto::Paginated,
};

pub struct UserUseCase {
    user_repository: Arc<UserRepository>,
}

impl UserUseCase {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        UserUseCase { user_repository }
    }
    pub fn list(&self, q: GetUserListRequest) -> Result<Paginated<User>, String> {
        match self.user_repository.paginate(q) {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        }
    }
    pub fn get(&self, id: Uuid) -> Result<User, String> {
        match self.user_repository.get(id) {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        }
    }
    pub fn delete(&self, id: Uuid) -> Result<Option<User>, String> {
        match self.user_repository.delete(id) {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        }
    }
    pub fn update(&self) -> Result<String, String> {
        Ok("test".to_string())
    }
    pub fn create(&self) -> Result<String, String> {
        Ok("test".to_string())
    }
}
