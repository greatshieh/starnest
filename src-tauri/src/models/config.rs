//! 配置结构 - 应用配置相关的数据结构

use serde::{Deserialize, Serialize};

/// 认证配置
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AuthConfig {
    /// 访问令牌
    pub access_token: String,
    /// 刷新令牌（可选）
    pub refresh_token: Option<String>,
    /// 过期时间戳（可选）
    pub expires_at: Option<i64>,
    /// 用户信息
    pub user: UserInfo,
}

/// 用户信息
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserInfo {
    /// 用户登录名
    pub login: String,
    /// 用户头像 URL
    pub avatar_url: String,
}

/// 应用配置
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct AppConfig {
    /// 应用版本
    pub version: String,

    /// 最后更新检查时间戳
    pub last_update_check: Option<i64>,

    /// 数据库路径设置
    pub db_path: Option<String>,

    /// 笔记设置 - 笔记存储路径
    pub note_path: Option<String>,
    /// 笔记设置 - 自动保存
    #[serde(default)]
    pub auto_save: bool,
    /// 笔记设置 - 自动保存间隔（秒）
    pub auto_save_interval: Option<u32>,
    /// 笔记设置 - 默认笔记模板
    pub default_note_template: Option<String>,
    /// 笔记设置 - 启用 Markdown 预览
    #[serde(default)]
    pub enable_markdown_preview: bool,
    /// 笔记设置 - 语法高亮
    #[serde(default)]
    pub syntax_highlight: bool,

    /// 数据管理设置 - 自动同步
    #[serde(default)]
    pub auto_sync: bool,
    /// 数据管理设置 - 自动同步间隔（小时）
    pub auto_sync_interval: Option<u32>,

    /// 高级设置 - 启用调试模式
    #[serde(default)]
    pub enable_debug: bool,
    /// 高级设置 - 启用分析
    #[serde(default)]
    pub enable_analytics: bool,
    /// 高级设置 - 启用遥测
    #[serde(default)]
    pub enable_telemetry: bool,
    /// 高级设置 - 最大并发请求数
    pub max_concurrent_requests: Option<u32>,
    /// 高级设置 - 请求超时时间（秒）
    pub request_timeout: Option<u32>,
    /// 高级设置 - 缓存持续时间（小时）
    pub cache_duration: Option<u32>,
}

/// 全局配置
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    /// 认证配置
    pub auth: Option<AuthConfig>,
    /// 应用配置
    #[serde(default)]
    pub app: AppConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            auth: None,
            app: AppConfig {
                version: "1.0.0".to_string(),
                last_update_check: None,
                db_path: None,
                note_path: None,
                auto_save: true,
                auto_save_interval: Some(30),
                default_note_template: None,
                enable_markdown_preview: true,
                syntax_highlight: true,
                auto_sync: false,
                auto_sync_interval: Some(24),
                enable_debug: false,
                enable_analytics: true,
                enable_telemetry: true,
                max_concurrent_requests: Some(5),
                request_timeout: Some(30),
                cache_duration: Some(24),
            },
        }
    }
}
