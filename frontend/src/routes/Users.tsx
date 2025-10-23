import React, { useState, useEffect } from 'react';
import { usersAPI, WebSocketService, WebSocketMessage } from '../api';
import { User } from '../types';

export const Users: React.FC = () => {
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState(true);
  const [messages, setMessages] = useState<WebSocketMessage[]>([]);
  const [pagination, setPagination] = useState({
    page: 1,
    size: 10,
    total: 0,
    pages: 0
  });

  useEffect(() => {
    fetchUsers();
    setupWebSocket();
  }, [pagination.page]);

  const fetchUsers = async () => {
    try {
      setLoading(true);
      const response = await usersAPI.getUsers(pagination.page, pagination.size);
      setUsers(response.data.users);
      setPagination(prev => ({
        ...prev,
        ...response.data.pagination
      }));
    } catch (error) {
      console.error('Failed to fetch users:', error);
    } finally {
      setLoading(false);
    }
  };

  const setupWebSocket = () => {
    const wsService = new WebSocketService();
    
    wsService.connect((data) => {
      if (data.event === 'user_registered' || data.event === 'user_logged_in') {
        setMessages(prev => [data, ...prev.slice(0, 9)]);
        // 如果有新用户注册，刷新用户列表
        if (data.event === 'user_registered') {
          fetchUsers();
        }
      }
    });

    return () => {
      wsService.disconnect();
    };
  };

  const handleDeleteUser = async (id: string) => {
    if (!window.confirm('确定要删除这个用户吗？')) return;

    try {
      await usersAPI.deleteUser(id);
      setUsers(users.filter(user => user.id !== id));
      fetchUsers(); // 刷新列表
    } catch (error) {
      console.error('Failed to delete user:', error);
    }
  };

  const handlePageChange = (newPage: number) => {
    setPagination(prev => ({ ...prev, page: newPage }));
  };

  if (loading) {
    return (
      <div className="container">
        <div style={{ textAlign: 'center', padding: '50px' }}>
          加载中...
        </div>
      </div>
    );
  }

  return (
    <div className="container">
      <div className="card">
        <h2>用户管理</h2>
        
        <div className="user-list">
          {users.map(user => (
            <div key={user.id} className="user-item">
              <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                <div>
                  <h4>{user.username}</h4>
                  <p>邮箱: {user.email}</p>
                  <p>注册时间: {new Date(user.created_at).toLocaleString()}</p>
                </div>
                <button 
                  className="btn btn-secondary"
                  onClick={() => handleDeleteUser(user.id)}
                  style={{ marginLeft: '10px' }}
                >
                  删除
                </button>
              </div>
            </div>
          ))}
        </div>

        {/* 分页控件 */}
        {pagination.pages > 1 && (
          <div style={{ display: 'flex', justifyContent: 'center', gap: '10px', marginTop: '20px' }}>
            <button
              className="btn btn-secondary"
              disabled={pagination.page === 1}
              onClick={() => handlePageChange(pagination.page - 1)}
            >
              上一页
            </button>
            
            <span style={{ padding: '10px' }}>
              第 {pagination.page} 页，共 {pagination.pages} 页
            </span>
            
            <button
              className="btn btn-secondary"
              disabled={pagination.page === pagination.pages}
              onClick={() => handlePageChange(pagination.page + 1)}
            >
              下一页
            </button>
          </div>
        )}
      </div>

      {/* WebSocket 消息显示 */}
      <div className="card">
        <h3>实时事件</h3>
        <div className="websocket-messages">
          {messages.length === 0 ? (
            <p style={{ textAlign: 'center', color: '#666' }}>等待实时事件...</p>
          ) : (
            messages.map((message, index) => (
              <div key={index} className="message-item">
                <strong>{message.username}</strong> {message.event === 'user_registered' ? '注册了账号' : '登录了系统'}
                <div style={{ fontSize: '12px', color: '#666' }}>
                  {new Date(message.timestamp).toLocaleString()}
                </div>
              </div>
            ))
          )}
        </div>
      </div>
    </div>
  );
};