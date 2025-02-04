use axum::{http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpResponse<T> {
    pub status_code: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T> HttpResponse<T> {
    pub fn new(status_code: StatusCode, message: String, data: Option<T>) -> Self {
        HttpResponse {
            status_code: status_code.as_u16(),
            message,
            data,
        }
    }

    pub fn err(
        status_code: StatusCode,
        message: String,
        data: Option<T>,
    ) -> (StatusCode, Json<HttpResponse<T>>) {
        (
            status_code,
            axum::response::Json(HttpResponse::new(status_code, message, data)),
        )
    }

    pub fn ok(status_code: StatusCode, message: String, data: Option<T>) -> Json<HttpResponse<T>> {
        axum::response::Json(HttpResponse::new(status_code, message, data))
    }
}
