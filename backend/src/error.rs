//! 应用错误类型模块
//!
//! 定义统一的 `AppError` 枚举，并实现 Axum 的 `IntoResponse`，
//! 使路由处理函数可以直接返回 `Result<_, AppError>`。
//!
//! # 错误映射
//! | 变体              | HTTP 状态码 | 说明                     |
//! |------------------|-------------|--------------------------|
//! | DatabaseError    | 500         | SQLx 数据库错误，隐藏细节 |
//! | ValidationError  | 400         | 输入校验失败              |
//! | NotFound         | 404         | 资源不存在                |
//! | Unauthorized     | 401         | 未登录或 Token 无效       |
//! | Internal         | 500         | 其他内部错误，隐藏细节    |

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use tracing::error;

/// 应用程序统一错误类型
pub enum AppError {
    /// SQLx 数据库操作失败
    DatabaseError(sqlx::Error),
    /// 请求参数校验失败（返回具体原因给客户端）
    ValidationError(String),
    /// 请求的资源不存在
    NotFound(String),
    /// 未登录或认证失败
    Unauthorized(String),
    /// 其他内部错误（不向客户端暴露细节）
    Internal(String),
}

/// 将 `AppError` 转换为 Axum HTTP 响应
///
/// 规则：
/// - 数据库错误和内部错误只记录日志，返回通用提示，不泄露内部细节
/// - 校验错误、未授权、未找到直接返回具体提示
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::DatabaseError(err) => {
                // 记录详细错误日志供运维排查，不向客户端暴露
                error!("数据库错误: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "数据库操作失败，请稍后重试".to_string())
            }
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg)        => (StatusCode::NOT_FOUND, msg),
            AppError::Unauthorized(msg)    => (StatusCode::UNAUTHORIZED, msg),
            AppError::Internal(msg) => {
                error!("内部错误: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "服务器内部错误，请稍后重试".to_string())
            }
        };

        // 统一的 JSON 错误响应体：{ "error": "..." }
        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}

/// 允许直接用 `?` 将 `sqlx::Error` 转换为 `AppError`
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err)
    }
}
