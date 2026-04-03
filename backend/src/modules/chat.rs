//! AI 对话路由（Gemini API 代理）
//!
//! 复用 AppState 中的 reqwest::Client 连接池，
//! 不再每次请求创建新客户端。

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::env;

use crate::AppState;
use crate::error::AppError;

#[derive(Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Serialize)]
pub struct ChatResponse {
    pub reply: String,
}

/// POST /api/chat
pub async fn chat_with_gemini(
    State(state): State<AppState>,
    Json(payload): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, AppError> {
    let api_key = env::var("GEMINI_API_KEY")
        .map_err(|_| AppError::Internal("GEMINI_API_KEY 未配置".into()))?;

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}",
        api_key
    );

    let body = serde_json::json!({
        "contents": [{ "parts": [{ "text": payload.message }] }]
    });

    // 复用共享 HTTP 客户端
    let response = state.http_client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Gemini 请求失败: {:?}", e);
            AppError::Internal("AI 服务请求失败".into())
        })?;

    let data: serde_json::Value = response.json().await
        .map_err(|e| {
            tracing::error!("Gemini 响应解析失败: {:?}", e);
            AppError::Internal("AI 响应解析失败".into())
        })?;

    let reply = data["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .unwrap_or("AI 暂时无法回复，请稍后重试。")
        .to_string();

    Ok(Json(ChatResponse { reply }))
}
