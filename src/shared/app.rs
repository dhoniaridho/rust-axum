use std::sync::Arc;

use crate::{domains::user::use_case::user::UserUseCase, infrasturcture::database::DB};

pub struct AppState {
    pub user_use_case: UserUseCase,
    pub db: Arc<DB>,
}