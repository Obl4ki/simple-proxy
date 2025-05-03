use thiserror::Error;

use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;

#[derive(Error, Debug)]
pub enum ProxyApiError {
    #[error("Bad request")]
    BadRequest,
}

impl IntoResponse for ProxyApiError {
    fn into_response(self) -> Response {
        match self {
            ProxyApiError::BadRequest => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
        }
    }
}
