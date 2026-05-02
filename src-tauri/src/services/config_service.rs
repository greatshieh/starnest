//! 配置服务 - 处理配置相关业务逻辑

use crate::infrastructure::utils::file::copy_dir_all;
use crate::models::config::Config;
use crate::repos::traits::ConfigRepo;
use crate::services::traits::ConfigService;
use async_trait::async_trait;
use serde_json::Value;
use std::fs;
use std::sync::OnceLock;

/// 配置服务实现
pub struct ConfigServiceImpl {
    config_repo: Box<dyn ConfigRepo + Send + Sync>,
}

/// 配置服务单例
static CONFIG_SERVICE: OnceLock<Box<dyn ConfigService + Send + Sync>> = OnceLock::new();

#[async_trait]
impl ConfigService for ConfigServiceImpl {
    async fn get_config(&self) -> Result<Value, String> {
        let config = self.config_repo.load_config().await?;
        Ok(serde_json::to_value(config).unwrap())
    }

    async fn save_config(&self, config: Config) -> Result<(), String> {
        let old_config = self.config_repo.load_config().await?;

        let old_note_path = self.config_repo.get_note_storage_path(&old_config).await?;
        let new_note_path = self.config_repo.get_note_storage_path(&config).await?;

        if old_note_path != new_note_path && old_note_path.exists() {
            if !new_note_path.exists() {
                fs::create_dir_all(&new_note_path)
                    .map_err(|e| format!("Failed to create new note directory: {:?}", e))?;
            }

            copy_dir_all(old_note_path.as_path(), new_note_path.as_path())
                .map_err(|e| format!("Failed to copy notes to new location: {:?}", e))?;

            fs::remove_dir_all(old_note_path)
                .map_err(|e| format!("Failed to remove old note directory: {:?}", e))?;
        }

        let old_db_path = self.config_repo.get_database_path(&old_config).await?;
        let new_db_path = self.config_repo.get_database_path(&config).await?;

        if old_db_path != new_db_path && old_db_path.exists() {
            if let Some(parent) = new_db_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)
                        .map_err(|e| format!("Failed to create new database directory: {:?}", e))?;
                }
            }

            fs::copy(&old_db_path, &new_db_path)
                .map_err(|e| format!("Failed to copy database to new location: {:?}", e))?;

            fs::remove_file(old_db_path)
                .map_err(|e| format!("Failed to remove old database file: {:?}", e))?;
        }

        self.config_repo.save_config(&config).await
    }

    async fn get_default_config(&self) -> Result<Value, String> {
        let default_config = Config::default();
        Ok(serde_json::to_value(default_config).unwrap())
    }

    async fn reset_config(&self) -> Result<(), String> {
        let current_config = self.config_repo.load_config().await?;
        
        let auth_config = current_config.auth.clone();

        let mut new_config = Config::default();
        let default_note_path = self.config_repo.get_note_storage_path(&new_config).await?;
        new_config.app.note_path = Some(default_note_path.to_string_lossy().to_string());
        new_config.auth = auth_config;

        self.config_repo.save_config(&new_config).await
    }
}

impl ConfigServiceImpl {
    pub fn new(config_repo: Box<dyn ConfigRepo + Send + Sync>) -> Self {
        Self { config_repo }
    }
}

/// 获取配置服务实例
pub fn get_config_service() -> &'static (dyn ConfigService + Send + Sync) {
    &**CONFIG_SERVICE.get_or_init(|| {
        let config_repo = crate::repos::config_repo::get_config_repo();
        Box::new(ConfigServiceImpl::new(config_repo))
    })
}
