use crate::error::error::GenericErrors;
use axum::body::Body;
use axum::extract::Request;
use axum::http::{HeaderValue, header};
use axum::middleware::Next;
use axum::response::Response;

pub async fn check_header(req: Request<Body>, next: Next) -> Result<Response, GenericErrors> {
    let headers = req.headers().get(header::CONTENT_TYPE);

    match headers {
        Some(headers) => {
            if headers.eq(&HeaderValue::from_static("application/json")) {
                Ok(next.run(req).await)
            } else {
                Err(GenericErrors::ContentTypeNotAllowed)
            }
        }
        _ => Err(GenericErrors::ContentTypeNotAllowed),
    }
}

