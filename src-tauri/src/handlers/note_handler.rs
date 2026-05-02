//! 笔记处理器 - 处理笔记相关命令

use crate::services::note_service::get_note_service;
use tauri::command;

/// 保存笔记命令
///
/// 创建或更新笔记
///
/// # 参数
/// - `id`: 笔记 ID（0 表示新建）
/// - `github_id`: 仓库 GitHub ID
/// - `owner`: 仓库所有者
/// - `repo_name`: 仓库名称
/// - `note_name`: 笔记名称
/// - `content`: 笔记内容
///
/// # 返回值
/// 成功返回笔记信息，失败返回错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_save_note(
    id: i64,
    github_id: i64,
    owner: String,
    repo_name: String,
    note_name: String,
    content: String,
) -> Result<serde_json::Value, String> {
    if owner.trim().is_empty() {
        return Err("Owner cannot be empty".to_string());
    }
    if repo_name.trim().is_empty() {
        return Err("Repo name cannot be empty".to_string());
    }
    if note_name.trim().is_empty() {
        return Err("Note name cannot be empty".to_string());
    }

    let note_service = get_note_service().await;
    note_service
        .save_note(id, github_id, &owner, &repo_name, &note_name, &content)
        .await
}

/// 获取笔记内容命令
///
/// # 参数
/// - `owner`: 仓库所有者
/// - `repo_name`: 仓库名称
/// - `note_name`: 笔记名称（不含 .md 后缀）
///
/// # 返回值
/// 成功返回笔记内容，失败返回错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_read_note(
    owner: String,
    repo_name: String,
    note_name: String,
) -> Result<String, String> {
    let note_service = get_note_service().await;
    note_service.read_note(owner, repo_name, note_name).await
}

/// 获取仓库笔记列表命令
///
/// # 参数
/// - `github_id`: 仓库 GitHub ID
///
/// # 返回值
/// 成功返回笔记列表，失败返回错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_get_notes_by_repo(github_id: i64) -> Result<serde_json::Value, String> {
    let note_service = get_note_service().await;
    note_service.get_notes_by_repo(github_id).await
}

/// 获取默认笔记名称命令
///
/// # 参数
/// - `owner`: 仓库所有者
/// - `repo_name`: 仓库名称
///
/// # 返回值
/// 成功返回默认笔记名称，失败返回错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_get_default_note_name(owner: String, repo_name: String) -> Result<String, String> {
    let note_service = get_note_service().await;
    note_service
        .get_default_note_name_service(&owner, &repo_name)
        .await
}
