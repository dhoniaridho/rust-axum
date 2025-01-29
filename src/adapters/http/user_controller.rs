use crate::domains::user::use_case::{self, user::RegisterUserUseCase};
// use crate::shared::errors::AppError;
use axum::{
    // extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, vec};

#[derive(Deserialize)]
pub struct RegisterUserRequest {
    pub email: String,
    // pub password: String,
}

#[derive(Serialize)]
pub struct RegisterUserResponse {
    pub user_id: String,
    pub message: String,
}

pub async fn register_user(
    Json(payload): Json<RegisterUserRequest>,
) -> Result<Json<RegisterUserResponse>, (StatusCode, String)> {
    print!("{}", payload.email);
    match use_case::user::RegisterUserUseCase::execute() {
        Ok(d) => Ok(Json(RegisterUserResponse {
            user_id: d,
            message: "User registered successfully".to_string(),
        })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

pub async fn list() -> Result<Json<Vec<String>>, (StatusCode, String)> {
    Ok(Json(vec!["test".to_string()]))
}

// Function to create user routes
pub fn user_routes(use_case: Arc<RegisterUserUseCase>) -> Router {
    Router::new()
        .route("/", get(list))
        .route("/register", post(register_user))
        .route("/{id}", get(use_case::user::RegisterUserUseCase::execute()))
        .with_state(use_case)
}
