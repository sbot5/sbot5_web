-- ============================================================
-- 统一增强迁移：覆盖 P0–P3 所有 schema 变更
-- ============================================================

-- ── P1: 文章表增强 ───────────────────────────────────────────

-- 1. 添加 updated_at 字段，默认与 created_at 相同
ALTER TABLE posts ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT NOW();

-- 2. 文章摘要（独立于正文，用于列表展示）
ALTER TABLE posts ADD COLUMN IF NOT EXISTS summary TEXT;

-- 3. 封面图 URL
ALTER TABLE posts ADD COLUMN IF NOT EXISTS cover_image TEXT;

-- 4. SEO slug（唯一，用于友好 URL，如 /post/my-first-blog）
ALTER TABLE posts ADD COLUMN IF NOT EXISTS slug VARCHAR(255) UNIQUE;

-- 5. tags 从 TEXT 转为 TEXT[]（PostgreSQL 数组类型）
--    先重命名旧列，创建新列，迁移数据，再删除旧列
ALTER TABLE posts RENAME COLUMN tags TO tags_old;
ALTER TABLE posts ADD COLUMN tags TEXT[] DEFAULT '{}';

UPDATE posts
SET tags = CASE
    WHEN tags_old IS NOT NULL AND tags_old != ''
    THEN string_to_array(REPLACE(tags_old, ' ', ''), ',')
    ELSE '{}'
END;

ALTER TABLE posts DROP COLUMN tags_old;

-- 6. 定时发布时间（P3）
ALTER TABLE posts ADD COLUMN IF NOT EXISTS published_at TIMESTAMPTZ;

-- 为已发布且无 published_at 的文章，回填为 created_at
UPDATE posts SET published_at = created_at WHERE published = true AND published_at IS NULL;

-- ── P1: 文件上传表 ──────────────────────────────────────────

CREATE TABLE IF NOT EXISTS uploads (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    -- 原始文件名
    original_name VARCHAR(255) NOT NULL,
    -- 存储路径（相对于 uploads 目录）
    file_path VARCHAR(500) NOT NULL,
    -- MIME 类型，如 image/png
    mime_type VARCHAR(100) NOT NULL,
    -- 文件大小（字节）
    file_size BIGINT NOT NULL,
    -- 上传者
    uploader_id UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ── P3: 评论表 ──────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    -- 所属文章
    post_id UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    -- 评论者昵称（游客无需登录）
    author_name VARCHAR(100) NOT NULL,
    -- 评论者邮箱（用于 Gravatar 头像，不公开展示）
    author_email VARCHAR(255),
    -- 评论内容
    content TEXT NOT NULL,
    -- 审核状态：pending / approved / rejected
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 评论按文章查询的索引
CREATE INDEX IF NOT EXISTS idx_comments_post_id ON comments(post_id);
-- 评论按状态过滤的索引
CREATE INDEX IF NOT EXISTS idx_comments_status ON comments(status);

-- ── 索引优化 ─────────────────────────────────────────────────

-- 文章列表按时间倒序查询优化
CREATE INDEX IF NOT EXISTS idx_posts_created_at ON posts(created_at DESC);
-- 文章按 slug 查询（唯一索引已在列定义时创建）
-- 文章按分类查询
CREATE INDEX IF NOT EXISTS idx_posts_category ON posts(category);
-- 文章按发布状态过滤
CREATE INDEX IF NOT EXISTS idx_posts_published ON posts(published);
-- GIN 索引支持 tags 数组高效查询（如 @> 包含操作）
CREATE INDEX IF NOT EXISTS idx_posts_tags ON posts USING GIN(tags);
