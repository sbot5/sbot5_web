/**
 * Admin 页面 — 博客设置
 *
 * 管理员可修改：
 * - 博客标题
 * - 主页公告
 * - 主色调（影响全局 CSS 变量）
 * - 辅助色
 */

import { useState, useEffect } from 'react';
import { Save, RefreshCw } from 'lucide-react';
import { Loading } from '../components/UI/Loading';
import * as api from '../api/client';
import type { BlogSettings } from '../types';

export function Admin() {
  const [settings, setSettings] = useState<BlogSettings>({
    blog_title: '',
    primary_color: '#007aff',
    secondary_color: '#5856d6',
    notice: '',
  });

  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [saved, setSaved] = useState(false);

  /** 加载当前设置 */
  useEffect(() => {
    api.fetchSettings()
      .then(setSettings)
      .catch(err => setError(err instanceof Error ? err.message : '加载设置失败'))
      .finally(() => setLoading(false));
  }, []);

  /** 保存设置并刷新全局主题 */
  const handleSave = async () => {
    setSaving(true);
    setError(null);
    setSaved(false);

    try {
      const updated = await api.updateSettings(settings);
      setSettings(updated);
      setSaved(true);

      // 立即更新 CSS 变量，无需刷新页面
      document.documentElement.style.setProperty('--color-accent', updated.primary_color);
      document.documentElement.style.setProperty('--color-accent-2', updated.secondary_color);

      // 3 秒后隐藏成功提示
      setTimeout(() => setSaved(false), 3000);
    } catch (err) {
      setError(err instanceof Error ? err.message : '保存失败');
    } finally {
      setSaving(false);
    }
  };

  if (loading) return <Loading text="加载设置..." size="lg" />;

  return (
    <div className="page-admin">
      <div className="admin-card">
        <h1 className="admin-title">博客设置</h1>
        <p className="admin-subtitle">修改博客的基本信息和外观</p>

        {/* 错误 / 成功提示 */}
        {error && <div className="error-message" role="alert">{error}</div>}
        {saved && <div className="success-message" role="status">设置已保存！</div>}

        <div className="settings-form">
          {/* 基本信息 */}
          <section className="settings-section">
            <h2 className="settings-section-title">基本信息</h2>

            <div className="form-group">
              <label htmlFor="blog_title" className="form-label">博客标题</label>
              <input
                id="blog_title"
                type="text"
                className="form-input"
                placeholder="我的个人博客"
                value={settings.blog_title}
                onChange={e => setSettings({ ...settings, blog_title: e.target.value })}
                maxLength={100}
              />
            </div>

            <div className="form-group">
              <label htmlFor="notice" className="form-label">首页公告</label>
              <textarea
                id="notice"
                className="form-input form-textarea"
                placeholder="可以写一句欢迎语、近期更新或者站点说明..."
                value={settings.notice}
                onChange={e => setSettings({ ...settings, notice: e.target.value })}
                rows={3}
                maxLength={500}
              />
              <span className="form-hint">{settings.notice.length}/500</span>
            </div>
          </section>

          {/* 颜色设置 */}
          <section className="settings-section">
            <h2 className="settings-section-title">主题颜色</h2>

            <div className="color-pickers">
              <div className="form-group">
                <label htmlFor="primary_color" className="form-label">主色调</label>
                <div className="color-input-wrap">
                  <input
                    id="primary_color"
                    type="color"
                    className="color-input"
                    value={settings.primary_color}
                    onChange={e => setSettings({ ...settings, primary_color: e.target.value })}
                  />
                  <input
                    type="text"
                    className="form-input color-hex"
                    value={settings.primary_color}
                    onChange={e => setSettings({ ...settings, primary_color: e.target.value })}
                    maxLength={7}
                    placeholder="#007aff"
                  />
                </div>
              </div>

              <div className="form-group">
                <label htmlFor="secondary_color" className="form-label">辅助色</label>
                <div className="color-input-wrap">
                  <input
                    id="secondary_color"
                    type="color"
                    className="color-input"
                    value={settings.secondary_color}
                    onChange={e => setSettings({ ...settings, secondary_color: e.target.value })}
                  />
                  <input
                    type="text"
                    className="form-input color-hex"
                    value={settings.secondary_color}
                    onChange={e => setSettings({ ...settings, secondary_color: e.target.value })}
                    maxLength={7}
                    placeholder="#5856d6"
                  />
                </div>
              </div>
            </div>
          </section>

          {/* 操作按钮 */}
          <div className="settings-actions">
            <button
              className="btn btn-ghost btn-sm"
              onClick={() => window.location.reload()}
              title="重新加载设置"
            >
              <RefreshCw size={15} />
              重置
            </button>
            <button
              className="btn btn-primary"
              onClick={handleSave}
              disabled={saving}
            >
              <Save size={15} />
              {saving ? '保存中...' : '保存设置'}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
