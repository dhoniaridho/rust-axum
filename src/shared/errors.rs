// use axum::{
//     http::StatusCode,
//     response::{IntoResponse, Response},
//     Json,
// };
// // use serde::Serialize;

// // #[derive(Error, Debug)]
// // pub enum AppError {
// //     #[error("Validation Error: {0}")]
// //     Validation(String),

// //     #[error("Database Error: {0}")]
// //     DatabaseError(String),

// //     #[error("Unexpected Error: {0}")]
// //     Unexpected(String),
// // }

// // #[derive(Serialize)]
// // struct ErrorResponse {
// //     error: String,
// // // }

// // impl IntoResponse for AppError {
// //     fn into_response(self) -> Response {
// //         let (status, message) = match self {
// //             AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
// //             AppError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
// //             AppError::Unexpected(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
// //         };

// //         let body = Json(ErrorResponse { error: message });

// //         (status, body).into_response()
// //     }
// // }
