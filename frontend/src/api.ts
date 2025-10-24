import axios from 'axios';
import { LoginResponse, UsersResponse, WebSocketMessage } from './types';

const API_BASE = (import.meta as any).env?.VITE_API_BASE || 'http://localhost:8080';

export const api = axios.create({
  baseURL: API_BASE,
  timeout: 10000,
});

// 请求拦截器：自动添加 token
api.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('auth_token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

// 响应拦截器：处理 token 过期
api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      localStorage.removeItem('auth_token');
      localStorage.removeItem('user');
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);

export const authAPI = {
  register: (username: string, email: string, password: string) =>
    api.post('/register', { username, email, password }),

  login: (email: string, password: string) =>
    api.post<LoginResponse>('/login', { email, password }),

  logout: () => {
    localStorage.removeItem('auth_token');
    localStorage.removeItem('user');
  },
};

export const usersAPI = {
  getUsers: (page: number = 1, size: number = 10) =>
    api.get<UsersResponse>(`/users?page=${page}&size=${size}`),

  getUser: (id: string) =>
    api.get(`/users/${id}`),

  deleteUser: (id: string) =>
    api.delete(`/users/${id}`),
};

// WebSocket 连接管理
export class WebSocketService {
  private ws: WebSocket | null = null;
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;

  connect(onMessage: (data: any) => void) {
    const wsUrl = (import.meta as any).env?.VITE_WS_URL || 'ws://localhost:8080/ws';
    
    try {
      this.ws = new WebSocket(wsUrl);
      
      this.ws.onopen = () => {
        console.log('WebSocket connected');
        this.reconnectAttempts = 0;
      };

      this.ws.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          onMessage(data);
        } catch (error) {
          console.error('Error parsing WebSocket message:', error);
        }
      };

      this.ws.onclose = () => {
        console.log('WebSocket disconnected');
        this.attemptReconnect(onMessage);
      };

      this.ws.onerror = (error) => {
        console.error('WebSocket error:', error);
      };
    } catch (error) {
      console.error('WebSocket connection failed:', error);
    }
  }

  private attemptReconnect(onMessage: (data: any) => void) {
    if (this.reconnectAttempts < this.maxReconnectAttempts) {
      this.reconnectAttempts++;
      const delay = Math.min(1000 * this.reconnectAttempts, 10000);
      
      console.log(`Attempting to reconnect in ${delay}ms...`);
      
      setTimeout(() => {
        this.connect(onMessage);
      }, delay);
    }
  }

  disconnect() {
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
  }
}

export type { WebSocketMessage };