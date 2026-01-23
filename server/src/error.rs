use std::io::Error as IoError;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum ServerError {
    Database(sqlx::Error),
    Internal(IoError),
    NotFound(String),
}

#[derive(serde::Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let generic_error = (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong".to_string(),
        );

        tracing::error!(?self, "Error occurred");
        let (status_code, msg) = match self {
            ServerError::NotFound(s) => (StatusCode::NOT_FOUND, s),
            _ => generic_error,
        };

        let body = Json(ErrorResponse { error: msg });
        (status_code, body).into_response()
    }
}

impl From<sqlx::Error> for ServerError {
    fn from(err: sqlx::Error) -> Self {
        ServerError::Database(err)
    }
}

impl From<IoError> for ServerError {
    fn from(err: IoError) -> Self {
        ServerError::Internal(err)
    }
}
