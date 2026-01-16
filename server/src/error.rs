use std::io::Error as IoError;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum ServerError {
    DatabaseError(sqlx::Error),
    InternalError(IoError),
}

#[derive(serde::Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        //let (status_code, msg) = match self {
        //    ServerError::DatabaseError(_) => (
        //        StatusCode::INTERNAL_SERVER_ERROR,
        //        "Something went wrong".to_string(),
        //    ),
        //};
        let (status_code, msg) = (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong".to_string(),
        );
        let body = Json(ErrorResponse { error: msg });
        (status_code, body).into_response()
    }
}

impl From<sqlx::Error> for ServerError {
    fn from(err: sqlx::Error) -> Self {
        ServerError::DatabaseError(err)
    }
}

impl From<IoError> for ServerError {
    fn from(err: IoError) -> Self {
        ServerError::InternalError(err)
    }
}
