/**
 * Footer 组件 — 全局底部
 *
 * 显示版权信息和可选的外链。
 * 保持简洁，避免喧宾夺主。
 */

interface FooterProps {
  /** 博客名称，用于版权声明 */
  blogTitle: string;
}

export function Footer({ blogTitle }: FooterProps) {
  const year = new Date().getFullYear();

  return (
    <footer className="site-footer">
      <div className="footer-inner">
        <span className="footer-copy">
          © {year} {blogTitle || 'My Blog'}. Powered by Rust + React.
        </span>
      </div>
    </footer>
  );
}
