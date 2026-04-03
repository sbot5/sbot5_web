-- 插入范例博客数据
INSERT INTO posts (title, content, published, category, tags) VALUES 
(
    '欢迎来到我的新博客：开启 Rust 与 React 的全栈之旅', 
    '# 为什么选择 Rust?

在构建这个博客时，我选择了 Rust 作为后端语言。Rust 的**内存安全**和**并发处理能力**让我感到非常惊艳。

## 技术栈清单
- **后端**: Axum + SQLx + PostgreSQL
- **前端**: React 19 + Vite
- **AI**: Gemini Pro Integration

这是我的第一篇博文，未来我将在这里分享更多关于系统编程和前端开发的见解。', 
    true, 
    '技术', 
    'Rust,React,全栈'
),
(
    '关于简约设计的思考', 
    '最近在重构博客界面，我一直在思考什么是好的设计。

> "Less is more." —— Mies van der Rohe

过度装饰往往会掩盖内容本身。这就是为什么我决定将 Logo 改为最简单的 `sbot5 Blog`。一个好的博客应该让读者专注于文字，而不是被花哨的动画分散注意力。', 
    true, 
    '设计', 
    '简约,UI/UX'
),
(
    '如何使用内置的 AI 助手提升写作效率', 
    '本站集成了 Google Gemini Pro AI。你可以在左侧菜单找到 **AI 助手**。

你可以尝试问它：
1. "帮我给这篇文章起一个更吸引人的标题"
2. "检查一下这段代码是否有逻辑错误"
3. "总结一下这篇文章的核心观点"

希望这个小工具能成为你创作路上的好帮手。', 
    true, 
    '指南', 
    'AI,Gemini'
)
ON CONFLICT DO NOTHING;
