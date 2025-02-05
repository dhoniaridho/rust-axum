use axum::{
    extract::{FromRequest, Request},
    http::StatusCode,
};
use serde_json::{json, Value};

use super::response::HttpResponse;

pub struct Qs<T>(pub T);

impl<S, T> FromRequest<S> for Qs<T>
where
    T: serde::de::DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<Value>);

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let (mut _parts, body) = req.into_parts();

        // We can use other extractors to provide better rejection messages.
        // For example, here we are using `axum::extract::MatchedPath` to
        // provide a better error message.
        //
        // Have to run that first since `Json` extraction consumes the request.

        let req = Request::from_parts(_parts, body);

        let qs = req.uri().query().unwrap_or_default();

        match serde_urlencoded::from_str::<T>(&qs) {
            Ok(parsed) => Ok(Qs(parsed).into()),
            Err(e) => Err((
                StatusCode::BAD_REQUEST,
                axum::Json(json!(HttpResponse::new(
                    StatusCode::BAD_REQUEST,
                    e.to_string(),
                    Some(Value::Null),
                ))),
            )),
        }
    }
}

pub struct BodyJson<T>(pub T);

impl<S, T> FromRequest<S> for BodyJson<T>
where
    T: serde::de::DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<Value>);

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let (mut _parts, body) = req.into_parts();

        // We can use other extractors to provide better rejection messages.
        // For example, here we are using `axum::extract::MatchedPath` to
        // provide a better error message.
        //
        let req = Request::from_parts(_parts, body);
        let bodyreq = req.into_body();

        // We can use other extractors to provide better rejection messages.
        // For example, here we are using `axum::extract::MatchedPath` to
        // provide a better error message.
        //
        let body = axum::body::to_bytes(bodyreq, 500000).await.unwrap();
        match serde_json::from_slice(&body) {
            Ok(parsed) => Ok(BodyJson(parsed).into()),
            Err(e) => Err((
                StatusCode::BAD_REQUEST,
                axum::Json(json!(HttpResponse::new(
                    StatusCode::BAD_REQUEST,
                    e.to_string(),
                    Some(Value::Null),
                ))),
            )),
        }
    }
}
