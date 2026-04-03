# CLAUDE.md

This file provides guidance to Claude Code when working with this repository.

## Project Overview

全栈个人博客系统：Rust/Axum 后端 + React/TypeScript 前端。

核心功能：
- 文章 CRUD（Markdown 编辑器 + 实时预览）
- JWT 认证 + 邀请码注册
- AI 创作助手（Gemini 代理）
- 评论系统（游客提交、管理员审核）
- 图片上传（本地存储）
- RSS Feed + Sitemap（SEO）
- 博客设置（标题、主题色、公告）

## Development Commands

### Backend (Rust/Axum)
```bash
cd backend
cargo run           # 启动服务 localhost:3000
cargo check         # 编译检查
cargo build         # 构建
cargo test          # 测试
cargo fmt           # 格式化
cargo clippy        # Lint
```

需要 `.env` 文件（参考 `.env.example`），必须配置：DATABASE_URL、JWT_SECRET。

### Frontend (React/TypeScript/Vite)
```bash
cd frontend
npm install         # 安装依赖
npm run dev         # 开发服务 localhost:5173
npm run build       # 生产构建（tsc + vite build）
npm run lint        # ESLint
```

## Architecture

### Backend (`backend/src/`)
- **`main.rs`** — 入口：AppState 构建、DB 连接池、CORS、路由挂载
- **`error.rs`** — 统一错误类型 AppError（DatabaseError/ValidationError/NotFound/Unauthorized/Internal）
- **`model.rs`** — 数据模型：Post、Comment、Upload、PaginatedPosts
- **`modules/mod.rs`** — 路由注册总线
- **`modules/post.rs`** — 文章 CRUD + slug 路由 + 分页
- **`modules/auth.rs`** — 注册（需邀请码）/登录
- **`modules/chat.rs`** — AI 对话（Gemini 代理）
- **`modules/comment.rs`** — 评论 CRUD + 审核
- **`modules/upload.rs`** — 图片上传（multipart）
- **`modules/settings.rs`** — 博客设置
- **`modules/feed.rs`** — RSS Feed 生成
- **`modules/sitemap.rs`** — Sitemap XML 生成
- **`utils/jwt.rs`** — JWT 创建/验证 + AuthenticatedUser 提取器

### Frontend (`frontend/src/`)
- **`types/index.ts`** — TypeScript 类型定义
- **`api/client.ts`** — 类型化 API 客户端
- **`hooks/useAuth.ts`** — 登录状态管理
- **`components/`** — Layout（Header/Footer）、Post（PostCard）、UI（Loading）
- **`pages/`** — Home、PostDetail、Editor、Login、Admin、Chat
- **`App.tsx`** — 路由配置 + 全局状态
- **`index.css`** — 设计系统（CSS 变量驱动）

### Database
PostgreSQL + SQLx。迁移文件在 `backend/migrations/`：
- posts（文章）：title, content, tags TEXT[], slug, summary, cover_image, published_at
- users（管理员）
- comments（评论）：author_name, content, status(pending/approved/rejected)
- uploads（上传记录）
- blog_settings（全局配置）

## API Routes

### Public
| Method | Path | Purpose |
|--------|------|---------|
| GET | /posts | 文章列表（分页+搜索+标签过滤） |
| GET | /posts/:id | 按 UUID 获取文章 |
| GET | /post/:slug | 按 slug 获取文章 |
| GET | /posts/:id/comments | 文章评论列表（已审核） |
| POST | /posts/:id/comments | 提交评论（待审核） |
| POST | /auth/register | 注册（需邀请码） |
| POST | /auth/login | 登录 |
| POST | /api/chat | AI 对话 |
| GET | /api/settings | 博客设置 |
| GET | /feed.xml | RSS Feed |
| GET | /sitemap.xml | Sitemap |
| GET | /uploads/* | 静态文件 |

### Admin (JWT Required)
| Method | Path | Purpose |
|--------|------|---------|
| POST | /admin/posts | 创建文章 |
| PATCH | /admin/posts/:id | 更新文章 |
| DELETE | /admin/posts/:id | 删除文章 |
| GET | /admin/comments | 所有评论 |
| PATCH | /admin/comments/:id | 审核评论 |
| DELETE | /admin/comments/:id | 删除评论 |
| POST | /api/upload | 上传图片 |
| PATCH | /api/settings | 更新设置 |

## Key Environment Variables
| Variable | Required | Description |
|----------|----------|-------------|
| DATABASE_URL | Yes | PostgreSQL connection |
| JWT_SECRET | Yes | JWT signing key (≥32 bytes) |
| REGISTER_CODE | No | Invite code (unset = registration disabled) |
| CORS_ORIGIN | No | Allowed origins (default: localhost:5173) |
| GEMINI_API_KEY | No | AI chat feature |
| HOST | No | Bind address (default: 127.0.0.1) |
| PORT | No | Bind port (default: 3000) |
