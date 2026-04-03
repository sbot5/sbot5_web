/**
 * 全局 TypeScript 类型定义
 * 与后端 Rust 数据模型对应
 */

// ═══════════════════════════════════════════════════════════
// 文章
// ═══════════════════════════════════════════════════════════

/** 博客文章 */
export interface Post {
  id: string;
  title: string;
  content: string;
  published: boolean;
  category: string | null;
  /** 标签数组（后端 TEXT[]） */
  tags: string[];
  /** SEO 友好 URL 路径 */
  slug: string | null;
  /** 独立摘要 */
  summary: string | null;
  /** 封面图 URL */
  cover_image: string | null;
  created_at: string;
  updated_at: string;
  /** 定时发布时间 */
  published_at: string | null;
}

/** 创建文章请求 */
export interface CreatePost {
  title: string;
  content: string;
  category?: string;
  tags?: string[];
  summary?: string;
  cover_image?: string;
  slug?: string;
  published?: boolean;
  published_at?: string;
}

/** 更新文章请求 */
export interface UpdatePost {
  title?: string;
  content?: string;
  category?: string;
  tags?: string[];
  summary?: string;
  cover_image?: string;
  slug?: string;
  published?: boolean;
  published_at?: string;
}

/** 文章列表分页响应 */
export interface PaginatedPosts {
  items: Post[];
  total: number;
  offset: number;
  limit: number;
}

/** 文章列表查询参数 */
export interface PostQuery {
  search?: string;
  category?: string;
  tag?: string;
  published?: boolean;
  limit?: number;
  offset?: number;
}

// ═══════════════════════════════════════════════════════════
// 评论
// ═══════════════════════════════════════════════════════════

/** 评论 */
export interface Comment {
  id: string;
  post_id: string;
  author_name: string;
  content: string;
  status: 'pending' | 'approved' | 'rejected';
  created_at: string;
}

/** 提交评论请求 */
export interface CreateComment {
  author_name: string;
  author_email?: string;
  content: string;
}

// ═══════════════════════════════════════════════════════════
// 文件上传
// ═══════════════════════════════════════════════════════════

/** 上传成功响应 */
export interface UploadResponse {
  url: string;
  id: string;
  original_name: string;
  file_size: number;
}

// ═══════════════════════════════════════════════════════════
// 博客设置 & 认证
// ═══════════════════════════════════════════════════════════

/** 博客全局设置 */
export interface BlogSettings {
  blog_title: string;
  primary_color: string;
  secondary_color: string;
  notice: string;
}

/** 登录响应 */
export interface AuthResponse {
  token: string;
}

/** AI 对话响应 */
export interface ChatResponse {
  reply: string;
}
