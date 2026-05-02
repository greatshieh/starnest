//! 认证服务 - 处理认证相关业务逻辑

use crate::infrastructure::external_api::github_client::get_user_info;
use crate::models::config::{AuthConfig, UserInfo};
use crate::repos::traits::AuthRepo;
use crate::services::traits::AuthService;
use async_trait::async_trait;
use serde_json::Value;
use std::sync::OnceLock;

/// 认证服务实现
pub struct AuthServiceImpl {
    auth_repo: Box<dyn AuthRepo + Send + Sync>,
}

/// 认证服务单例
static AUTH_SERVICE: OnceLock<Box<dyn AuthService + Send + Sync>> = OnceLock::new();

#[async_trait]
impl AuthService for AuthServiceImpl {
    /// 登录
    async fn login(&self, token: &str) -> Result<Value, String> {
        let user_response = get_user_info(token).await?;

        let auth_config = AuthConfig {
            access_token: token.to_string(),
            refresh_token: None,
            expires_at: None,
            user: UserInfo {
                login: user_response.login.clone(),
                avatar_url: user_response.avatar_url.clone(),
            },
        };

        self.auth_repo.save_auth_config(auth_config).await?;

        let result =
            serde_json::json!({"name": user_response.login, "avatar": user_response.avatar_url});

        Ok(result)
    }

    /// 登出
    async fn logout(&self) -> Result<(), String> {
        self.auth_repo.clear_auth_config().await
    }

    /// 获取认证配置
    async fn get_auth_config(&self) -> Result<Option<AuthConfig>, String> {
        self.auth_repo.get_auth_config().await
    }
}

impl AuthServiceImpl {
    pub fn new(auth_repo: Box<dyn AuthRepo + Send + Sync>) -> Self {
        Self { auth_repo }
    }
}

/// 获取认证服务实例
pub fn get_auth_service() -> &'static (dyn AuthService + Send + Sync) {
    &**AUTH_SERVICE.get_or_init(|| {
        let auth_repo = crate::repos::auth_repo::get_auth_repo();
        Box::new(AuthServiceImpl::new(auth_repo))
    })
}
