use axum::Router;
use std::sync::Arc;
use crate::adapters::http::user_controller::user_routes;
use crate::domains::user::use_case::user::RegisterUserUseCase;

pub fn create_routes() -> Router {
    let use_case = Arc::new(RegisterUserUseCase); // Inject dependencies
    Router::new().nest("/users", user_routes(use_case))
}
