use crate::app::response::ApiResponse;
use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use axum_valid::ValidRejection;
use std::error::Error;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
  #[error("Server not found")]
  NotFound,
  #[error("Method not allowed")]
  MethodNotAllowed,
  #[error("Database error: {0}")]
  Database(#[from] sea_orm::DbErr),
  #[error("Query parameter error: {0}")]
  Query(#[from] QueryRejection),
  #[error("Path parameter error: {0}")]
  Path(#[from] PathRejection),
  #[error("Body parameter error: {0}")]
  Json(#[from] JsonRejection),
  #[error("Validation failed: {0}")]
  Validation(String),
  #[error("Password hash error: {0}")]
  Bcrypt(#[from] bcrypt::BcryptError),
  #[error("JWT error: {0}")]
  JWT(#[from] jsonwebtoken::errors::Error),
  #[error("Unauthenticated: {0}")]
  Unauthenticated(String),
  #[error("{0}")]
  Biz(String),
  #[error("Internal server error: {0}")]
  Internal(String),
  #[error("Anyhow error: {0}")]
  AnyhowError(#[from] anyhow::Error),
  #[error("Configuration error: {0}")]
  Config(#[from] config::ConfigError),
  #[error("Std error: {0}")]
  StdError(#[from] Box<dyn Error + Send + Sync>),
  #[error("IO error: {0}")]
  Io(#[from] std::io::Error),
}

impl From<ValidRejection<ApiError>> for ApiError {
  fn from(value: ValidRejection<ApiError>) -> Self {
    match value {
      ValidRejection::Valid(errors) => ApiError::Validation(errors.to_string()),
      ValidRejection::Inner(error) => error,
    }
  }
}

impl ApiError {
  pub fn status_code(&self) -> StatusCode {
    match self {
      ApiError::NotFound => StatusCode::NOT_FOUND,
      ApiError::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
      ApiError::Database(_)
      | ApiError::Bcrypt(_)
      | ApiError::Internal(_)
      | ApiError::AnyhowError(_)
      | ApiError::StdError(_)
      | ApiError::Io(_)
      | ApiError::Config(_) => StatusCode::INTERNAL_SERVER_ERROR,
      ApiError::Query(_) | ApiError::Path(_) | ApiError::Json(_) | ApiError::Validation(_) => {
        StatusCode::BAD_REQUEST
      }
      ApiError::JWT(_) | ApiError::Unauthenticated(_) => StatusCode::UNAUTHORIZED,
      ApiError::Biz(_) => StatusCode::OK,
    }
  }
}

impl IntoResponse for ApiError {
  fn into_response(self) -> Response {
    let status_code = self.status_code();
    let body = axum::Json(ApiResponse::<()>::err(self.to_string()));

    (status_code, body).into_response()
  }
}

impl From<ApiError> for Response {
  fn from(value: ApiError) -> Self {
    value.into_response()
  }
}

pub type ApiResult<T> = Result<T, ApiError>;
