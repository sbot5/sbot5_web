# SBOT_WEB 全栈博客项目

这是一个基于 **Rust (Axum)** 和 **React (Vite)** 构建的现代化、简约风格博客系统。项目采用前后端分离架构，注重代码简洁性与高性能。

## 🌟 特性

- **简约 UI 设计**：采用 CSS Grid 布局，完美适配 Edge/Chrome 等现代浏览器。
- **高性能后端**：使用 Rust Axum 框架，异步 I/O 驱动，极致响应速度。
- **全栈 CRUD**：支持文章的创建、查看、编辑、删除（带认证保护）。
- **内容分类与标签**：支持文章归档，方便内容管理。
- **JWT 身份验证**：基于 JSON Web Token 的安全登录机制。
- **数据库自动迁移**：集成 SQLx 迁移，启动即自动更新数据库表结构。
- **Markdown 支持**：内置完整的 Markdown 渲染引擎，支持代码高亮与预览。

## 🛠️ 技术栈

### 后端 (Backend)
- **语言**: Rust 1.75+
- **框架**: [Axum](https://github.com/tokio-rs/axum)
- **数据库**: PostgreSQL (使用 [SQLx](https://github.com/launchbadge/sqlx))
- **认证**: JWT (jsonwebtoken), bcrypt (密码哈希)
- **校验**: validator

### 前端 (Frontend)
- **框架**: React 19 (TypeScript/JS)
- **构建工具**: Vite
- **图标库**: Lucide React
- **渲染**: React Markdown
- **样式**: 原生 CSS (Grid & Flexbox)

## 🚀 快速启动

### 1. 环境准备
- 安装 [Rust](https://www.rust-lang.org/)
- 安装 [Node.js](https://nodejs.org/)
- 准备一个 PostgreSQL 数据库

### 2. 后端配置
进入 `backend` 目录，创建并配置 `.env` 文件：
```bash
cd backend
cp .env.example .env # 如果没有 example，请手动创建
```
在 `.env` 中填入你的数据库连接串：
```env
DATABASE_URL=postgres://用户名:密码@localhost:5432/数据库名
```
运行后端：
```bash
cargo run
```

### 3. 前端启动
进入 `frontend` 目录：
```bash
cd frontend
npm install
npm run dev
```
访问 `http://localhost:5173` 即可看到你的博客。

## 📂 项目结构
```text
.
├── backend/                # Rust 后端源代码
│   ├── migrations/         # SQL 数据库迁移脚本
│   └── src/                # 业务逻辑代码
├── frontend/               # React 前端源代码
│   ├── src/api.js          # API 调用封装
│   └── src/App.jsx         # 前端路由与 UI 逻辑
└── README.md
```

## 📝 许可证
MIT License.
