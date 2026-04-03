/**
 * API 客户端模块
 *
 * 封装所有后端 HTTP 通信，提供类型化的函数接口。
 */

import type {
  Post,
  CreatePost,
  UpdatePost,
  PostQuery,
  PaginatedPosts,
  BlogSettings,
  AuthResponse,
  ChatResponse,
  Comment,
  CreateComment,
  UploadResponse,
} from '../types';

/** 后端地址（通过 Vite 环境变量覆盖） */
const API_BASE = import.meta.env.VITE_API_URL ?? 'http://localhost:3000';

/** 构造 JWT 认证请求头 */
const authHeader = (): Record<string, string> => {
  const token = localStorage.getItem('token');
  return token ? { Authorization: `Bearer ${token}` } : {};
};

/** 通用 fetch 封装：类型安全 + 统一错误处理 */
async function request<T>(path: string, init: RequestInit = {}): Promise<T> {
  const res = await fetch(`${API_BASE}${path}`, {
    ...init,
    headers: { 'Content-Type': 'application/json', ...init.headers },
  });

  if (!res.ok) {
    const body = await res.json().catch(() => ({}));
    throw new Error((body as { error?: string }).error ?? `HTTP ${res.status}`);
  }

  if (res.status === 204) return undefined as T;
  return res.json() as Promise<T>;
}

// ── 文章 ─────────────────────────────────────────────────

/** 获取文章列表（带分页） */
export async function fetchPosts(query: PostQuery = {}): Promise<PaginatedPosts> {
  const params = new URLSearchParams();
  if (query.search)    params.set('search', query.search);
  if (query.category)  params.set('category', query.category);
  if (query.tag)       params.set('tag', query.tag);
  if (query.published !== undefined) params.set('published', String(query.published));
  if (query.limit)     params.set('limit', String(query.limit));
  if (query.offset)    params.set('offset', String(query.offset));

  const qs = params.toString() ? `?${params}` : '';
  return request<PaginatedPosts>(`/posts${qs}`);
}

/** 按 UUID 获取文章 */
export async function fetchPost(id: string): Promise<Post> {
  return request<Post>(`/posts/${id}`);
}

/** 按 slug 获取文章 */
export async function fetchPostBySlug(slug: string): Promise<Post> {
  return request<Post>(`/post/${slug}`);
}

/** 创建文章（需登录） */
export async function createPost(data: CreatePost): Promise<Post> {
  return request<Post>('/admin/posts', {
    method: 'POST',
    headers: authHeader(),
    body: JSON.stringify(data),
  });
}

/** 更新文章（需登录） */
export async function updatePost(id: string, data: UpdatePost): Promise<Post> {
  return request<Post>(`/admin/posts/${id}`, {
    method: 'PATCH',
    headers: authHeader(),
    body: JSON.stringify(data),
  });
}

/** 删除文章（需登录） */
export async function deletePost(id: string): Promise<void> {
  return request<void>(`/admin/posts/${id}`, {
    method: 'DELETE',
    headers: authHeader(),
  });
}

// ── 评论 ─────────────────────────────────────────────────

/** 获取文章的已审核评论 */
export async function fetchComments(postId: string): Promise<Comment[]> {
  return request<Comment[]>(`/posts/${postId}/comments`);
}

/** 提交评论（游客，待审核） */
export async function createComment(postId: string, data: CreateComment): Promise<Comment> {
  return request<Comment>(`/posts/${postId}/comments`, {
    method: 'POST',
    body: JSON.stringify(data),
  });
}

/** 管理员查看所有评论 */
export async function fetchAllComments(status?: string): Promise<Comment[]> {
  const qs = status ? `?status=${status}` : '';
  return request<Comment[]>(`/admin/comments${qs}`, { headers: authHeader() });
}

/** 审核评论 */
export async function moderateComment(id: string, status: 'approved' | 'rejected'): Promise<Comment> {
  return request<Comment>(`/admin/comments/${id}`, {
    method: 'PATCH',
    headers: authHeader(),
    body: JSON.stringify({ status }),
  });
}

/** 删除评论 */
export async function deleteComment(id: string): Promise<void> {
  return request<void>(`/admin/comments/${id}`, {
    method: 'DELETE',
    headers: authHeader(),
  });
}

// ── 文件上传 ─────────────────────────────────────────────

/** 上传图片（需登录），使用 multipart/form-data */
export async function uploadFile(file: File): Promise<UploadResponse> {
  const formData = new FormData();
  formData.append('file', file);

  const token = localStorage.getItem('token');
  const res = await fetch(`${API_BASE}/api/upload`, {
    method: 'POST',
    headers: token ? { Authorization: `Bearer ${token}` } : {},
    body: formData, // 不设置 Content-Type，浏览器自动设置 multipart boundary
  });

  if (!res.ok) {
    const body = await res.json().catch(() => ({}));
    throw new Error((body as { error?: string }).error ?? `上传失败: HTTP ${res.status}`);
  }

  return res.json();
}

// ── 认证 ─────────────────────────────────────────────────

/** 登录 */
export async function login(username: string, password: string): Promise<AuthResponse> {
  return request<AuthResponse>('/auth/login', {
    method: 'POST',
    body: JSON.stringify({ username, password }),
  });
}

/** 注册（需邀请码） */
export async function register(
  username: string,
  password: string,
  inviteCode: string,
): Promise<void> {
  return request<void>('/auth/register', {
    method: 'POST',
    body: JSON.stringify({ username, password, invite_code: inviteCode }),
  });
}

// ── 设置 ─────────────────────────────────────────────────

export async function fetchSettings(): Promise<BlogSettings> {
  return request<BlogSettings>('/api/settings');
}

export async function updateSettings(data: BlogSettings): Promise<BlogSettings> {
  return request<BlogSettings>('/api/settings', {
    method: 'PATCH',
    headers: authHeader(),
    body: JSON.stringify(data),
  });
}

// ── AI 对话 ──────────────────────────────────────────────

export async function chatWithAI(message: string): Promise<ChatResponse> {
  return request<ChatResponse>('/api/chat', {
    method: 'POST',
    body: JSON.stringify({ message }),
  });
}
