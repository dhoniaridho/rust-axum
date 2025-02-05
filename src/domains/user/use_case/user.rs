use std::sync::Arc;

use uuid::Uuid;

use crate::{
    domains::user::{
        dto::request::{CreateUserBodyRequest, GetUserListRequest, UpdateUserBodyRequest},
        model::user::User,
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
    pub fn update(&self, id: Uuid, data: UpdateUserBodyRequest) -> Result<User, String> {
        match self.user_repository.update(id, data.to_model()) {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        }
    }
    pub fn create(&self, data: CreateUserBodyRequest) -> Result<User, String> {
        match self.user_repository.create(data.to_model()) {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        }
    }
}
