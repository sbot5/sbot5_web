/**
 * Header 组件 — 全局顶部导航栏
 *
 * 包含：
 * - 左侧：博客名称（链接到首页）
 * - 右侧：主导航链接 + 登录/登出按钮
 * - 已登录时显示「写文章」快捷入口
 */

import { Link, useNavigate } from 'react-router-dom';
import { PenSquare, LogOut, LogIn, MessageSquare, Settings } from 'lucide-react';

interface HeaderProps {
  /** 博客标题，从博客设置中读取 */
  blogTitle: string;
  isLoggedIn: boolean;
  onLogout: () => void;
}

export function Header({ blogTitle, isLoggedIn, onLogout }: HeaderProps) {
  const navigate = useNavigate();

  const handleLogout = () => {
    onLogout();
    navigate('/');
  };

  return (
    <header className="site-header">
      <div className="header-inner">
        {/* 博客 Logo / 名称 */}
        <Link to="/" className="site-logo">
          {blogTitle || 'My Blog'}
        </Link>

        {/* 主导航 */}
        <nav className="header-nav">
          <Link to="/" className="nav-link">文章</Link>
          <Link to="/chat" className="nav-link">
            <MessageSquare size={16} />
            AI 助手
          </Link>

          {isLoggedIn ? (
            <>
              {/* 写文章按钮 */}
              <button
                className="btn btn-primary btn-sm"
                onClick={() => navigate('/new')}
              >
                <PenSquare size={15} />
                写文章
              </button>

              {/* 博客设置 */}
              <Link to="/admin" className="nav-link" title="博客设置">
                <Settings size={18} />
              </Link>

              {/* 登出 */}
              <button
                className="nav-link icon-btn"
                onClick={handleLogout}
                title="退出登录"
              >
                <LogOut size={18} />
              </button>
            </>
          ) : (
            <Link to="/login" className="nav-link" title="管理员登录">
              <LogIn size={18} />
              登录
            </Link>
          )}
        </nav>
      </div>
    </header>
  );
}
