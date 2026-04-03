/**
 * 应用入口文件
 * 挂载 React 根组件到 #root DOM 节点
 */

import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import './index.css';
import App from './App';

// 获取根 DOM 节点（index.html 中定义）
const rootElement = document.getElementById('root');
if (!rootElement) {
  throw new Error('无法找到 #root 元素，请检查 index.html');
}

createRoot(rootElement).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
