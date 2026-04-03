/**
 * Editor 页面 — 文章编辑器（新建 / 编辑）
 *
 * 支持：标签数组、slug、摘要、封面图上传、分屏预览
 */

import { useState, useEffect, useRef } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import ReactMarkdown from 'react-markdown';
import { Save, Eye, EyeOff, Upload } from 'lucide-react';
import { Loading } from '../components/UI/Loading';
import * as api from '../api/client';
import type { CreatePost } from '../types';

export function Editor() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const isEditing = Boolean(id);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const [form, setForm] = useState<CreatePost>({
    title: '',
    content: '',
    category: '',
    tags: [],
    summary: '',
    cover_image: '',
    slug: '',
    published: true,
  });

  /** 标签输入框的临时文本（逗号分隔输入） */
  const [tagsInput, setTagsInput] = useState('');
  const [showPreview, setShowPreview] = useState(true);
  const [loading, setLoading] = useState(isEditing);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [uploading, setUploading] = useState(false);

  useEffect(() => {
    if (!id) return;
    api.fetchPost(id)
      .then(post => {
        setForm({
          title: post.title,
          content: post.content,
          category: post.category ?? '',
          tags: post.tags,
          summary: post.summary ?? '',
          cover_image: post.cover_image ?? '',
          slug: post.slug ?? '',
          published: post.published,
        });
        setTagsInput(post.tags.join(', '));
      })
      .catch(err => setError(err instanceof Error ? err.message : '加载失败'))
      .finally(() => setLoading(false));
  }, [id]);

  const handleSave = async () => {
    if (!form.title.trim()) { setError('请填写标题'); return; }
    if (!form.content.trim()) { setError('请填写内容'); return; }

    setSaving(true);
    setError(null);

    // 从输入框解析标签
    const tags = tagsInput.split(',').map(t => t.trim()).filter(Boolean);

    try {
      const data = { ...form, tags };
      if (isEditing && id) {
        await api.updatePost(id, data);
        navigate(`/post/${id}`);
      } else {
        const post = await api.createPost(data);
        navigate(`/post/${post.id}`);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : '保存失败');
    } finally {
      setSaving(false);
    }
  };

  /** 封面图上传 */
  const handleUpload = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    setUploading(true);
    try {
      const result = await api.uploadFile(file);
      setForm(prev => ({ ...prev, cover_image: result.url }));
    } catch (err) {
      setError(err instanceof Error ? err.message : '上传失败');
    } finally {
      setUploading(false);
    }
  };

  const update = (field: keyof CreatePost, value: string | boolean) => {
    setForm(prev => ({ ...prev, [field]: value }));
    setError(null);
  };

  if (loading) return <Loading text="加载文章..." size="lg" />;

  return (
    <div className="page-editor">
      {/* 工具栏 */}
      <div className="editor-toolbar">
        <h2 className="editor-title">{isEditing ? '编辑文章' : '写新文章'}</h2>
        <div className="editor-actions">
          <button className="btn btn-ghost btn-sm" onClick={() => setShowPreview(v => !v)}>
            {showPreview ? <EyeOff size={15} /> : <Eye size={15} />}
            {showPreview ? '隐藏预览' : '显示预览'}
          </button>
          <label className="toggle-label">
            <input type="checkbox" checked={form.published} onChange={e => update('published', e.target.checked)} />
            {form.published ? '发布' : '草稿'}
          </label>
          <button className="btn btn-secondary btn-sm" onClick={() => navigate(-1)}>取消</button>
          <button className="btn btn-primary btn-sm" onClick={handleSave} disabled={saving}>
            <Save size={15} /> {saving ? '保存中...' : '保存'}
          </button>
        </div>
      </div>

      {error && <div className="error-message" style={{ marginBottom: '1rem' }}>{error}</div>}

      {/* 元信息 */}
      <div className="editor-meta">
        <input
          className="editor-title-input"
          placeholder="文章标题..."
          value={form.title}
          onChange={e => update('title', e.target.value)}
          maxLength={255}
        />
        <div className="editor-meta-row">
          <input className="form-input" placeholder="分类" value={form.category} onChange={e => update('category', e.target.value)} />
          <input className="form-input" placeholder="标签（逗号分隔）" value={tagsInput} onChange={e => setTagsInput(e.target.value)} />
        </div>
        <div className="editor-meta-row">
          <input className="form-input" placeholder="自定义 slug（留空自动生成）" value={form.slug} onChange={e => update('slug', e.target.value)} />
          <div className="editor-upload-row">
            <input
              className="form-input"
              placeholder="封面图 URL"
              value={form.cover_image}
              onChange={e => update('cover_image', e.target.value)}
            />
            <button className="btn btn-ghost btn-sm" onClick={() => fileInputRef.current?.click()} disabled={uploading}>
              <Upload size={14} /> {uploading ? '上传中...' : '上传'}
            </button>
            <input ref={fileInputRef} type="file" accept="image/*" onChange={handleUpload} hidden />
          </div>
        </div>
        <textarea
          className="form-input form-textarea"
          placeholder="文章摘要（可选，留空将从正文截取）"
          value={form.summary}
          onChange={e => update('summary', e.target.value)}
          rows={2}
          style={{ marginTop: '0.5rem' }}
        />
      </div>

      {/* 编辑 + 预览 */}
      <div className={`editor-body ${showPreview ? 'split' : 'full'}`}>
        <div className="editor-pane">
          <div className="pane-label">Markdown 编辑</div>
          <textarea
            className="editor-textarea"
            placeholder="开始写作... 支持 Markdown"
            value={form.content}
            onChange={e => update('content', e.target.value)}
            spellCheck={false}
          />
        </div>
        {showPreview && (
          <div className="editor-pane">
            <div className="pane-label">预览</div>
            <div className="editor-preview markdown-body">
              {form.content ? <ReactMarkdown>{form.content}</ReactMarkdown> : <p className="preview-placeholder">预览区域</p>}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
