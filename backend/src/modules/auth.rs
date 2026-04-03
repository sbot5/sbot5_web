//! 认证路由处理模块
//!
//! - 注册需要邀请码（REGISTER_CODE 环境变量），未配置则关闭注册
//! - 登录返回 JWT Token
//! - 密码策略：8–20 位，字母+数字

use axum::{extract::State, http::StatusCode, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::Row;
use validator::Validate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;
use crate::error::AppError;
use crate::utils::jwt::create_token;

/// 注册请求体（比登录多一个邀请码字段）
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 20, message = "用户名 3-20 字符"))]
    pub username: String,
    #[validate(length(min = 8, max = 20, message = "密码 8-20 字符"))]
    pub password: String,
    /// 邀请码，必须与服务端 REGISTER_CODE 匹配
    pub invite_code: String,
}

/// 登录请求体
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1))]
    pub username: String,
    #[validate(length(min = 1))]
    pub password: String,
}

/// 登录成功响应
#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}

/// POST /auth/register
///
/// 注册管理员账户。必须提供正确的邀请码。
/// 如果 REGISTER_CODE 未配置，注册功能完全关闭。
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<StatusCode, AppError> {
    // 第一步：检查邀请码
    let expected_code = state.register_code.as_deref()
        .ok_or_else(|| AppError::ValidationError("注册功能已关闭".into()))?;

    if payload.invite_code != expected_code {
        return Err(AppError::ValidationError("邀请码无效".into()));
    }

    // 第二步：格式校验
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // 第三步：密码强度校验
    validate_password_strength(&payload.password)?;

    // 第四步：bcrypt 哈希
    let hashed = hash(&payload.password, DEFAULT_COST)
        .map_err(|_| AppError::Internal("密码加密失败".into()))?;

    // 第五步：写入数据库
    sqlx::query("INSERT INTO users (username, password_hash) VALUES ($1, $2)")
        .bind(&payload.username)
        .bind(hashed)
        .execute(&state.db)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(ref db_err) = e {
                if db_err.code().as_deref() == Some("23505") {
                    return AppError::ValidationError("用户名已被占用".into());
                }
            }
            AppError::DatabaseError(e)
        })?;

    Ok(StatusCode::CREATED)
}

/// POST /auth/login
///
/// 验证账户并返回 JWT Token。
/// 用户名不存在和密码错误返回相同提示，防止枚举攻击。
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = sqlx::query("SELECT id, password_hash FROM users WHERE username = $1")
        .bind(&payload.username)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::ValidationError("账户名或密码错误".into()))?;

    let user_id: Uuid = user.get("id");
    let stored_hash: String = user.get("password_hash");

    let valid = verify(&payload.password, &stored_hash).unwrap_or(false);
    if !valid {
        return Err(AppError::ValidationError("账户名或密码错误".into()));
    }

    let token = create_token(user_id, &state.jwt_secret)?;
    Ok(Json(AuthResponse { token }))
}

/// 密码强度校验：必须同时包含字母和数字
fn validate_password_strength(password: &str) -> Result<(), AppError> {
    let has_letter = password.chars().any(|c| c.is_alphabetic());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());

    if !has_letter || !has_digit {
        return Err(AppError::ValidationError(
            "密码必须同时包含字母和数字".into(),
        ));
    }
    Ok(())
}
