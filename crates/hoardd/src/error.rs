use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

/// Error type for HTTP handlers: an `anyhow` error plus the status to report.
#[derive(Debug)]
pub(crate) struct AppError {
    pub(crate) status: StatusCode,
    pub(crate) err: anyhow::Error,
}

pub(crate) type ApiResult<T> = Result<T, AppError>;

impl AppError {
    pub(crate) fn not_found(what: impl std::fmt::Display) -> Self {
        Self { status: StatusCode::NOT_FOUND, err: anyhow::anyhow!("{what} not found") }
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
        Self { status: StatusCode::UNAUTHORIZED, err: anyhow::anyhow!("missing or invalid bearer token") }
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
        (self.status, Json(json!({ "error": format!("{:#}", self.err) }))).into_response()
    }
}
