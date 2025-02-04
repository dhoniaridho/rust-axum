use std::sync::Arc;

use crate::infrasturcture::database::DB;

pub struct AppState {
    pub db: Arc<DB>,
}
