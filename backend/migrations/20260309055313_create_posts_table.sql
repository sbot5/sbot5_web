-- Add migration script here
-- 创建文章表
CREATE TABLE IF NOT EXISTS posts (
    -- 使用 UUID 作为唯一主键，更加安全且不易被遍历
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    -- 文章标题，不能为空
    title VARCHAR(255) NOT NULL,
    -- 文章正文内容，TEXT 类型不限制长度
    content TEXT NOT NULL,
    -- 是否已发布（布尔值），默认是 false（草稿）
    published BOOLEAN NOT NULL DEFAULT FALSE,
    -- 创建时间，默认取当前时间
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
