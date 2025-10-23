export interface User {
  id: string;
  username: string;
  email: string;
  created_at: string;
}

export interface LoginResponse {
  token: string;
  user: User;
}

export interface Pagination {
  page: number;
  size: number;
  total: number;
  pages: number;
}

export interface UsersResponse {
  users: User[];
  pagination: Pagination;
}

export interface WebSocketMessage {
  event: string;
  user_id: string;
  username: string;
  timestamp: string;
}