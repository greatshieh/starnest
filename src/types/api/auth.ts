/**
 * 认证相关 API 类型定义
 * @module api/auth
 */

/**
 * 登录请求参数
 */
export interface LoginRequest {
  token: string
}

/**
 * 登录响应数据
 */
export interface LoginResponse {
  name: string
  avatar: string
}

/**
 * 认证配置信息
 */
export interface AuthConfig {
  access_token: string
  refresh_token?: string
  expires_at?: number
  user: {
    login: string
    avatar_url: string
  }
}

/**
 * 用户信息
 */
export interface UserInfo {
  name: string
  avatar: string
}