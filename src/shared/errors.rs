use axum::http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse<E> {
    status_code: u16,
    message: String,
    data: Option<E>,
}

impl<E> ErrorResponse<E> {
    pub fn new(status_code: StatusCode, message: String, data: Option<E>) -> Self {
        ErrorResponse {
            status_code: status_code.as_u16(),
            message,
            data,
        }
    }
}

// pub async fn bad_request(
//     req: Request<Body>,
//     next: Next,
// ) -> Result<axum::response::Response, (StatusCode, Json<ErrorResponse<Vec<String>>)> {
//     let res = next.run(req).await;
//     let status = res.status();
//     if status == StatusCode::BAD_REQUEST {
//         return Err((
//             StatusCode::BAD_REQUEST,
//             Json(ErrorResponse::new(
//                 StatusCode::BAD_REQUEST,
//                 String::from("Bad request"),
//                 res.extensions().get::<Vec<String>>().cloned(),
//             )),
//         ));
//     }
//     Ok(res)
// }
