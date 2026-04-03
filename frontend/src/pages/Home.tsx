/**
 * Home 页面 — 文章列表 + 分页
 */

import { useState, useEffect, useCallback } from 'react';
import { Search, ChevronLeft, ChevronRight } from 'lucide-react';
import { PostCard } from '../components/Post/PostCard';
import { Loading } from '../components/UI/Loading';
import * as api from '../api/client';
import type { Post } from '../types';

const PAGE_SIZE = 10;

export function Home() {
  const [posts, setPosts] = useState<Post[]>([]);
  const [total, setTotal] = useState(0);
  const [page, setPage] = useState(0);         // 当前页（0-based）
  const [search, setSearch] = useState('');
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  /** 搜索防抖 */
  const [debouncedSearch, setDebouncedSearch] = useState('');
  useEffect(() => {
    const timer = setTimeout(() => {
      setDebouncedSearch(search);
      setPage(0); // 搜索时回到第一页
    }, 400);
    return () => clearTimeout(timer);
  }, [search]);

  const loadPosts = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const data = await api.fetchPosts({
        search: debouncedSearch || undefined,
        published: true,
        limit: PAGE_SIZE,
        offset: page * PAGE_SIZE,
      });
      setPosts(data.items);
      setTotal(data.total);
    } catch (err) {
      setError(err instanceof Error ? err.message : '加载失败');
    } finally {
      setLoading(false);
    }
  }, [debouncedSearch, page]);

  useEffect(() => { loadPosts(); }, [loadPosts]);

  const totalPages = Math.ceil(total / PAGE_SIZE);

  return (
    <div className="page-home">
      {/* 搜索栏 */}
      <div className="search-bar">
        <Search size={18} className="search-icon" />
        <input
          type="search"
          className="search-input"
          placeholder="搜索文章标题或内容..."
          value={search}
          onChange={e => setSearch(e.target.value)}
        />
      </div>

      {loading && <Loading text="正在加载文章..." />}
      {error && <div className="error-message" role="alert">{error}</div>}

      {!loading && !error && posts.length === 0 && (
        <div className="empty-state">
          <p>{search ? `没有找到与「${search}」相关的文章` : '暂时没有文章'}</p>
        </div>
      )}

      {!loading && !error && posts.length > 0 && (
        <>
          <div className="post-list">
            {posts.map(post => <PostCard key={post.id} post={post} />)}
          </div>

          {/* 分页控件 */}
          {totalPages > 1 && (
            <div className="pagination">
              <button
                className="btn btn-ghost btn-sm"
                disabled={page === 0}
                onClick={() => setPage(p => p - 1)}
              >
                <ChevronLeft size={16} /> 上一页
              </button>

              <span className="pagination-info">
                {page + 1} / {totalPages}
                <span className="pagination-total">（共 {total} 篇）</span>
              </span>

              <button
                className="btn btn-ghost btn-sm"
                disabled={page + 1 >= totalPages}
                onClick={() => setPage(p => p + 1)}
              >
                下一页 <ChevronRight size={16} />
              </button>
            </div>
          )}
        </>
      )}
    </div>
  );
}
