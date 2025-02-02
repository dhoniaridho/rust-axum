use axum::Router;
use std::sync::Arc;
use crate::{adapters::http::user_controller::user_routes, domains::user::use_case::user::UserUseCase, infrasturcture::database::DB, shared::app::AppState};

pub fn create_routes() -> Router {
    let state  = Arc::new(
        AppState {
            user_use_case: UserUseCase::new(),
            db: Arc::new(DB::new()),
        }
    );
    Router::new().nest("/users", user_routes(state))
}
