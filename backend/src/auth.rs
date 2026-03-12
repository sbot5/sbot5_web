use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::request::Parts,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{Utc, Duration};
use crate::error::AppError;

const JWT_SECRET: &[u8] = b"secret_key_change_me_in_production";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
}

/// 登录请求参数
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// 登录响应 (返回 Token)
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
}

pub fn create_token(user_id: Uuid) -> Result<String, AppError> {
    let exp = Utc::now() + Duration::days(1);
    let claims = Claims {
        sub: user_id,
        exp: exp.timestamp(),
    };
    
    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| AppError::Internal("Token creation failed".into()))
}

/// 核心：受保护的路由提取器
pub struct AuthenticatedUser(pub Uuid);

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 从 Header 中提取 Bearer Token
        let auth_header = parts.headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| AppError::ValidationError("Missing authorization header".into()))?;

        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::ValidationError("Invalid token format".into()));
        }

        let token = &auth_header[7..];
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(JWT_SECRET),
            &Validation::default(),
        ).map_err(|_| AppError::ValidationError("Invalid or expired token".into()))?;

        Ok(AuthenticatedUser(token_data.claims.sub))
    }
}
