//! 认证处理器 - 处理认证相关命令

use crate::services::auth_service::get_auth_service;
use serde_json::Value;
use tauri::command;

/// 全局认证服务实例
static AUTH_SERVICE: once_cell::sync::Lazy<
    &'static (dyn crate::services::traits::AuthService + Send + Sync),
> = once_cell::sync::Lazy::new(get_auth_service);

/// 登录命令
///
/// 通过 GitHub 访问令牌进行登录验证
///
/// # 参数
/// - `token`: GitHub 访问令牌
///
/// # 返回值
/// 成功返回用户信息，失败返回错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_login(token: String) -> Result<Value, String> {
    if token.trim().is_empty() {
        return Err("Token cannot be empty".to_string());
    }
    AUTH_SERVICE.login(&token).await
}

/// 登出命令
///
/// 清除当前用户的认证信息
///
/// # 返回值
/// 成功返回空对象，失败返回错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_logout() -> Result<Value, String> {
    AUTH_SERVICE
        .logout()
        .await
        .map(|_| Value::Object(serde_json::Map::new()))
}

/// 获取认证配置命令
///
/// 获取当前用户的认证配置信息
///
/// # 返回值
/// 成功返回认证配置，失败返回错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_get_auth_config() -> Result<Option<Value>, String> {
    match AUTH_SERVICE.get_auth_config().await {
        Ok(Some(auth_config)) => {
            let auth_value = serde_json::to_value(auth_config)
                .map_err(|e| format!("Failed to serialize auth config: {:?}", e))?;
            Ok(Some(auth_value))
        }
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    }
}
