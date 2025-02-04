use crate::{
    adapters::http::user_controller::user_routes, infrasturcture::database::DB,
    shared::app::AppState,
};
use axum::Router;
use std::sync::Arc;

pub fn create_routes() -> Router {
    let db = Arc::new(DB::new());

    let state = Arc::new(AppState { db });
    Router::new().nest("/users", user_routes(state))
}
