use anyhow::Result;
use axum::{
    Json,
    extract::Request,
    http::HeaderValue,
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use http_body_util::BodyExt;

use super::{error::AppError, response_code::AppResponseCode};

/// 大部分场景可以使用它作为 handler 响应
pub type AppResult = Result<Response, AppError>;

/// 应用统一响应对象
///
/// - 支持作为 handler 返回, 通过 into_response()
/// - 支持 ::ok ::err 等方法边界的构造 [AppResult], 也可以通过 ::new().msg().data().build() 这样的方式来自定义构造
#[derive(Debug, Serialize, Deserialize)]
pub struct AppResponse<T: Serialize> {
    pub code: AppResponseCode,
    pub msg: String,
    pub data: Option<T>,
}

// 支持 AppResponse 作为响应
impl<T: Serialize> IntoResponse for AppResponse<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

// 为 [AppResponse]实现一些便利工具方法, 用于快速构造响应
impl<T: Serialize> AppResponse<T> {
    pub fn new() -> Self {
        AppResponse {
            code: AppResponseCode::Ok,
            msg: "".to_string(),
            data: None,
        }
    }

    pub fn ok(data: T) -> AppResult {
        Ok(AppResponse::<T> {
            code: AppResponseCode::Ok,
            msg: "".to_string(),
            data: Some(data),
        }
        .into_response())
    }

    /// 使用默认错误码和给定消息构造响应
    pub fn err(msg: impl Into<String>) -> AppResult {
        Ok(AppResponse::<()> {
            code: AppResponseCode::Err,
            msg: msg.into(),
            data: None,
        }
        .into_response())
    }

    pub fn code(mut self, code: AppResponseCode) -> Self {
        self.code = code;
        self
    }

    pub fn msg(mut self, msg: impl Into<String>) -> Self {
        self.msg = msg.into();
        self
    }

    #[allow(dead_code)]
    pub fn data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    #[allow(dead_code)]
    pub fn build(self) -> AppResult {
        Ok(self.into_response())
    }
}

/// 将所有 text/plain 响应转换为 AppResponse 响应
pub async fn text_response_process(request: Request, next: Next) -> Response {
    let response = next.run(request).await;

    if let Some(content_type) = response.headers().get("Content-Type") {
        if content_type.to_str().unwrap() != "text/plain; charset=utf-8" {
            return response;
        }

        let is_ok: bool =
            !response.status().is_server_error() && !response.status().is_client_error();

        let (mut parts, body) = response.into_parts();

        let bytes = match body.collect().await {
            Ok(collected) => collected.to_bytes(),
            Err(err) => {
                return AppResponse::<()>::new()
                    .code(AppResponseCode::Err)
                    .msg(format!("提取body内容失败: {}", err))
                    .into_response();
            }
        };

        let code = if is_ok {
            AppResponseCode::Ok
        } else {
            AppResponseCode::Err
        };

        parts.headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
        );

        Response::from_parts(
            parts,
            AppResponse::<()>::new()
                .code(code)
                .msg(String::from_utf8_lossy(&bytes.to_vec()))
                .into_response()
                .into_body(),
        )
    } else {
        response
    }
}
