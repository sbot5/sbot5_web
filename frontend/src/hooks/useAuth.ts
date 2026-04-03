/**
 * useAuth Hook
 *
 * 管理用户登录状态：
 * - 读取 localStorage 中的 JWT token 判断是否已登录
 * - 提供 login / logout 方法
 * - 状态变更会触发组件重新渲染
 */

import { useState, useCallback } from 'react';

interface UseAuthReturn {
  isLoggedIn: boolean;
  /** 登录成功后调用，传入 token 并持久化到 localStorage */
  login: (token: string) => void;
  /** 登出，清除 localStorage 中的 token */
  logout: () => void;
}

export function useAuth(): UseAuthReturn {
  // 初始状态从 localStorage 读取，避免刷新页面后丢失登录态
  const [isLoggedIn, setIsLoggedIn] = useState<boolean>(
    () => Boolean(localStorage.getItem('token')),
  );

  const login = useCallback((token: string) => {
    localStorage.setItem('token', token);
    setIsLoggedIn(true);
  }, []);

  const logout = useCallback(() => {
    localStorage.removeItem('token');
    setIsLoggedIn(false);
  }, []);

  return { isLoggedIn, login, logout };
}
