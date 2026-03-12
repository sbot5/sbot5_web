import { useState, useEffect } from 'react';
import { BrowserRouter as Router, Routes, Route, Link, useNavigate, useParams, useLocation } from 'react-router-dom';
import ReactMarkdown from 'react-markdown';
import { Plus, LogIn, LogOut, ChevronLeft, Trash2, Edit, Calendar, Folder, Github, Mail } from 'lucide-react';
import './App.css';
import { fetchPosts, fetchPostDetail, createPost, login, register, deletePost, updatePost } from './api';

// --- 组件: 一言 ---
function Hitokoto() {
  const [text, setText] = useState('世界灿烂盛大，欢迎回家。');
  useEffect(() => {
    fetch('https://v1.hitokoto.cn')
      .then(res => res.json())
      .then(data => setText(data.hitokoto))
      .catch(() => {});
  }, []);
  return <div className="hitokoto" style={{fontSize: '0.85rem', color: '#888', fontStyle: 'italic', marginTop: '15px', borderTop: '1px solid #eee', paddingTop: '10px'}}>{text}</div>;
}

// --- 组件: 侧边栏 ---
function Sidebar() {
  return (
    <aside className="sidebar">
      <div className="sidebar-box">
        <img src="https://api.dicebear.com/7.x/notionists/svg?seed=Zixu" alt="avatar" className="author-avatar" />
        <h3 style={{margin: '0 0 10px 0', fontSize: '1.25rem'}}>Zixu</h3>
        <p style={{color: '#666', fontSize: '0.9rem', margin: '0'}}>很高兴能在这里与你相遇</p>
        <div style={{display: 'flex', justifyContent: 'center', gap: '20px', marginTop: '20px'}}>
          <Github size={20} color="#555" /> <Mail size={20} color="#555" />
        </div>
      </div>
      <div className="sidebar-box">
        <h4 style={{margin: '0 0 10px 0', textAlign: 'left', borderBottom: '1px solid #eee', paddingBottom: '8px'}}>站点公告</h4>
        <p style={{fontSize: '0.85rem', textAlign: 'left', color: '#666', lineHeight: '1.6'}}>欢迎来到我的个人主页。这里记录生活点滴与心情感悟，希望你在这里过得愉快。</p>
        <Hitokoto />
      </div>
    </aside>
  );
}

// --- 页面: 列表页 ---
function PostList() {
  const [posts, setPosts] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchPosts().then(setPosts).finally(() => setLoading(false));
  }, []);

  if (loading) return <div className="loading" style={{textAlign: 'center', padding: '50px'}}>正在加载内容...</div>;

  return (
    <div className="post-list">
      {posts.map((post, index) => (
        <article key={post.id} className="post-card" style={{ flexDirection: index % 2 === 0 ? 'row' : 'row-reverse' }}>
          <div className="post-cover">
            <img src={`https://picsum.photos/seed/${post.id}/800/600`} alt="cover" loading="lazy" />
          </div>
          <div className="post-info">
            <div style={{fontSize: '0.8rem', color: '#999', marginBottom: '8px', display: 'flex', alignItems: 'center', gap: '5px'}}>
              <Calendar size={14} /> {new Date(post.created_at).toLocaleDateString()}
              {post.category && <span style={{background: '#49b1f5', color: '#fff', padding: '2px 6px', borderRadius: '4px', fontSize: '0.7rem'}}>{post.category}</span>}
            </div>
            <h2 style={{margin: '0 0 12px 0', fontSize: '1.4rem'}}><Link to={`/post/${post.id}`} style={{textDecoration: 'none', color: 'inherit'}}>{post.title}</Link></h2>
            <p style={{color: '#777', fontSize: '0.9rem', margin: '0 0 15px 0', display: '-webkit-box', WebkitLineClamp: '2', WebkitBoxOrient: 'vertical', overflow: 'hidden'}}>{post.content.substring(0, 100)}...</p>
            <Link to={`/post/${post.id}`} style={{fontSize: '0.85rem', color: '#49b1f5', textDecoration: 'none', fontWeight: 'bold'}}>查看详情 »</Link>
          </div>
        </article>
      ))}
    </div>
  );
}

// --- 页面: 详情页 ---
function PostDetail({ isLoggedIn }) {
  const { id } = useParams();
  const [post, setPost] = useState(null);
  const navigate = useNavigate();

  useEffect(() => {
    fetchPostDetail(id).then(setPost);
  }, [id]);

  if (!post) return <div style={{textAlign: 'center', padding: '100px'}}>正在加载文章...</div>;

  return (
    <div className="post-full" style={{background: '#fff', padding: '40px', borderRadius: '12px', boxShadow: 'var(--shadow)'}}>
      <div style={{marginBottom: '30px', display: 'flex', justifyContent: 'space-between'}}>
        <Link to="/" style={{textDecoration: 'none', color: '#888', fontSize: '0.9rem'}}><ChevronLeft size={18} style={{verticalAlign: 'middle'}}/> 返回主页</Link>
        {isLoggedIn && (
          <div className="admin-actions" style={{display: 'flex', gap: '10px'}}>
            <button onClick={() => navigate(`/edit/${id}`)} style={{background: 'none', border: 'none', cursor: 'pointer', color: '#49b1f5'}}><Edit size={18}/></button>
            <button onClick={async () => { if(confirm('确定永久删除这篇文章吗？')) { await deletePost(id); navigate('/'); } }} style={{background: 'none', border: 'none', cursor: 'pointer', color: '#ff4d4f'}}><Trash2 size={18}/></button>
          </div>
        )}
      </div>
      <h1 style={{fontSize: '2.2rem', margin: '0 0 15px 0', lineHeight: '1.3'}}>{post.title}</h1>
      <div style={{color: '#aaa', fontSize: '0.85rem', marginBottom: '40px', borderBottom: '1px solid #eee', paddingBottom: '15px'}}>
        <span>日期: {new Date(post.created_at).toLocaleDateString()}</span>
        <span style={{marginLeft: '20px'}}>分类: {post.category || '默认'}</span>
      </div>
      <div className="markdown-body" style={{fontSize: '1.1rem', color: '#333'}}>
        <ReactMarkdown>{post.content}</ReactMarkdown>
      </div>
    </div>
  );
}

// --- 页面: 编辑器 ---
function Editor() {
  const { id } = useParams();
  const [form, setForm] = useState({ title: '', content: '', category: '', tags: '' });
  const navigate = useNavigate();

  useEffect(() => {
    if (id) fetchPostDetail(id).then(p => setForm({ title: p.title, content: p.content, category: p.category || '', tags: p.tags || '' }));
  }, [id]);

  const handleSave = async () => {
    id ? await updatePost(id, { ...form, published: true }) : await createPost({ ...form, published: true });
    navigate('/');
  };

  return (
    <div className="editor-container" style={{background: '#fff', padding: '30px', borderRadius: '12px', boxShadow: 'var(--shadow)'}}>
      <input 
        value={form.title} 
        onChange={e => setForm({...form, title: e.target.value})} 
        placeholder="请输入标题..." 
        style={{width: '100%', padding: '12px', fontSize: '1.5rem', border: 'none', borderBottom: '2px solid #eee', outline: 'none', marginBottom: '20px'}} 
      />
      <div style={{display: 'flex', gap: '15px', marginBottom: '20px'}}>
        <input placeholder="分类" value={form.category} onChange={e => setForm({...form, category: e.target.value})} style={{flex: 1, padding: '10px', borderRadius: '6px', border: '1px solid #ddd'}} />
        <input placeholder="标签" value={form.tags} onChange={e => setForm({...form, tags: e.target.value})} style={{flex: 1, padding: '10px', borderRadius: '6px', border: '1px solid #ddd'}} />
      </div>
      <div style={{display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '20px', minHeight: '500px'}}>
        <textarea 
          value={form.content} 
          onChange={e => setForm({...form, content: e.target.value})} 
          placeholder="在此输入正文 (支持 Markdown)..." 
          style={{padding: '15px', borderRadius: '8px', border: '1px solid #ddd', resize: 'vertical', fontSize: '1rem'}}
        />
        <div style={{padding: '15px', background: '#fcfcfc', border: '1px solid #eee', borderRadius: '8px', overflowY: 'auto'}}>
          <div className="markdown-body"><ReactMarkdown>{form.content}</ReactMarkdown></div>
        </div>
      </div>
      <div style={{marginTop: '25px', display: 'flex', justifyContent: 'flex-end', gap: '15px'}}>
        <button onClick={() => navigate(-1)} style={{padding: '10px 25px', borderRadius: '25px', border: '1px solid #ddd', background: '#fff', cursor: 'pointer'}}>取消</button>
        <button className="btn-primary" onClick={handleSave} style={{fontWeight: 'bold'}}>确认发布</button>
      </div>
    </div>
  );
}

function LoginView({ onLogin }) {
  const [isRegister, setIsRegister] = useState(false);
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const navigate = useNavigate();

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      if (isRegister) {
        await register(username, password);
        alert('注册成功'); setIsRegister(false);
      } else {
        const data = await login(username, password);
        localStorage.setItem('token', data.token);
        onLogin(); navigate('/');
      }
    } catch (err) { alert(err.message); }
  };

  return (
    <div className="login-container">
      <div style={{background: '#fff', padding: '40px', borderRadius: '15px', boxShadow: 'var(--shadow)', width: '100%', maxWidth: '380px'}}>
        <h2 style={{textAlign: 'center', margin: '0 0 30px 0'}}>{isRegister ? '欢迎加入' : '请先登录'}</h2>
        <form onSubmit={handleSubmit} style={{display: 'flex', flexDirection: 'column', gap: '20px'}}>
          <input placeholder="账户名" value={username} onChange={e => setUsername(e.target.value)} required style={{padding: '12px', borderRadius: '8px', border: '1px solid #eee', background: '#f9f9f9'}} />
          <input type="password" placeholder="密码" value={password} onChange={e => setPassword(e.target.value)} required style={{padding: '12px', borderRadius: '8px', border: '1px solid #eee', background: '#f9f9f9'}} />
          <button type="submit" className="btn-primary" style={{padding: '12px', fontSize: '1rem'}}>确认</button>
        </form>
        <button onClick={() => setIsRegister(!isRegister)} style={{width: '100%', background: 'none', border: 'none', color: '#49b1f5', marginTop: '20px', cursor: 'pointer', fontSize: '0.9rem'}}>
          {isRegister ? '已有账号？去登录' : '没有账号？点击注册'}
        </button>
      </div>
    </div>
  );
}

function App() {
  const [isLoggedIn, setIsLoggedIn] = useState(!!localStorage.getItem('token'));
  const { pathname } = useLocation();

  return (
    <>
      <header className="header">
        <Link to="/" style={{textDecoration: 'none'}}><h1 style={{margin: 0, fontSize: '1.4rem', color: '#49b1f5', fontWeight: 900}}>SBOT BLOG</h1></Link>
        <nav style={{display: 'flex', gap: '25px', alignItems: 'center'}}>
          <Link to="/" style={{textDecoration: 'none', color: '#444', fontWeight: 500}}>首页</Link>
          {isLoggedIn ? (
            <>
              <Link to="/new" style={{color: '#666'}}><Plus /></Link>
              <button onClick={() => { localStorage.removeItem('token'); setIsLoggedIn(false); }} style={{background: 'none', border: 'none', cursor: 'pointer', color: '#666'}}><LogOut /></button>
            </>
          ) : (
            <Link to="/login" style={{color: '#666'}}><LogIn /></Link>
          )}
        </nav>
      </header>

      {pathname === '/' && (
        <div className="hero">
          <h2 style={{fontSize: '3rem', margin: 0, letterSpacing: '2px'}}>Zixu's Space</h2>
          <p style={{marginTop: '15px', fontSize: '1.2rem', opacity: 0.9}}>记录当下，展望未来</p>
        </div>
      )}

      <main className="main-layout" style={{marginTop: pathname === '/' ? '0' : '80px'}}>
        <section className="content-area">
          <Routes>
            <Route path="/" element={<PostList />} />
            <Route path="/post/:id" element={<PostDetail isLoggedIn={isLoggedIn} />} />
            <Route path="/new" element={<Editor />} />
            <Route path="/edit/:id" element={<Editor />} />
            <Route path="/login" element={<LoginView onLogin={() => setIsLoggedIn(true)} />} />
          </Routes>
        </section>
        <Sidebar />
      </main>

      <footer style={{textAlign: 'center', padding: '50px 0', background: '#fff', borderTop: '1px solid #eee', marginTop: '50px', color: '#999', fontSize: '0.9rem'}}>
        <p>© 2026 SBOT BLOG | 愿你在这里度过美好的时光</p>
      </footer>
    </>
  );
}

export default function WrappedApp() {
  return <Router><App /></Router>;
}
