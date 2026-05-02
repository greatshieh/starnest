//! 配置仓储 - 实现配置相关的数据访问操作

use crate::models::config::{AuthConfig, Config};
use crate::repos::traits::ConfigRepo;
use async_trait::async_trait;
use dirs;
use serde_json;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

/// 配置仓库实现
pub struct ConfigRepoImpl;

impl ConfigRepoImpl {
    pub fn new() -> Self {
        Self
    }
}

fn get_data_dir() -> Result<PathBuf, String> {
    let data_dir = dirs::data_dir().ok_or_else(|| "Failed to get data directory".to_string())?;
    Ok(data_dir.join("starnest"))
}

fn get_config_file_path() -> Result<PathBuf, String> {
    Ok(get_data_dir()?.join("config.json"))
}

fn get_database_path(config: &Config) -> Result<PathBuf, String> {
    if let Some(ref custom_path) = config.app.db_path {
        Ok(PathBuf::from(custom_path))
    } else {
        Ok(get_data_dir()?.join("starnest.db"))
    }
}

pub(crate) fn get_note_storage_path(config: &Config) -> Result<PathBuf, String> {
    if let Some(ref custom_path) = config.app.note_path {
        Ok(PathBuf::from(custom_path))
    } else {
        Ok(get_data_dir()?.join("notes"))
    }
}

fn init_base_dirs() -> Result<(), String> {
    let data_dir = get_data_dir()?;
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)
            .map_err(|e| format!("Failed to create data directory: {:?}", e))?;
    }

    let cache_dir = dirs::cache_dir()
        .ok_or_else(|| "Failed to get cache directory".to_string())?
        .join("starnest");
    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir)
            .map_err(|e| format!("Failed to create cache directory: {:?}", e))?;
    }

    Ok(())
}

fn init_config_dirs(config: &Config) -> Result<(), String> {
    init_base_dirs()?;

    let notes_path = get_note_storage_path(config)?;
    if !notes_path.exists() {
        fs::create_dir_all(&notes_path)
            .map_err(|e| format!("Failed to create notes directory: {:?}", e))?;
    }

    Ok(())
}

fn save_config_sync(config: &Config) -> Result<(), String> {
    let config_path = get_config_file_path()?;

    let content = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {:?}", e))?;

    let mut file =
        File::create(&config_path).map_err(|e| format!("Failed to create config file: {:?}", e))?;

    file.write_all(content.as_bytes())
        .map_err(|e| format!("Failed to write config file: {:?}", e))
}

fn load_config_sync() -> Result<Config, String> {
    let config_path = get_config_file_path()?;

    if !config_path.exists() {
        let mut config = Config::default();

        let default_note_path = get_data_dir()?.join("notes");
        config.app.note_path = Some(default_note_path.to_string_lossy().to_string());

        init_config_dirs(&config)?;
        save_config_sync(&config)?;
        return Ok(config);
    }

    let mut file =
        File::open(&config_path).map_err(|e| format!("Failed to open config file: {:?}", e))?;

    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| format!("Failed to read config file: {:?}", e))?;

    let config: Config = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse config file: {:?}", e))?;

    Ok(config)
}

#[async_trait]
impl ConfigRepo for ConfigRepoImpl {
    async fn load_config(&self) -> Result<Config, String> {
        load_config_sync()
    }

    async fn save_config(&self, config: &Config) -> Result<(), String> {
        save_config_sync(config)
    }

    async fn get_note_storage_path(&self, config: &Config) -> Result<PathBuf, String> {
        get_note_storage_path(config)
    }

    async fn get_database_path(&self, config: &Config) -> Result<PathBuf, String> {
        get_database_path(config)
    }
}

pub fn save_auth_config(auth: AuthConfig) -> Result<(), String> {
    let mut config = load_config_sync()?;
    config.auth = Some(auth);
    save_config_sync(&config)
}

pub(crate) fn load_config_internal() -> Result<Config, String> {
    load_config_sync()
}

pub(crate) fn save_config_internal(config: &Config) -> Result<(), String> {
    save_config_sync(config)
}

pub fn get_config_repo() -> Box<dyn ConfigRepo + Send + Sync> {
    Box::new(ConfigRepoImpl::new())
}
