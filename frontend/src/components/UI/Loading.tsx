/**
 * Loading 组件 — 通用加载状态占位
 *
 * 用法：
 *   <Loading />              全屏居中
 *   <Loading text="加载中" />  自定义文字
 *   <Loading size="sm" />    小尺寸，用于内联场景
 */

interface LoadingProps {
  text?: string;
  size?: 'sm' | 'md' | 'lg';
}

export function Loading({ text = '加载中...', size = 'md' }: LoadingProps) {
  return (
    <div className={`loading-container loading-${size}`}>
      {/* CSS 动画加载圆圈，在 index.css 中定义 */}
      <span className="loading-spinner" aria-hidden="true" />
      <span className="loading-text">{text}</span>
    </div>
  );
}
