import { useState, useEffect } from 'react';
import { BrowserRouter as Router, Routes, Route, Link, useNavigate, useParams } from 'react-router-dom';
import ReactMarkdown from 'react-markdown';
import { Plus, LogIn, LogOut, ChevronLeft } from 'lucide-react';
import './App.css';
import { fetchPosts, fetchPostDetail, createPost } from './api';

// --- 1. 首页组件 ---
function PostList() {
  const [posts, setPosts] = useState([]);
  useEffect(() => { fetchPosts().then(setPosts); }, []);

  return (
    <div className="posts-list">
      {posts.map(post => (
        <Link to={`/post/${post.id}`} key={post.id} className="post-card">
          <h2>{post.title}</h2>
          <p>{post.content.substring(0, 100)}...</p>
          <div className="post-meta">{new Date(post.created_at).toLocaleDateString()}</div>
        </Link>
      ))}
    </div>
  );
}

// --- 2. 详情页组件 (Markdown) ---
function PostDetail() {
  const { id } = useParams();
  const [post, setPost] = useState(null);
  useEffect(() => { fetchPostDetail(id).then(setPost); }, [id]);

  if (!post) return <div className="loading">Loading...</div>;

  return (
    <article className="post-full">
      <Link to="/" className="back-btn"><ChevronLeft size={20}/> Back</Link>
      <h1>{post.title}</h1>
      <div className="post-meta">{new Date(post.created_at).toLocaleDateString()}</div>
      <div className="markdown-body">
        <ReactMarkdown>{post.content}</ReactMarkdown>
      </div>
    </article>
  );
}

// --- 3. 编辑器组件 ---
function Editor() {
  const [title, setTitle] = useState('');
  const [content, setContent] = useState('');
  const navigate = useNavigate();

  const handleSave = async () => {
    const token = localStorage.getItem('token');
    if (!token) return alert('Please login first');
    
    await fetch('http://localhost:8000/posts', {
      method: 'POST',
      headers: { 
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token}`
      },
      body: JSON.stringify({ title, content, published: true })
    });
    navigate('/');
  };

  return (
    <div className="editor-container">
      <input 
        className="editor-title" 
        placeholder="Post Title" 
        value={title} 
        onChange={e => setTitle(e.target.value)} 
      />
      <div className="editor-main">
        <textarea 
          placeholder="Write in Markdown..." 
          value={content} 
          onChange={e => setContent(e.target.value)}
        />
        <div className="preview">
          <ReactMarkdown>{content}</ReactMarkdown>
        </div>
      </div>
      <button className="btn-primary" onClick={handleSave}>Publish</button>
    </div>
  );
}

// --- 核心 App 结构 ---
function App() {
  const [isLoggedIn, setIsLoggedIn] = useState(!!localStorage.getItem('token'));

  return (
    <Router>
      <div className="container">
        <header className="header">
          <Link to="/" className="logo"><h1>SBOT_WEB</h1></Link>
          <nav>
            {isLoggedIn ? (
              <>
                <Link to="/new" className="nav-icon"><Plus /></Link>
                <button onClick={() => { localStorage.removeItem('token'); setIsLoggedIn(false); }} className="nav-icon"><LogOut /></button>
              </>
            ) : (
              <Link to="/login" className="nav-icon"><LogIn /></Link>
            )}
          </nav>
        </header>

        <Routes>
          <Route path="/" element={<PostList />} />
          <Route path="/post/:id" element={<PostDetail />} />
          <Route path="/new" element={<Editor />} />
          <Route path="/login" element={<LoginView onLogin={() => setIsLoggedIn(true)} />} />
        </Routes>
      </div>
    </Router>
  );
}

// 简单登录视图
function LoginView({ onLogin }) {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const navigate = useNavigate();

  const handleLogin = async () => {
    const res = await fetch('http://localhost:8000/auth/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password })
    });
    const data = await res.json();
    if (data.token) {
      localStorage.setItem('token', data.token);
      onLogin();
      navigate('/');
    }
  };

  return (
    <div className="login-box">
      <input placeholder="Username" onChange={e => setUsername(e.target.value)} />
      <input type="password" placeholder="Password" onChange={e => setPassword(e.target.value)} />
      <button onClick={handleLogin}>Login</button>
    </div>
  );
}

export default App;
