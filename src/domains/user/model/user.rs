use bcrypt::BcryptResult;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{domains::user::dto::request::CreateUserBodyRequest, schema::users};

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub username: String,
    pub password: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct PartialUser {
    pub email: Option<String>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl User {
    pub fn new(user: CreateUserBodyRequest) -> Self {
        match bcrypt::hash(user.password, bcrypt::DEFAULT_COST) {
            Ok(p) => User {
                id: Uuid::new_v4(),
                email: user.email,
                name: user.name,
                username: user.username,
                password: p,
                deleted_at: None,
                created_at: chrono::Utc::now().naive_utc(),
                updated_at: chrono::Utc::now().naive_utc(),
            },
            Err(e) => panic!("{}", e),
        }
    }

    pub fn verify_password(&self, password: &str) -> BcryptResult<bool> {
        bcrypt::verify(password, &self.password)
    }
}
