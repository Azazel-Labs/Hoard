use axum::Json;
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use serde_json::json;

use crate::i18n::tr;

/// Error type for HTTP handlers: an `anyhow` error plus the status to report.
#[derive(Debug)]
pub(crate) struct AppError {
    pub(crate) status: StatusCode,
    pub(crate) err: anyhow::Error,
}

pub(crate) type ApiResult<T> = Result<T, AppError>;

impl AppError {
    pub(crate) fn not_found(msg: impl std::fmt::Display) -> Self {
        Self { status: StatusCode::NOT_FOUND, err: anyhow::anyhow!("{msg}") }
    }
    pub(crate) fn bad_request(msg: impl std::fmt::Display) -> Self {
        Self { status: StatusCode::BAD_REQUEST, err: anyhow::anyhow!("{msg}") }
    }
    pub(crate) fn conflict(msg: impl std::fmt::Display) -> Self {
        Self { status: StatusCode::CONFLICT, err: anyhow::anyhow!("{msg}") }
    }
    pub(crate) fn unavailable(msg: impl std::fmt::Display) -> Self {
        Self { status: StatusCode::SERVICE_UNAVAILABLE, err: anyhow::anyhow!("{msg}") }
    }
    pub(crate) fn unprocessable(msg: impl std::fmt::Display) -> Self {
        Self { status: StatusCode::UNPROCESSABLE_ENTITY, err: anyhow::anyhow!("{msg}") }
    }
    pub(crate) fn unauthorized() -> Self {
        Self { status: StatusCode::UNAUTHORIZED, err: anyhow::anyhow!(tr!("error-sign-in-first")) }
    }
    pub(crate) fn forbidden(msg: impl std::fmt::Display) -> Self {
        Self { status: StatusCode::FORBIDDEN, err: anyhow::anyhow!("{msg}") }
    }
    pub(crate) fn too_many(msg: impl std::fmt::Display) -> Self {
        Self { status: StatusCode::TOO_MANY_REQUESTS, err: anyhow::anyhow!("{msg}") }
    }
}

impl<E: Into<anyhow::Error>> From<E> for AppError {
    fn from(e: E) -> Self {
        Self { status: StatusCode::INTERNAL_SERVER_ERROR, err: e.into() }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        if self.status.is_server_error() {
            tracing::error!(error = ?self.err, "request failed");
        }
        // The message is in the language the request asked for.
        let vary = [(header::VARY, HeaderValue::from_static("accept-language"))];
        (self.status, vary, Json(json!({ "error": format!("{:#}", self.err) }))).into_response()
    }
}
