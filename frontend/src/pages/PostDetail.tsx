/**
 * PostDetail 页面 — 文章阅读 + 评论区
 */

import { useState, useEffect } from 'react';
import { useParams, useNavigate, Link } from 'react-router-dom';
import ReactMarkdown from 'react-markdown';
import { ArrowLeft, Calendar, Tag, Folder, Edit2, Trash2, Send, MessageCircle } from 'lucide-react';
import { Loading } from '../components/UI/Loading';
import * as api from '../api/client';
import type { Post, Comment } from '../types';

interface PostDetailProps {
  isLoggedIn: boolean;
}

export function PostDetail({ isLoggedIn }: PostDetailProps) {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();

  const [post, setPost] = useState<Post | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [deleting, setDeleting] = useState(false);

  // 评论相关状态
  const [comments, setComments] = useState<Comment[]>([]);
  const [commentName, setCommentName] = useState('');
  const [commentEmail, setCommentEmail] = useState('');
  const [commentContent, setCommentContent] = useState('');
  const [submitting, setSubmitting] = useState(false);
  const [commentMsg, setCommentMsg] = useState<string | null>(null);

  useEffect(() => {
    if (!id) return;
    setLoading(true);

    // 尝试按 slug 或 UUID 获取文章
    const isUuid = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(id);
    const fetchFn = isUuid ? api.fetchPost(id) : api.fetchPostBySlug(id);

    fetchFn
      .then(p => {
        setPost(p);
        // 加载评论
        api.fetchComments(p.id).then(setComments).catch(() => {});
      })
      .catch(err => setError(err instanceof Error ? err.message : '文章加载失败'))
      .finally(() => setLoading(false));

    window.scrollTo({ top: 0, behavior: 'smooth' });
  }, [id]);

  const handleDelete = async () => {
    if (!post || !window.confirm('确定删除这篇文章？')) return;
    setDeleting(true);
    try {
      await api.deletePost(post.id);
      navigate('/');
    } catch (err) {
      alert(err instanceof Error ? err.message : '删除失败');
    } finally {
      setDeleting(false);
    }
  };

  /** 提交评论 */
  const handleComment = async () => {
    if (!post || !commentName.trim() || !commentContent.trim()) return;
    setSubmitting(true);
    setCommentMsg(null);
    try {
      await api.createComment(post.id, {
        author_name: commentName.trim(),
        author_email: commentEmail.trim() || undefined,
        content: commentContent.trim(),
      });
      setCommentContent('');
      setCommentMsg('评论已提交，审核通过后将显示');
    } catch (err) {
      setCommentMsg(err instanceof Error ? err.message : '提交失败');
    } finally {
      setSubmitting(false);
    }
  };

  if (loading) return <Loading text="正在加载文章..." size="lg" />;

  if (error) {
    return (
      <div className="page-center">
        <div className="error-message">{error}</div>
        <Link to="/" className="btn btn-secondary" style={{ marginTop: '1rem' }}>返回首页</Link>
      </div>
    );
  }

  if (!post) return null;

  const date = new Date(post.created_at).toLocaleDateString('zh-CN', {
    year: 'numeric', month: 'long', day: 'numeric',
  });

  return (
    <article className="page-post-detail">
      {/* 工具栏 */}
      <div className="post-toolbar">
        <Link to="/" className="back-link"><ArrowLeft size={16} /> 返回列表</Link>
        {isLoggedIn && (
          <div className="post-actions">
            <button className="btn btn-ghost btn-sm" onClick={() => navigate(`/edit/${post.id}`)}>
              <Edit2 size={15} /> 编辑
            </button>
            <button className="btn btn-danger btn-sm" onClick={handleDelete} disabled={deleting}>
              <Trash2 size={15} /> {deleting ? '删除中...' : '删除'}
            </button>
          </div>
        )}
      </div>

      {/* 封面图 */}
      {post.cover_image && (
        <div className="post-cover">
          <img src={post.cover_image} alt={post.title} />
        </div>
      )}

      {/* 文章头部 */}
      <header className="post-header">
        {post.category && <span className="post-category post-category-lg">{post.category}</span>}
        <h1 className="post-title">{post.title}</h1>
        <div className="post-meta">
          <span className="meta-item"><Calendar size={14} /> {date}</span>
          {post.category && <span className="meta-item"><Folder size={14} /> {post.category}</span>}
        </div>
        {post.tags.length > 0 && (
          <div className="post-tags">
            <Tag size={13} />
            {post.tags.map(tag => <span key={tag} className="tag">{tag}</span>)}
          </div>
        )}
      </header>

      <hr className="post-divider" />

      {/* Markdown 正文 */}
      <div className="post-content markdown-body">
        <ReactMarkdown>{post.content}</ReactMarkdown>
      </div>

      {/* ── 评论区 ── */}
      <section className="comment-section">
        <h2 className="comment-section-title">
          <MessageCircle size={20} />
          评论 ({comments.length})
        </h2>

        {/* 已有评论 */}
        {comments.length > 0 ? (
          <div className="comment-list">
            {comments.map(c => (
              <div key={c.id} className="comment-item">
                <div className="comment-author">{c.author_name}</div>
                <div className="comment-date">
                  {new Date(c.created_at).toLocaleDateString('zh-CN')}
                </div>
                <div className="comment-body">{c.content}</div>
              </div>
            ))}
          </div>
        ) : (
          <p className="comment-empty">暂无评论，来说两句？</p>
        )}

        {/* 发表评论 */}
        <div className="comment-form">
          <h3 className="comment-form-title">发表评论</h3>
          <div className="comment-form-row">
            <input
              className="form-input"
              placeholder="你的昵称 *"
              value={commentName}
              onChange={e => setCommentName(e.target.value)}
              maxLength={100}
            />
            <input
              className="form-input"
              placeholder="邮箱（可选，不公开）"
              value={commentEmail}
              onChange={e => setCommentEmail(e.target.value)}
              type="email"
            />
          </div>
          <textarea
            className="form-input form-textarea"
            placeholder="写下你的想法..."
            value={commentContent}
            onChange={e => setCommentContent(e.target.value)}
            rows={4}
            maxLength={2000}
          />
          {commentMsg && (
            <div className={commentMsg.includes('失败') ? 'error-message' : 'success-message'}>
              {commentMsg}
            </div>
          )}
          <button
            className="btn btn-primary btn-sm"
            onClick={handleComment}
            disabled={submitting || !commentName.trim() || !commentContent.trim()}
          >
            <Send size={14} />
            {submitting ? '提交中...' : '提交评论'}
          </button>
        </div>
      </section>
    </article>
  );
}
