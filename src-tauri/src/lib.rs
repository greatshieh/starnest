//! starnest - GitHub Star Manager 后端应用入口
//!
//! 基于 Tauri 的跨平台应用，用于管理 GitHub Star 仓库。
//!
//! # 项目结构
//!
//! 采用实用分层架构设计：
//! - **models**: 数据模型层（实体、DTO、配置）
//! - **repos**: 仓储层（数据访问）
//! - **services**: 服务层（业务逻辑）
//! - **handlers**: 处理器层（命令处理）
//! - **infrastructure**: 基础设施层（外部 API、工具）
//! - **db**: 数据库连接管理

use regex::Regex;
use std::fs;
use std::path::PathBuf;

use tauri::http::{response::Builder as ResponseBuilder, status::StatusCode};

// 模块导入
mod db;
mod handlers;
mod infrastructure;
mod models;
mod repos;
mod services;

// 导入异步任务包装函数
use crate::infrastructure::utils::async_task::spawn_tauri_task_detached;

// 导出命令处理器
pub use handlers::auth_handler::*;
pub use handlers::category_handler::*;
pub use handlers::collection_handler::*;
pub use handlers::config_handler::*;
pub use handlers::note_handler::*;
pub use handlers::repo_handler::*;

/// 获取头像命令
///
/// 从缓存或网络获取头像图片
///
/// # 参数
/// - `asset_url`: 头像资源 URL
///
/// # 返回值
/// 成功返回头像字节数据，失败返回错误信息
pub async fn get_avatar(asset_url: &str) -> Result<Vec<u8>, String> {
    let re = Regex::new(r"asset://localhost/|http://asset\.localhost/").unwrap();
    let url = re.replace_all(asset_url, "").to_string();
    let decoded_url = percent_encoding::percent_decode_str(&url)
        .decode_utf8_lossy()
        .to_string();

    let hash = infrastructure::utils::hash::compute_sha256_hash(&decoded_url);
    let avatar_path = get_avatar_path(&hash)?;

    if avatar_path.exists() {
        fs::read(&avatar_path).map_err(|e| format!("Failed to read avatar file: {:?}", e))
    } else {
        let full_url = if decoded_url.starts_with("http://") || decoded_url.starts_with("https://")
        {
            decoded_url.to_string()
        } else {
            format!("https://{}", decoded_url)
        };

        let client = reqwest::Client::new();
        let response = client
            .get(&full_url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch avatar: {:?}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "Failed to fetch avatar: HTTP {}",
                response.status()
            ));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to read avatar bytes: {:?}", e))?
            .to_vec();

        if let Some(parent) = avatar_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create avatar directory: {:?}", e))?;
        }

        fs::write(&avatar_path, &bytes)
            .map_err(|e| format!("Failed to save avatar file: {:?}", e))?;

        Ok(bytes)
    }
}

/// 获取头像缓存路径
///
/// # 参数
/// - `hash`: 头像 URL 的 SHA256 哈希值
///
/// # 返回值
/// 成功返回头像文件路径，失败返回错误信息
fn get_avatar_path(hash: &str) -> Result<PathBuf, String> {
    let cache_dir = dirs::cache_dir().ok_or_else(|| "Failed to get cache directory".to_string())?;
    Ok(cache_dir.join("starnest").join("avatars").join(hash))
}

/// 初始化应用设置
///
/// 执行数据库初始化流程：
/// 1. 初始化配置
/// 2. 获取数据库路径
/// 3. 建立数据库连接池
/// 4. 初始化数据表结构
/// 5. 设置全局连接池
///
/// # 返回值
/// 成功返回 Ok(()), 失败返回错误信息
fn setup() -> Result<(), String> {
    // 初始化应用配置
    let config = db::config::init_config()?;

    let _ = tauri::async_runtime::block_on(async move { crate::db::sqlite::init_db(config).await });

    Ok(())
}

/// Tauri 应用入口函数
///
/// 注册 URI 协议处理器、插件和命令处理器，启动应用。
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .setup(|_app| {
            setup()?;
            Ok(())
        })
        .register_asynchronous_uri_scheme_protocol("asset", |_ctx, request, responder| {
            let uri_str = request.uri().to_string();
            spawn_tauri_task_detached("avatar_fetcher", async move {
                let img_bin = get_avatar(&uri_str).await?;
                responder.respond(
                    ResponseBuilder::new()
                        .status(StatusCode::OK)
                        .header("Content-Type", "image/png")
                        .body(img_bin)
                        .map_err(|e| format!("Failed to build response: {}", e))?,
                );
                Ok(())
            });
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            cmd_login,
            cmd_logout,
            cmd_sync_repos,
            cmd_get_repos,
            cmd_get_repo_filters,
            cmd_search_repos,
            cmd_sync_fts,
            cmd_get_repo_readme,
            cmd_save_note,
            cmd_read_note,
            cmd_get_notes_by_repo,
            cmd_get_default_note_name,
            cmd_get_config,
            cmd_get_default_config,
            cmd_reset_config,
            cmd_save_config,
            cmd_get_categories,
            cmd_update_repo_categories,
            cmd_create_category,
            cmd_delete_category,
            cmd_get_repo_events_by_date,
            cmd_fetch_repo_activities_from_github,
            cmd_star_repo,
            cmd_unstar_repo,
            cmd_soft_delete_repo,
            cmd_restore_repo,
            cmd_get_recent_updated_repos,
            cmd_get_repo_status_categories,
            cmd_get_repo_stats,
            cmd_get_categories_paged,
            cmd_create_collection,
            cmd_update_collection,
            cmd_delete_collection,
            cmd_get_collection_by_id,
            cmd_get_all_collections,
            cmd_get_collections_paged,
            cmd_get_repos_by_collection,
            cmd_get_collections_by_repo,
            cmd_update_collection_repos,
            cmd_is_repo_in_collection,
            cmd_update_category,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
