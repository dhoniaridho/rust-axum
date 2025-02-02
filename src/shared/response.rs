use axum::http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpResponse<T> {
    pub status_code: u16,
    pub message: String,
    pub data: T
}

impl<T> HttpResponse<T> {
    pub fn new(status_code: StatusCode, message: String, data: T) -> Self {
        HttpResponse {
            status_code: status_code.as_u16(),
            message,
            data
        }
    }
}