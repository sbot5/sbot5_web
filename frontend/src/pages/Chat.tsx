/**
 * Chat 页面 — AI 创作助手
 *
 * 通过 Gemini API 代理实现对话功能。
 * 使用场景：起标题、润色文字、构思文章大纲等创作辅助。
 */

import { useState, useRef, useEffect } from 'react';
import { Send, Bot, User } from 'lucide-react';
import * as api from '../api/client';

/** 对话消息结构 */
interface Message {
  role: 'user' | 'ai';
  content: string;
}

/** AI 的初始欢迎语 */
const WELCOME_MESSAGE: Message = {
  role: 'ai',
  content: '你好！我是 AI 创作助手。你可以让我帮你：\n- 起文章标题\n- 润色和改写文字\n- 构思文章大纲\n- 解释技术概念\n\n有什么我能帮到你的吗？',
};

export function Chat() {
  const [messages, setMessages] = useState<Message[]>([WELCOME_MESSAGE]);
  const [input, setInput] = useState('');
  const [loading, setLoading] = useState(false);

  /** 用于自动滚动到底部 */
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  /** 发送消息 */
  const handleSend = async () => {
    const text = input.trim();
    if (!text || loading) return;

    // 添加用户消息
    setMessages(prev => [...prev, { role: 'user', content: text }]);
    setInput('');
    setLoading(true);

    try {
      const { reply } = await api.chatWithAI(text);
      setMessages(prev => [...prev, { role: 'ai', content: reply }]);
    } catch (err) {
      setMessages(prev => [
        ...prev,
        { role: 'ai', content: 'AI 服务暂时不可用，请稍后重试。' },
      ]);
    } finally {
      setLoading(false);
    }
  };

  /** 按 Enter 发送（Shift+Enter 换行） */
  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  return (
    <div className="page-chat">
      {/* 页面标题 */}
      <div className="chat-header">
        <Bot size={22} />
        <div>
          <h1 className="chat-title">AI 创作助手</h1>
          <p className="chat-subtitle">由 Gemini 提供支持</p>
        </div>
      </div>

      {/* 消息列表 */}
      <div className="chat-messages">
        {messages.map((msg, i) => (
          <div
            key={i}
            className={`chat-message ${msg.role === 'user' ? 'user' : 'ai'}`}
          >
            {/* 角色头像 */}
            <div className="message-avatar">
              {msg.role === 'ai' ? <Bot size={16} /> : <User size={16} />}
            </div>

            {/* 消息气泡 */}
            <div className="message-bubble">
              {/* 保留换行符 */}
              {msg.content.split('\n').map((line, j) => (
                <span key={j}>
                  {line}
                  {j < msg.content.split('\n').length - 1 && <br />}
                </span>
              ))}
            </div>
          </div>
        ))}

        {/* AI 正在输入指示器 */}
        {loading && (
          <div className="chat-message ai">
            <div className="message-avatar"><Bot size={16} /></div>
            <div className="message-bubble typing-indicator">
              <span /><span /><span />
            </div>
          </div>
        )}

        {/* 滚动锚点 */}
        <div ref={bottomRef} />
      </div>

      {/* 输入区域 */}
      <div className="chat-input-area">
        <textarea
          className="chat-input"
          placeholder="输入消息... (Enter 发送，Shift+Enter 换行)"
          value={input}
          onChange={e => setInput(e.target.value)}
          onKeyDown={handleKeyDown}
          rows={1}
          disabled={loading}
        />
        <button
          className="btn btn-primary chat-send-btn"
          onClick={handleSend}
          disabled={loading || !input.trim()}
          title="发送"
        >
          <Send size={17} />
        </button>
      </div>
    </div>
  );
}
