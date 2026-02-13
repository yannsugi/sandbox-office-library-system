import axios from 'axios';

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080';

export const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Types
export interface Division {
  id: number;
  name: string;
}

export interface User {
  id: number;
  name: string;
  division_id: number;
}

export interface Book {
  id: number;
  title: string;
  division_id: number;
  borrowed_by_user_id: number | null;
}

export interface CreateDivisionRequest {
  name: string;
}

export interface CreateUserRequest {
  name: string;
  division_id: number;
}

export interface CreateBookRequest {
  title: string;
  division_id: number;
}

export interface BorrowBookRequest {
  user_id: number;
}

// API functions
export const divisionsApi = {
  create: (data: CreateDivisionRequest) =>
    api.post<Division>('/divisions', data),

  list: () =>
    api.get<Division[]>('/divisions'),
};

export const usersApi = {
  create: (data: CreateUserRequest) =>
    api.post<User>('/users', data),

  list: () =>
    api.get<User[]>('/users'),
};

export const booksApi = {
  create: (data: CreateBookRequest) =>
    api.post<Book>('/books', data),

  list: () =>
    api.get<Book[]>('/books'),

  get: (id: number) =>
    api.get<Book>(`/books/${id}`),

  borrow: (id: number, data: BorrowBookRequest) =>
    api.post<Book>(`/books/${id}/borrow`, data),

  return: (id: number) =>
    api.post<Book>(`/books/${id}/return`, {}),
};
