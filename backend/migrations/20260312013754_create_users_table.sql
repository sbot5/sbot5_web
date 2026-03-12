-- 创建用户表用于登录
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 在 posts 表中关联用户 (可选，目前我们先全局管理文章)
ALTER TABLE posts ADD COLUMN author_id UUID REFERENCES users(id);
