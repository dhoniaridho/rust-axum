use app::routes::create_routes;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

use shared::errors::handle_not_found;
use thiserror::Error;
mod adapters;
mod app;
mod domains;
mod infrasturcture;
mod schema;
mod shared;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .merge(create_routes());

    let app = app.fallback(axum::routing::any(|| async { handle_not_found() }));

    #[derive(Debug, Error)]
    pub enum ServerError {
        #[error(transparent)]
        ValidationError(#[from] validator::ValidationErrors),
        #[error(transparent)]
        BadRequest(#[from] axum::http::Error),
    }

    impl IntoResponse for ServerError {
        fn into_response(self) -> Response {
            match self {
                ServerError::ValidationError(_) => {
                    let message = format!("Input validation error: [{self}]").replace('\n', ", ");
                    (StatusCode::BAD_REQUEST, message)
                }
                ServerError::BadRequest(_) => {
                    let message = format!("Bad request: [{self}]").replace('\n', ", ");
                    (StatusCode::BAD_REQUEST, message)
                }
            }
            .into_response()
        }
    }

    // run our app with hyper, listening globally on port 3000
    match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Ok(l) => {
            println!("listening on http://localhost:3000");
            match axum::serve(l, app.into_make_service()).await {
                Ok(()) => println!("Server closed"),
                Err(e) => println!("Error: {}", e),
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
