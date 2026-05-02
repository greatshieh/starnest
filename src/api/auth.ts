/**
 * 认证相关 API 封装
 * 统一处理认证相关的后端调用
 * @module api/auth
 */

import { invoke } from '@tauri-apps/api/core'
import type { api } from '@/types'

/**
 * 用户登录
 * @param request - 登录请求参数
 * @returns 用户信息
 */
export async function login(request: api.auth.LoginRequest): Promise<api.auth.LoginResponse> {
  const result = await invoke('cmd_login', { token: request.token })
  return result as api.auth.LoginResponse
}

/**
 * 用户登出
 * 清除本地认证状态
 */
export async function logout(): Promise<void> {
  await invoke('cmd_logout')
}

/**
 * 获取认证配置
 * @returns 认证配置信息
 */
export async function getAuthConfig(): Promise<api.auth.AuthConfig | undefined> {
  const result = await invoke('cmd_get_config')
  const config = result as api.config.GetConfigResponse
  return config.auth
}