use anyhow::Result;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    Internal(anyhow::Error),
    BadRequest(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Internal(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(RestErrorWrapResponse {
                    errors: RestErrorResponse {
                        error_message: format!("{:?}", e),
                    },
                }),
            ),
            Self::BadRequest(e) => (
                StatusCode::BAD_REQUEST,
                Json(RestErrorWrapResponse {
                    errors: RestErrorResponse {
                        error_message: format!("{:?}", e),
                    },
                }),
            ),
        }
        .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        Self::Internal(value.into())
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Serialize)]
pub struct RestErrorWrapResponse {
    #[serde(rename = "errors")]
    errors: RestErrorResponse,
}

#[derive(Serialize)]
pub struct RestErrorResponse {
    #[serde(rename = "error-message")]
    error_message: String,
}
