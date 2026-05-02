/**
 * 配置相关 API 类型定义
 * @module api/config
 */

/**
 * 应用配置信息
 */
export interface AppConfig {
    version: string
    last_update_check?: number

    base_path?: string

    note_path?: string
    vditorMode?: 'wysiwyg' | 'ir' | 'sv'
    auto_save: boolean
    auto_save_interval?: number
    default_note_template?: string
    enable_markdown_preview: boolean
    syntax_highlight: boolean

    auto_sync: boolean
    auto_sync_interval?: number

    enable_debug: boolean
    enable_analytics: boolean
    enable_telemetry: boolean
    max_concurrent_requests?: number
    request_timeout?: number
    cache_duration?: number
}

/**
 * 获取配置响应
 */
export interface GetConfigResponse {
    auth?: AuthConfig
    app?: AppConfig
}

/**
 * 认证配置信息（与 auth.ts 中一致，为了避免循环引用重复定义）
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
 * 保存配置请求参数
 */
export interface SaveConfigRequest {
    config: GetConfigResponse
}

/**
 * 设置应用配置请求参数
 */
export interface SetAppConfigRequest {
    appConfig: Partial<AppConfig>
}
