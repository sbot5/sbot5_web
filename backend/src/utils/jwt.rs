//! JWT 工具模块
//!
//! 从 `AppState` 中读取 JWT 密钥，不再每次读环境变量。
//! 提供 `AuthenticatedUser` Axum 提取器供受保护路由使用。

use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Utc, Duration};

use crate::AppState;
use crate::error::AppError;

/// Token 中的声明
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// 用户 UUID
    pub sub: Uuid,
    /// 过期时间（Unix 秒级时间戳）
    pub exp: i64,
}

/// 为指定用户生成有效期 24 小时的 JWT Token
pub fn create_token(user_id: Uuid, secret: &str) -> Result<String, AppError> {
    let exp = (Utc::now() + Duration::hours(24)).timestamp();
    let claims = Claims { sub: user_id, exp };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| {
        tracing::error!("JWT 编码失败: {:?}", e);
        AppError::Internal("Token 生成失败".into())
    })
}

/// 已认证用户提取器
///
/// 在路由参数中添加即可自动验证 JWT：
/// ```rust,ignore
/// async fn handler(user: AuthenticatedUser, ...) { ... }
/// ```
pub struct AuthenticatedUser(pub Uuid);

#[async_trait]
impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // 读取 Authorization 请求头
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("请先登录".into()))?;

        // 校验 "Bearer <token>" 格式
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("Authorization 格式错误".into()))?;

        // 验证签名和过期时间
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| {
            tracing::debug!("JWT 验证失败: {:?}", e);
            AppError::Unauthorized("登录已过期，请重新登录".into())
        })?;

        Ok(AuthenticatedUser(token_data.claims.sub))
    }
}
