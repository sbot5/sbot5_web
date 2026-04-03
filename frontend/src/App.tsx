/**
 * App 根组件
 *
 * 职责：
 * 1. 路由配置（React Router v7）
 * 2. 全局状态：登录态 + 博客设置
 * 3. 布局骨架：Header + 主内容区 + Footer
 * 4. 将博客设置的主题色同步到 CSS 变量
 */

import { useEffect, useState } from 'react';
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';

import { useAuth } from './hooks/useAuth';
import { Header } from './components/Layout/Header';
import { Footer } from './components/Layout/Footer';
import { Loading } from './components/UI/Loading';

import { Home }       from './pages/Home';
import { PostDetail } from './pages/PostDetail';
import { Editor }     from './pages/Editor';
import { Login }      from './pages/Login';
import { Admin }      from './pages/Admin';
import { Chat }       from './pages/Chat';

import * as api from './api/client';
import type { BlogSettings } from './types';

/**
 * PrivateRoute — 仅登录用户可访问的路由守卫
 */
function PrivateRoute({
  isLoggedIn,
  children,
}: {
  isLoggedIn: boolean;
  children: React.ReactNode;
}) {
  if (!isLoggedIn) return <Navigate to="/login" replace />;
  return <>{children}</>;
}

function AppContent() {
  const { isLoggedIn, login, logout } = useAuth();

  const [settings, setSettings] = useState<BlogSettings | null>(null);
  const [settingsError, setSettingsError] = useState(false);

  /** 应用启动时加载博客设置，用于标题和主题色 */
  useEffect(() => {
    api.fetchSettings()
      .then(s => {
        setSettings(s);
        // 将后端存储的颜色写入 CSS 变量，实现动态主题
        document.documentElement.style.setProperty('--color-accent', s.primary_color);
        document.documentElement.style.setProperty('--color-accent-2', s.secondary_color);
        // 更新浏览器标签页标题
        if (s.blog_title) document.title = s.blog_title;
      })
      .catch(() => {
        // 设置加载失败时使用默认值继续渲染，不阻塞页面
        setSettingsError(true);
        setSettings({
          blog_title: 'My Blog',
          primary_color: '#007aff',
          secondary_color: '#5856d6',
          notice: '',
        });
      });
  }, []);

  // 首次加载设置前显示加载态
  if (!settings) {
    return <Loading text="正在初始化..." size="lg" />;
  }

  return (
    <div className="site-wrapper">
      {/* 顶部导航 */}
      <Header
        blogTitle={settings.blog_title}
        isLoggedIn={isLoggedIn}
        onLogout={logout}
      />

      {/* 首页公告条 */}
      {settings.notice && (
        <div className="notice-bar">
          <span>{settings.notice}</span>
        </div>
      )}

      {/* 设置加载失败的降级提示 */}
      {settingsError && (
        <div className="error-banner">
          博客设置加载失败，正在以默认配置运行
        </div>
      )}

      {/* 主内容区 */}
      <main className="site-main">
        <div className="main-container">
          <Routes>
            {/* 公开路由 */}
            <Route path="/"        element={<Home />} />
            <Route path="/post/:id"   element={<PostDetail isLoggedIn={isLoggedIn} />} />
            <Route path="/chat"    element={<Chat />} />
            <Route path="/login"   element={<Login onLogin={login} />} />

            {/* 受保护路由：需要登录 */}
            <Route path="/new" element={
              <PrivateRoute isLoggedIn={isLoggedIn}><Editor /></PrivateRoute>
            } />
            <Route path="/edit/:id" element={
              <PrivateRoute isLoggedIn={isLoggedIn}><Editor /></PrivateRoute>
            } />
            <Route path="/admin" element={
              <PrivateRoute isLoggedIn={isLoggedIn}><Admin /></PrivateRoute>
            } />

            {/* 404 回退 */}
            <Route path="*" element={<Navigate to="/" replace />} />
          </Routes>
        </div>
      </main>

      {/* 底部 */}
      <Footer blogTitle={settings.blog_title} />
    </div>
  );
}

/** 顶层导出：包裹 BrowserRouter */
export default function App() {
  return (
    <BrowserRouter>
      <AppContent />
    </BrowserRouter>
  );
}
