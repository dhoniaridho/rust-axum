use axum::{
    extract::State,
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use std::sync::Arc;
use validator::{Validate, ValidationErrors};

use crate::{
    domains::user::dto::{request::GetUserListRequest, response::UserResponse},
    shared::{app::AppState, errors::ErrorResponse, extractor::Qs, response::HttpResponse},
};

pub async fn list(
    State(state): State<Arc<AppState>>,
    Qs(q): Qs<GetUserListRequest>,
) -> Result<
    Json<HttpResponse<Vec<UserResponse>>>,
    (StatusCode, Json<ErrorResponse<ValidationErrors>>),
> {
    if let Err(e) = q.validate() {
        println!("{:}", e);
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                String::from("Invalid query parameters"),
                Some(e),
            )),
        ));
    }
    match state.user_use_case.list(state.db.clone(), q) {
        Ok(data) => Ok(Json(HttpResponse::new(
            StatusCode::OK,
            String::from("Success"),
            data.into_iter()
                .map(|user| UserResponse::new(user))
                .collect(),
        ))),
        Err(_) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                String::from("Invalid query parameters"),
                None,
            )),
        )),
    }
}

pub async fn create(
    State(state): State<Arc<AppState>>,
) -> Result<Json<String>, (StatusCode, String)> {
    match state.user_use_case.create() {
        Ok(d) => Ok(Json(d)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

pub async fn get_one(
    State(state): State<Arc<AppState>>,
) -> Result<Json<String>, (StatusCode, String)> {
    match state.user_use_case.get() {
        Ok(d) => Ok(Json(d)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

pub async fn delete_one(
    State(state): State<Arc<AppState>>,
) -> Result<Json<String>, (StatusCode, String)> {
    match state.user_use_case.delete() {
        Ok(d) => Ok(Json(d)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

pub async fn update_one(
    State(state): State<Arc<AppState>>,
) -> Result<Json<String>, (StatusCode, String)> {
    match state.user_use_case.update() {
        Ok(d) => Ok(Json(d)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

// Function to create user routes
pub fn user_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(list))
        .route("/", post(create))
        .route("/{id}", get(get_one))
        .route("/{id}", delete(delete_one))
        .route("/{id}", put(update_one))
        .with_state(state)
}
