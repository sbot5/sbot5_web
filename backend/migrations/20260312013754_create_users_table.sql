-- 创建用户表用于登录
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 使用 IF NOT EXISTS 确保多次运行不报错
ALTER TABLE posts ADD COLUMN IF NOT EXISTS author_id UUID REFERENCES users(id);
