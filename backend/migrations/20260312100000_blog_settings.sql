-- 博客配置表
CREATE TABLE IF NOT EXISTS blog_settings (
    id SERIAL PRIMARY KEY,
    blog_title VARCHAR(100) DEFAULT 'My Blog',
    primary_color VARCHAR(20) DEFAULT '#00aeec',
    secondary_color VARCHAR(20) DEFAULT '#fb7299',
    notice TEXT DEFAULT '欢迎来到我的博客！',
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 初始化一条默认配置
INSERT INTO blog_settings (id, blog_title) VALUES (1, 'SBOT BLOG') ON CONFLICT (id) DO NOTHING;
