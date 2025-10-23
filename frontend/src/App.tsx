import React from 'react';
import { Routes, Route, Navigate, Link, useNavigate } from 'react-router-dom';
import { Login } from './routes/Login';
import { Register } from './routes/Register';
import { Users } from './routes/Users';
import { authAPI } from './api';

const App: React.FC = () => {
  const navigate = useNavigate();
  const token = localStorage.getItem('auth_token');
  const user = localStorage.getItem('user');

  const handleLogout = () => {
    authAPI.logout();
    navigate('/login');
  };

  return (
    <div>
      {/* 导航栏 */}
      {token && (
        <nav className="navbar">
          <div className="nav-content">
            <h1>Rust FullStack App</h1>
            <div className="nav-links">
              <Link to="/users">用户管理</Link>
              {user && (
                <span>欢迎, {JSON.parse(user).username}</span>
              )}
              <button 
                onClick={handleLogout}
                className="btn btn-secondary"
                style={{ marginLeft: '10px' }}
              >
                退出登录
              </button>
            </div>
          </div>
        </nav>
      )}

      {/* 路由配置 */}
      <Routes>
        <Route 
          path="/login" 
          element={!token ? <Login /> : <Navigate to="/users" replace />} 
        />
        <Route 
          path="/register" 
          element={!token ? <Register /> : <Navigate to="/users" replace />} 
        />
        <Route 
          path="/users" 
          element={token ? <Users /> : <Navigate to="/login" replace />} 
        />
        <Route 
          path="/" 
          element={<Navigate to={token ? "/users" : "/login"} replace />} 
        />
      </Routes>
    </div>
  );
};

export default App;