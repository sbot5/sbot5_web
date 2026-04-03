//! 文件上传路由模块 (P1)
//!
//! 支持图片上传，存储到本地 uploads/ 目录。
//! 文件按年月归类存储，如 uploads/2026/04/abc123.png。
//!
//! 限制：
//! - 仅允许图片类型（JPEG、PNG、GIF、WebP）
//! - 单文件最大 5MB

use axum::{
    extract::{Multipart, State},
    Json,
};
use uuid::Uuid;

use crate::AppState;
use crate::error::AppError;
use crate::model::UploadResponse;
use crate::utils::jwt::AuthenticatedUser;

/// 允许的 MIME 类型白名单
const ALLOWED_TYPES: &[&str] = &[
    "image/jpeg",
    "image/png",
    "image/gif",
    "image/webp",
];

/// 最大文件大小：5MB
const MAX_SIZE: usize = 5 * 1024 * 1024;

/// POST /api/upload (需要 JWT)
///
/// 接受 multipart/form-data，字段名为 "file"。
/// 返回文件的公开访问 URL。
pub async fn upload_file(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, AppError> {
    // 读取 multipart 中第一个 "file" 字段
    let field = multipart
        .next_field()
        .await
        .map_err(|e| AppError::ValidationError(format!("文件读取失败: {}", e)))?
        .ok_or_else(|| AppError::ValidationError("请上传一个文件".into()))?;

    // 获取文件名和 MIME 类型
    let original_name = field
        .file_name()
        .unwrap_or("unknown")
        .to_string();

    let content_type = field
        .content_type()
        .unwrap_or("application/octet-stream")
        .to_string();

    // 校验 MIME 类型
    if !ALLOWED_TYPES.contains(&content_type.as_str()) {
        return Err(AppError::ValidationError(format!(
            "不支持的文件类型: {}，仅允许 JPEG/PNG/GIF/WebP",
            content_type
        )));
    }

    // 读取文件内容
    let data = field
        .bytes()
        .await
        .map_err(|e| AppError::ValidationError(format!("文件读取失败: {}", e)))?;

    // 校验文件大小
    if data.len() > MAX_SIZE {
        return Err(AppError::ValidationError(format!(
            "文件过大: {}MB，最大允许 5MB",
            data.len() / 1024 / 1024
        )));
    }

    // 生成存储路径：uploads/2026/04/uuid.ext
    let now = chrono::Utc::now();
    let sub_dir = format!("{}/{:02}", now.format("%Y"), now.format("%m"));
    let extension = mime_to_extension(&content_type);
    let file_id = Uuid::new_v4();
    let file_name = format!("{}.{}", file_id, extension);
    let relative_path = format!("{}/{}", sub_dir, file_name);

    // 创建子目录并写入文件
    let full_dir = format!("{}/{}", state.upload_dir, sub_dir);
    tokio::fs::create_dir_all(&full_dir)
        .await
        .map_err(|e| AppError::Internal(format!("创建目录失败: {}", e)))?;

    let full_path = format!("{}/{}", full_dir, file_name);
    tokio::fs::write(&full_path, &data)
        .await
        .map_err(|e| AppError::Internal(format!("文件写入失败: {}", e)))?;

    // 记录到数据库
    let upload = sqlx::query_as::<_, crate::model::Upload>(
        "INSERT INTO uploads (original_name, file_path, mime_type, file_size, uploader_id)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id, original_name, file_path, mime_type, file_size, created_at",
    )
    .bind(&original_name)
    .bind(&relative_path)
    .bind(&content_type)
    .bind(data.len() as i64)
    .bind(user.0)
    .fetch_one(&state.db)
    .await?;

    // 返回可访问的 URL
    let url = format!("/uploads/{}", relative_path);

    Ok(Json(UploadResponse {
        url,
        id: upload.id,
        original_name: upload.original_name,
        file_size: upload.file_size,
    }))
}

/// 从 MIME 类型推断文件扩展名
fn mime_to_extension(mime: &str) -> &str {
    match mime {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/gif" => "gif",
        "image/webp" => "webp",
        _ => "bin",
    }
}
