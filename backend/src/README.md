# Rust 博客后端模块说明 (backend/src)

本项目采用 **Axum + SQLx + PostgreSQL** 构建，遵循典型的三层/模块化架构。

## 1. 核心模块功能描述

### 📂 `main.rs` (启动与配置)
- **入口函数:** 初始化 `tokio` 运行时。
- **配置加载:** 使用 `dotenvy` 加载 `.env` 环境变量（如 `DATABASE_URL`）。
- **日志系统:** 集成 `tracing-subscriber`，提供生产级别的异步日志记录。
- **连接池管理:** 创建并维护 `PgPool` 连接池，通过 Axum 的 `State` 注入到各个路由。

### 📂 `model.rs` (数据模型与校验)
- **`Post`:** 核心文章结构体，映射数据库表字段。
- **`CreatePost` / `UpdatePost`:** 专门用于处理请求体的 DTO（数据传输对象），集成了 `validator` 进行输入校验（如标题不能为空、长度限制等）。
- **`PostQuery`:** 定义了分页（limit, offset）、搜索（search）和状态过滤（published）的查询参数。

### 📂 `router.rs` (路由与业务逻辑)
- **路由定义:** 集中管理 `/posts` 下的所有路径。
- **CRUD 实现:**
  - `GET /posts`: 支持关键词模糊搜索和分页的文章列表。
  - `POST /posts`: 包含数据校验的新文章创建。
  - `GET /posts/:id`: 根据 UUID 获取单篇文章详情。
  - `PATCH /posts/:id`: 局部更新文章内容（使用 `COALESCE` SQL 函数）。
  - `DELETE /posts/:id`: 安全删除文章。

### 📂 `error.rs` (全局错误处理)
- **错误收敛:** 将数据库 `sqlx::Error`、校验错误和业务逻辑错误统一封装。
- **标准化响应:** 确保所有错误均以 `{"error": "消息"}` 的 JSON 格式返回给前端，并附带正确的 HTTP 状态码。

## 2. 前后端连接方案

### 跨域配置 (CORS)
由于 React 运行在 `5173` 端口，而后端运行在 `8000` 端口，必须在 `main.rs` 中配置 `tower-http` 的 CORS 插件。

### API 映射关系
| 前端功能 | 请求方法 | 后端路径 | 参数/Body |
| :--- | :--- | :--- | :--- |
| 文章列表页 | GET | `/posts` | `?search=xx&limit=10` |
| 文章详情页 | GET | `/posts/:id` | `id` (UUID) |
| 发布文章 | POST | `/posts` | `CreatePost` JSON |
| 编辑/隐藏文章 | PATCH | `/posts/:id` | `UpdatePost` JSON |
| 删除文章 | DELETE | `/posts/:id` | `id` (UUID) |

## 3. 待实现功能清单
- [ ] 用户认证 (JWT)
- [ ] 文章标签 (Tags) 与分类 (Categories)
- [ ] Markdown 渲染支持
- [ ] 评论系统
