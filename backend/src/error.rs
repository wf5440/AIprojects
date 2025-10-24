use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("JWT错误: {0}")]
    JwtError(String),
    
    #[error("认证错误: {0}")]
    AuthError(String),
    
    #[error("请求参数错误: {0}")]
    BadRequest(String),
    
    #[error("未找到资源")]
    NotFound,
    
    #[error("内部服务器错误")]
    InternalError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::JwtError(_) => (StatusCode::UNAUTHORIZED, "JWT error"),
            AppError::AuthError(_) => (StatusCode::UNAUTHORIZED, "Authentication error"),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, "Bad request"),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            AppError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
        };

        let body = Json(json!({
            "error": error_message,
            "message": self.to_string(),
        }));

        (status, body).into_response()
    }
}