use std::sync::Arc;

use crate::{
    domains::user::{dto::request::GetUserListRequest, model::user::User},
    infrasturcture::database::DB,
};

pub struct UserUseCase;

impl UserUseCase {
    pub fn new() -> Self {
        UserUseCase
    }
    pub fn list(&self, db: Arc<DB>, q: GetUserListRequest) -> Result<Vec<User>, String> {
        use crate::schema::users::dsl::*;
        use diesel::prelude::*;
        let mut conn = Arc::clone(&db.pool).get().unwrap();
        let size = q.page_size.unwrap_or(1).into();
        let page = q.page.unwrap_or(1);
        let page = if page > 1 { Some(page - 1) } else { None }; // Result handling
        let offset = i64::from(page.unwrap_or(0)) * size;

        let data = users
            .offset(i64::from(offset))
            .limit(size)
            .load::<User>(&mut conn)
            .unwrap();
        Ok(data)
    }
    pub fn get(&self) -> Result<String, String> {
        Ok("test".to_string())
    }
    pub fn delete(&self) -> Result<String, String> {
        Ok("test".to_string())
    }
    pub fn update(&self) -> Result<String, String> {
        Ok("test".to_string())
    }
    pub fn create(&self) -> Result<String, String> {
        Ok("test".to_string())
    }
}
