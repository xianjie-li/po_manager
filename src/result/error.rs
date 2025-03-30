use axum::response::{IntoResponse, Response};

use super::response::AppResponse;

/// 应用通用错误类型
pub struct AppError(pub anyhow::Error);

// 支持 AppError 作为响应
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let res = AppResponse::<()>::err(self.0.to_string());
        res.into_response()
    }
}

// AppError 转换 anyhow::Error, 用于支持 ? 操作符
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
