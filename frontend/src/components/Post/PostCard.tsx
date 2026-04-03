/**
 * PostCard — 文章列表卡片
 * 支持封面图、标签数组、摘要
 */

import { Link } from 'react-router-dom';
import { Calendar, Clock, Tag } from 'lucide-react';
import type { Post } from '../../types';

interface PostCardProps {
  post: Post;
}

/** 估算阅读时间（中文约 300 字/分钟） */
function estimateReadTime(content: string): number {
  return Math.max(1, Math.ceil(content.length / 300));
}

/** 从 Markdown 提取纯文本摘要 */
function extractExcerpt(content: string): string {
  const plain = content
    .replace(/#+\s/g, '')
    .replace(/[*_`~>]/g, '')
    .replace(/\[([^\]]+)\]\([^)]+\)/g, '$1')
    .replace(/\n+/g, ' ')
    .trim();
  return plain.length > 120 ? `${plain.slice(0, 120)}…` : plain;
}

export function PostCard({ post }: PostCardProps) {
  const readTime = estimateReadTime(post.content);
  const excerpt = post.summary || extractExcerpt(post.content);
  const date = new Date(post.created_at).toLocaleDateString('zh-CN', {
    year: 'numeric', month: 'long', day: 'numeric',
  });

  /** 优先用 slug 构建链接 */
  const postUrl = post.slug ? `/post/${post.slug}` : `/post/${post.id}`;

  return (
    <article className="post-card">
      {/* 封面图 */}
      {post.cover_image && (
        <Link to={postUrl} className="post-card-cover">
          <img src={post.cover_image} alt={post.title} loading="lazy" />
        </Link>
      )}

      <div className="post-card-body">
        {post.category && (
          <span className="post-category">{post.category}</span>
        )}

        <Link to={postUrl} className="post-card-title">
          {post.title}
        </Link>

        <p className="post-card-excerpt">{excerpt}</p>

        <div className="post-card-meta">
          <span className="meta-item">
            <Calendar size={13} /> {date}
          </span>
          <span className="meta-item">
            <Clock size={13} /> {readTime} 分钟阅读
          </span>
          {post.tags.length > 0 && (
            <span className="meta-item">
              <Tag size={13} />
              {post.tags.slice(0, 3).join(' · ')}
            </span>
          )}
        </div>
      </div>
    </article>
  );
}
