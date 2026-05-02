//! 配置处理器 - 处理配置相关命令

use crate::models::config::Config;
use crate::services::config_service::get_config_service;
use crate::services::traits::ConfigService;
use serde_json::Value;
use tauri::command;

/// 全局配置服务实例
static CONFIG_SERVICE: once_cell::sync::Lazy<&'static (dyn ConfigService + Send + Sync)> =
    once_cell::sync::Lazy::new(get_config_service);

/// 获取配置命令
///
/// 获取当前应用配置
///
/// # 返回值
/// 成功返回配置对象，失败返回错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_get_config() -> Result<Value, String> {
    CONFIG_SERVICE.get_config().await
}

/// 获取默认配置命令
///
/// 获取应用的默认配置（不包含认证信息）
///
/// # 返回值
/// 成功返回默认配置对象，失败返回错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_get_default_config() -> Result<Value, String> {
    CONFIG_SERVICE.get_default_config().await
}

/// 重置配置命令
///
/// 将应用配置重置为默认值，保留认证信息（access_token、user info等）
///
/// # 返回值
/// 成功返回空对象，失败返回错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_reset_config() -> Result<Value, String> {
    CONFIG_SERVICE.reset_config().await.map(|_| serde_json::json!({}))
}

/// 保存配置命令
///
/// 保存应用配置
///
/// # 参数
/// - `config`: 配置对象
///
/// # 返回值
/// 成功返回空对象，失败返回错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_save_config(config: Config) -> Result<Value, String> {
    CONFIG_SERVICE
        .save_config(config)
        .await
        .map(|_| serde_json::json!({}))
}
