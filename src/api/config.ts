/**
 * 配置相关 API 封装
 * 统一处理配置相关的后端调用
 * @module api/config
 */

import { invoke } from '@tauri-apps/api/core'
import type { api } from '@/types'

/**
 * 获取配置
 * @returns 完整配置信息（包含认证和应用配置）
 */
export async function getConfig(): Promise<api.config.GetConfigResponse> {
  const result = await invoke('cmd_get_config')
  return result as api.config.GetConfigResponse
}

/**
 * 获取默认配置
 * @returns 默认配置信息（不包含认证信息）
 */
export async function getDefaultConfig(): Promise<api.config.GetConfigResponse> {
  const result = await invoke('cmd_get_default_config')
  return result as api.config.GetConfigResponse
}

/**
 * 重置配置（保留认证信息）
 * @returns 空对象
 */
export async function resetConfig(): Promise<void> {
  await invoke('cmd_reset_config')
}

/**
 * 保存配置
 * @param request - 保存请求参数
 */
export async function saveConfig(request: api.config.SaveConfigRequest): Promise<void> {
  await invoke('cmd_save_config', { request })
}

/**
 * 设置应用配置
 * @param request - 设置请求参数
 */
export async function setAppConfig(request: api.config.SetAppConfigRequest): Promise<void> {
  const currentConfig = await getConfig()
  const updatedConfig: api.config.GetConfigResponse = {
    auth: currentConfig.auth,
    app: { 
      version: '1.0.0', 
      auto_save: false,
      enable_markdown_preview: true,
      syntax_highlight: true,
      auto_sync: false,
      enable_debug: false,
      enable_analytics: false,
      enable_telemetry: false,
      ...currentConfig.app, 
      ...request.appConfig 
    },
  }
  await saveConfig({ config: updatedConfig })
}