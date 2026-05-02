use dirs;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AuthConfig {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<i64>,
    pub user: UserInfo,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserInfo {
    pub login: String,
    pub avatar_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct AppConfig {
    pub version: String,
    pub last_update_check: Option<i64>,

    // Database path settings - user customizable
    pub db_path: Option<String>,

    #[serde(default)]
    pub high_contrast_mode: bool,
    #[serde(default)]
    pub reduce_motion: bool,

    // Notes settings
    pub note_path: Option<String>,
    #[serde(default)]
    pub auto_save: bool,
    pub auto_save_interval: Option<u32>,
    pub default_note_template: Option<String>,
    #[serde(default)]
    pub enable_markdown_preview: bool,
    #[serde(default)]
    pub syntax_highlight: bool,

    // Data Management settings
    #[serde(default)]
    pub auto_sync: bool,
    pub auto_sync_interval: Option<u32>,

    // Advanced settings
    #[serde(default)]
    pub enable_debug: bool,
    #[serde(default)]
    pub enable_analytics: bool,
    #[serde(default)]
    pub enable_telemetry: bool,
    pub max_concurrent_requests: Option<u32>,
    pub request_timeout: Option<u32>,
    pub cache_duration: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub auth: Option<AuthConfig>,
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
                high_contrast_mode: false,
                reduce_motion: true,
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

/// 获取配置文件存储的根目录（固定为 dirs::data_dir()/starnest）
/// 该路径不可变且不对外暴露
fn get_data_dir() -> Result<PathBuf, String> {
    let data_dir = dirs::data_dir().ok_or_else(|| "Failed to get data directory".to_string())?;
    Ok(data_dir.join("starnest"))
}

/// 获取缓存文件存储的根目录（固定为 dirs::cache_dir()/starnest）
/// 该路径不可变且不对外暴露
pub fn get_cache_dir() -> Result<PathBuf, String> {
    let cache_dir = dirs::cache_dir().ok_or_else(|| "Failed to get cache directory".to_string())?;
    Ok(cache_dir.join("starnest"))
}

/// 获取配置文件的绝对路径（固定路径）
pub fn get_config_file_path() -> Result<PathBuf, String> {
    Ok(get_data_dir()?.join("config.json"))
}

/// 获取数据库文件路径
/// 优先使用用户自定义路径，否则使用默认路径
pub fn get_database_path(config: &Config) -> Result<PathBuf, String> {
    if let Some(ref custom_path) = config.app.db_path {
        let db_path = PathBuf::from(custom_path);
        if let Some(parent) = db_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create database directory: {:?}", e))?;
            }
        }
        Ok(db_path)
    } else {
        let default_path = get_data_dir()?.join("starnest.db");
        Ok(default_path)
    }
}

/// 获取笔记存储路径
/// 优先使用用户自定义路径，否则使用默认路径
pub fn get_note_storage_path(config: &Config) -> Result<PathBuf, String> {
    if let Some(ref custom_path) = config.app.note_path {
        Ok(PathBuf::from(custom_path))
    } else {
        Ok(get_data_dir()?.join("notes"))
    }
}

/// 初始化基础目录（data_dir 和 cache_dir）
/// 这些目录不随用户配置变化，只需初始化一次
fn init_base_dirs() -> Result<(), String> {
    let data_dir = get_data_dir()?;
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)
            .map_err(|e| format!("Failed to create data directory: {:?}", e))?;
    }

    let cache_dir = get_cache_dir()?;
    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir)
            .map_err(|e| format!("Failed to create cache directory: {:?}", e))?;
    }

    Ok(())
}

/// 初始化配置相关的目录
pub fn init_config_dirs(config: &Config) -> Result<(), String> {
    init_base_dirs()?;

    let notes_path = get_note_storage_path(config)?;
    if !notes_path.exists() {
        fs::create_dir_all(&notes_path)
            .map_err(|e| format!("Failed to create notes directory: {:?}", e))?;
    }

    Ok(())
}

/// 加载配置文件
/// 首次运行时自动创建默认配置
pub fn load_config() -> Result<Config, String> {
    let config_path = get_config_file_path()?;

    if !config_path.exists() {
        let config = Config::default();
        init_config_dirs(&config)?;
        save_config(&config)?;
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

/// 初始化应用配置并返回全局配置引用
///
/// 该函数实现了配置的懒加载和单例模式：
/// 1. 首先检查 CONFIG 静态变量是否已初始化
/// 2. 如果已初始化，直接返回缓存的配置引用（避免重复加载）
/// 3. 如果未初始化，调用 load_config() 从文件加载配置
/// 4. 将加载的配置设置到 CONFIG 静态变量中，供后续调用使用
///
/// # 返回值
/// - `Ok(&'static Config)`: 成功返回全局配置的静态引用
/// - `Err(String)`: 配置加载或设置失败时返回错误信息
///
/// # 注意
/// 该函数是线程安全的，使用 `OnceLock` 保证只初始化一次
pub fn init_config() -> Result<&'static Config, String> {
    if let Some(config) = CONFIG.get() {
        return Ok(config);
    }

    let config = load_config()?;

    match CONFIG.set(config) {
        Ok(_) => Ok(CONFIG.get().unwrap()),
        Err(_) => Err("Failed to set config".to_string()),
    }
}

/// 保存配置到文件
pub fn save_config(config: &Config) -> Result<(), String> {
    let config_path = get_config_file_path()?;

    let content = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {:?}", e))?;

    let mut file =
        File::create(&config_path).map_err(|e| format!("Failed to create config file: {:?}", e))?;

    file.write_all(content.as_bytes())
        .map_err(|e| format!("Failed to write config file: {:?}", e))
}

// 更新全局配置并持久化
// pub fn update_config(new_config: Config) -> Result<&'static Config, String> {
//     save_config(&new_config)?;

//     match CONFIG.set(new_config) {
//         Ok(_) => Ok(CONFIG.get().unwrap()),
//         Err(_) => Err("Failed to update config".to_string()),
//     }
// }
