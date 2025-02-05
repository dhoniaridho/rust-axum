use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Router,
};
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

use crate::{
    domains::user::{
        dto::{
            request::{CreateUserBodyRequest, GetUserListRequest, UpdateUserBodyRequest},
            response::UserResponse,
        },
        repository::user_repository::UserRepository,
        use_case::user::UserUseCase,
    },
    shared::{
        app::AppState,
        dto::Paginated,
        extractor::{BodyJson, Qs},
        response::HttpResponse,
    },
};

pub async fn list(
    State(state): State<Arc<UserState>>,
    Qs(q): Qs<GetUserListRequest>,
) -> Result<
    Json<HttpResponse<Paginated<UserResponse>>>,
    (StatusCode, Json<HttpResponse<ValidationErrors>>),
> {
    if let Err(e) = q.validate() {
        println!("{:}", e);
        return Err(HttpResponse::err(
            StatusCode::BAD_REQUEST,
            String::from("Invalid query parameters"),
            Some(e),
        ));
    }
    match state.user_use_case.list(q) {
        Ok(data) => Ok(Json(HttpResponse::new(
            StatusCode::OK,
            String::from("Success"),
            Some(Paginated::new(
                data.data
                    .into_iter()
                    .map(|user| UserResponse::new(user))
                    .collect(),
                data.meta,
            )),
        ))),
        Err(_) => Err(HttpResponse::err(
            StatusCode::BAD_REQUEST,
            String::from("Invalid query parameters"),
            None,
        )),
    }
}

pub async fn create(
    State(state): State<Arc<UserState>>,
    BodyJson(body): BodyJson<CreateUserBodyRequest>,
) -> Result<Json<HttpResponse<UserResponse>>, (StatusCode, Json<HttpResponse<ValidationErrors>>)> {
    match body.validate() {
        Ok(_) => (),
        Err(e) => {
            return Err(HttpResponse::err(
                StatusCode::BAD_REQUEST,
                String::from("Invalid request body"),
                Some(e),
            ))
        }
    }
    match state.user_use_case.create(body) {
        Ok(d) => Ok(HttpResponse::ok(
            StatusCode::OK,
            String::from("Success"),
            Some(UserResponse::new(d)),
        )),
        Err(e) => Err(
            HttpResponse::err(
                StatusCode::BAD_REQUEST,
                String::from(e),
                None,
            ),
        ),
    }
}

pub async fn get_one(
    State(state): State<Arc<UserState>>,
    Path(id): Path<String>,
) -> Result<Json<HttpResponse<UserResponse>>, (StatusCode, Json<HttpResponse<Option<Value>>>)> {
    match Uuid::parse_str(&id) {
        Ok(uuid) => match state.user_use_case.get(uuid) {
            Ok(d) => Ok(HttpResponse::ok(
                StatusCode::OK,
                String::from("Success"),
                Some(UserResponse::new(d)),
            )),
            Err(e) => Err(HttpResponse::err(StatusCode::NOT_FOUND, e, None)),
        },
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(HttpResponse::new(
                StatusCode::BAD_REQUEST,
                e.to_string(),
                None,
            )),
        )),
    }
}

pub async fn delete_one(
    State(state): State<Arc<UserState>>,
    Path(id): Path<String>,
) -> Result<Json<HttpResponse<Value>>, (StatusCode, Json<HttpResponse<Option<Value>>>)> {
    match Uuid::parse_str(&id) {
        Ok(id) => match state.user_use_case.delete(id) {
            Ok(_) => Ok(HttpResponse::ok(
                StatusCode::OK,
                String::from("Success"),
                None,
            )),
            Err(e) => Err(HttpResponse::err(
                StatusCode::BAD_REQUEST,
                e.to_string(),
                None,
            )),
        },
        Err(e) => Err(HttpResponse::err(
            StatusCode::BAD_REQUEST,
            e.to_string(),
            None,
        )),
    }
}

pub async fn update_one(
    Path(id): Path<String>,
    State(state): State<Arc<UserState>>,
    Json(body): Json<UpdateUserBodyRequest>,
) -> Result<Json<HttpResponse<UserResponse>>, (StatusCode, String)> {
    match state
        .user_use_case
        .update(Uuid::parse_str(&id).unwrap(), body)
    {
        Ok(d) => Ok(HttpResponse::ok(
            StatusCode::OK,
            String::from("Success"),
            Some(UserResponse::new(d)),
        )),
        Err(e) => Err((StatusCode::BAD_REQUEST, e)),
    }
}

pub struct UserState {
    pub user_use_case: Arc<UserUseCase>,
}

// Function to create user routes
pub fn user_routes(state: Arc<AppState>) -> Router {
    let user_repository = Arc::new(UserRepository::new(Arc::clone(&state.db)));
    let user_use_case = Arc::new(UserUseCase::new(user_repository));
    let state = Arc::new(UserState { user_use_case });
    Router::new()
        .route("/", get(list))
        .route("/", post(create))
        .route("/{id}", get(get_one))
        .route("/{id}", delete(delete_one))
        .route("/{id}", put(update_one))
        .with_state(state)
}
