//! 认证仓库 - 实现认证相关的数据访问操作

use crate::models::config::AuthConfig;
use crate::repos::traits::AuthRepo;
use async_trait::async_trait;

/// 认证仓库实现
pub struct AuthRepoImpl;

impl AuthRepoImpl {
    /// 创建认证仓库实例
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl AuthRepo for AuthRepoImpl {
    /// 保存认证配置
    async fn save_auth_config(&self, auth: AuthConfig) -> Result<(), String> {
        crate::repos::config_repo::save_auth_config(auth)
    }

    /// 清除认证配置
    async fn clear_auth_config(&self) -> Result<(), String> {
        let mut config = crate::repos::config_repo::load_config_internal()?;
        config.auth = None;
        crate::repos::config_repo::save_config_internal(&config)
    }

    /// 获取认证配置
    async fn get_auth_config(&self) -> Result<Option<AuthConfig>, String> {
        let config = crate::repos::config_repo::load_config_internal()?;
        Ok(config.auth)
    }
}

/// 获取认证仓库实例
pub fn get_auth_repo() -> Box<dyn AuthRepo + Send + Sync> {
    Box::new(AuthRepoImpl::new())
}
