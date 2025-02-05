use serde::Deserialize;
use validator::Validate;

use crate::domains::user::model::user::{PartialUser, User};

#[derive(Debug, Validate, Deserialize)]
pub struct GetUserListRequest {
    #[validate(range(min = 1, message = "page must be greater than 0"))]
    pub page: Option<u32>,

    #[validate(range(min = 1, message = "page_size must be greater than 0"))]
    pub page_size: Option<u32>,
}

#[derive(Validate, Deserialize, Debug)]
pub struct CreateUserBodyRequest {
    #[validate(length(min = 4))]
    pub name: String,

    #[validate(length(min = 4))]
    pub email: String,

    #[validate(length(min = 4))]
    pub username: String,

    #[validate(length(min = 4))]
    pub password: String,
}

impl CreateUserBodyRequest {
    pub fn to_model(&self) -> User {
        User::new(CreateUserBodyRequest {
            name: self.name.clone(),
            email: self.email.clone(),
            username: self.username.clone(),
            password: self.password.clone(),
        })
    }
}

#[derive(Validate, Deserialize, Debug)]
pub struct UpdateUserBodyRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl UpdateUserBodyRequest {
    pub fn to_model(&self) -> PartialUser {
        PartialUser {
            name: self.name.clone(),
            email: self.email.clone(),
            username: self.username.clone(),
            password: self.password.clone(),
        }
    }
}
