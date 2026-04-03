/**
 * Login 页面 — 管理员登录
 */

import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { LogIn, Eye, EyeOff } from 'lucide-react';
import * as api from '../api/client';

interface LoginProps {
  onLogin: (token: string) => void;
}

export function Login({ onLogin }: LoginProps) {
  const navigate = useNavigate();
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [showPwd, setShowPwd] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!username.trim() || !password) { setError('请填写账户名和密码'); return; }

    setLoading(true);
    setError(null);
    try {
      const { token } = await api.login(username.trim(), password);
      onLogin(token);
      navigate('/');
    } catch (err) {
      setError(err instanceof Error ? err.message : '登录失败');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="page-center">
      <div className="auth-card">
        <div className="auth-header">
          <LogIn size={28} className="auth-icon" />
          <h1 className="auth-title">管理员登录</h1>
          <p className="auth-subtitle">登录后可管理博客文章和评论</p>
        </div>

        {error && <div className="error-message" role="alert">{error}</div>}

        <form onSubmit={handleSubmit} className="auth-form">
          <div className="form-group">
            <label htmlFor="username" className="form-label">账户名</label>
            <input
              id="username" type="text" className="form-input" placeholder="请输入账户名"
              value={username} onChange={e => { setUsername(e.target.value); setError(null); }}
              autoComplete="username" autoFocus
            />
          </div>
          <div className="form-group">
            <label htmlFor="password" className="form-label">密码</label>
            <div className="input-with-icon">
              <input
                id="password" type={showPwd ? 'text' : 'password'} className="form-input" placeholder="请输入密码"
                value={password} onChange={e => { setPassword(e.target.value); setError(null); }}
                autoComplete="current-password"
              />
              <button type="button" className="input-icon-btn" onClick={() => setShowPwd(v => !v)} tabIndex={-1}>
                {showPwd ? <EyeOff size={16} /> : <Eye size={16} />}
              </button>
            </div>
          </div>
          <button type="submit" className="btn btn-primary btn-full" disabled={loading}>
            {loading ? '登录中...' : '登录'}
          </button>
        </form>
      </div>
    </div>
  );
}
