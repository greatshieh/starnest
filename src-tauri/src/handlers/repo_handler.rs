//! 仓库处理器模块
//!
//! 该模块负责处理前端发起的仓库相关命令请求，作为 Tauri IPC 的入口层。
//! 涵盖仓库同步、查询、搜索、标星管理等核心功能。

use crate::infrastructure::external_api::github_client::{star_repo, sync_repos, unstar_repo};
use crate::models::dto::{
    FetchRepoActivitiesRequest, GetReadmeRequest, GetRepoEventsByDateRequest,
    GetRepoFiltersRequest, GetReposRequest, SearchReposRequest, SoftDeleteRepoRequest,
    StarRepoRequest, SyncReposRequest,
};
use crate::repos::repo_repo::{restore_repo, soft_delete_repo};
use crate::services::activity_service::{fetch_and_save_activities, get_events_by_date};
use crate::services::repo_service::get_repo_service;
use tauri::{command, AppHandle, Runtime};

/// 同步 GitHub 仓库到本地数据库
///
/// 从 GitHub API 获取用户的标星仓库列表，并同步到本地 SQLite 数据库。
///
/// # Arguments
/// - `request`: 同步请求（包含 token）
/// - `app_handle`: Tauri 应用句柄，用于发送进度事件
///
/// # Returns
/// - `Result<serde_json::Value, String>`: 同步结果（包含新增、更新、删除数量）或错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_sync_repos<R: Runtime>(
    request: SyncReposRequest,
    app_handle: AppHandle<R>,
) -> Result<serde_json::Value, String> {
    // 参数校验：确保 token 不为空
    if request.token.trim().is_empty() {
        return Err("Token cannot be empty".to_string());
    }

    // 调用 GitHub 客户端同步仓库
    let sync_result = sync_repos(&request.token, &app_handle).await?;

    // 将同步结果序列化为 JSON 返回给前端
    Ok(serde_json::to_value(sync_result).unwrap())
}

/// 分页获取仓库列表
///
/// 根据分页参数、排序方式、语言和分类筛选条件获取仓库列表。
///
/// # Arguments
/// - `request`: 获取仓库请求
///
/// # Returns
/// - `Result<serde_json::Value, String>`: 仓库列表数据或错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_get_repos(request: GetReposRequest) -> Result<serde_json::Value, String> {
    // 获取仓库服务单例实例
    let repo_service = get_repo_service().await;
    // 调用服务层方法获取仓库列表
    repo_service
        .get_repos(
            request.page,
            request.page_size,
            request.sort,
            request.language,
            request.categories,
            request.deleted,
            request.sort_order,
        )
        .await
}

/// 获取仓库筛选条件（语言和分类）
///
/// 返回当前数据库中可用的语言列表和分类列表，用于前端筛选器。
/// 支持按categories/language过滤获取关联选项。
///
/// # Arguments
/// - `request`: 获取筛选条件请求
///
/// # Returns
/// - `Result<serde_json::Value, String>`: 筛选条件数据或错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_get_repo_filters(
    request: GetRepoFiltersRequest,
) -> Result<serde_json::Value, String> {
    // 获取仓库服务单例实例
    let repo_service = get_repo_service().await;
    // 调用服务层方法获取筛选条件
    repo_service
        .get_repo_filters(request.deleted, request.categories, request.language)
        .await
}

/// 搜索仓库
///
/// 根据关键词搜索仓库名称、描述等字段。
///
/// # Arguments
/// - `request`: 搜索请求
///
/// # Returns
/// - `Result<serde_json::Value, String>`: 搜索结果或错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_search_repos(request: SearchReposRequest) -> Result<serde_json::Value, String> {
    // 获取仓库服务单例实例
    let repo_service = get_repo_service().await;
    // 调用服务层方法搜索仓库
    repo_service
        .search_repos(
            &request.query,
            request.page,
            request.page_size,
            request.deleted,
        )
        .await
}

/// 同步全文搜索索引
///
/// 更新 FTS（全文搜索）索引，确保搜索功能能够检索最新的仓库数据。
///
/// # Returns
/// - `Result<String, String>`: 成功消息或错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_sync_fts() -> Result<String, String> {
    // 获取仓库服务单例实例
    let repo_service = get_repo_service().await;
    // 调用服务层方法同步 FTS 索引
    repo_service.sync_fts().await
}

/// 获取仓库 README 内容
///
/// 从 GitHub 获取指定仓库的 README.md 文件内容。
///
/// # Arguments
/// - `request`: 获取 README 请求
///
/// # Returns
/// - `Result<String, String>`: README 内容（Markdown 格式）或错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_get_repo_readme(request: GetReadmeRequest) -> Result<String, String> {
    // 参数校验：确保 owner 不为空
    if request.owner.trim().is_empty() {
        return Err("Owner cannot be empty".to_string());
    }
    // 参数校验：确保 repo_name 不为空
    if request.repo_name.trim().is_empty() {
        return Err("Repo name cannot be empty".to_string());
    }
    // 获取仓库服务单例实例
    let repo_service = get_repo_service().await;
    // 调用服务层方法获取 README
    repo_service
        .get_repo_readme(&request.owner, &request.repo_name)
        .await
}

/// 获取最近一周更新的仓库
///
/// 返回最近一周内有推送活动的仓库列表，按更新时间降序排列。
///
/// # Returns
/// - `Result<serde_json::Value, String>`: 最近更新的仓库列表或错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_get_recent_updated_repos() -> Result<serde_json::Value, String> {
    // 获取仓库服务单例实例
    let repo_service = get_repo_service().await;
    // 调用服务层方法获取最近更新的仓库
    repo_service.get_recent_updated_repos().await
}

/// 获取状态分类仓库（归档、低活跃、废弃）
///
/// 返回按状态分类的仓库统计，包括归档、低活跃、废弃三种状态。
///
/// # Returns
/// - `Result<serde_json::Value, String>`: 状态分类统计数据或错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_get_repo_status_categories() -> Result<serde_json::Value, String> {
    // 获取仓库服务单例实例
    let repo_service = get_repo_service().await;
    // 调用服务层方法获取状态分类
    repo_service.get_repo_status_categories().await
}

/// 获取仓库统计信息
///
/// 返回仪表盘所需的统计数据，包括总仓库数、分类数、未分类仓库数、最近标星数。
///
/// # Returns
/// - `Result<serde_json::Value, String>`: 统计信息或错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_get_repo_stats() -> Result<serde_json::Value, String> {
    // 获取仓库服务单例实例
    let repo_service = get_repo_service().await;
    // 调用服务层方法获取统计信息
    repo_service.get_repo_stats().await
}

/// 获取按日期聚合的事件统计
///
/// 返回指定仓库按日期聚合的事件数量，用于前端图表展示。
///
/// # Arguments
/// - `request`: 获取事件统计请求
///
/// # Returns
/// - `Result<serde_json::Value, String>`: 按日期聚合的事件统计或错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_get_repo_events_by_date(
    request: GetRepoEventsByDateRequest,
) -> Result<serde_json::Value, String> {
    // 调用活动服务获取按日期聚合的事件
    let result = get_events_by_date(request.repo_id).await?;
    // 将结果序列化为 JSON 返回给前端
    Ok(serde_json::to_value(result).unwrap())
}

/// 从 GitHub 获取活动并保存到数据库
///
/// 从 GitHub API 获取指定仓库的活动记录，并保存到本地数据库。
///
/// # Arguments
/// - `request`: 获取活动请求
///
/// # Returns
/// - `Result<(), String>`: 成功返回空，失败返回错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_fetch_repo_activities_from_github(
    request: FetchRepoActivitiesRequest,
) -> Result<(), String> {
    // 参数校验：确保 owner 不为空
    if request.owner.trim().is_empty() {
        return Err("Owner cannot be empty".to_string());
    }
    // 参数校验：确保 repo_name 不为空
    if request.repo_name.trim().is_empty() {
        return Err("Repo name cannot be empty".to_string());
    }
    // 调用活动服务获取并保存活动
    fetch_and_save_activities(&request.owner, &request.repo_name).await?;
    Ok(())
}

/// 为仓库添加标星
///
/// 通过 GitHub API 为指定仓库添加标星，并恢复本地数据库中的仓库记录。
///
/// # Arguments
/// - `request`: 标星请求
///
/// # Returns
/// - `Result<(), String>`: 成功返回空，失败返回错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_star_repo(request: StarRepoRequest) -> Result<(), String> {
    // 参数校验：确保 owner 不为空
    if request.owner.trim().is_empty() {
        return Err("Owner cannot be empty".to_string());
    }
    // 参数校验：确保 repo_name 不为空
    if request.repo_name.trim().is_empty() {
        return Err("Repo name cannot be empty".to_string());
    }
    // 通过 GitHub API 添加标星
    star_repo(&request.owner, &request.repo_name).await?;

    // 恢复本地数据库中的仓库记录（取消软删除标记）
    restore_repo(&request.owner, &request.repo_name).await?;

    Ok(())
}

/// 取消仓库标星
///
/// 通过 GitHub API 取消指定仓库的标星，并在本地数据库中软删除该仓库。
///
/// # Arguments
/// - `request`: 取消标星请求
///
/// # Returns
/// - `Result<(), String>`: 成功返回空，失败返回错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_unstar_repo(request: StarRepoRequest) -> Result<(), String> {
    // 参数校验：确保 owner 不为空
    if request.owner.trim().is_empty() {
        return Err("Owner cannot be empty".to_string());
    }
    // 参数校验：确保 repo_name 不为空
    if request.repo_name.trim().is_empty() {
        return Err("Repo name cannot be empty".to_string());
    }
    // 通过 GitHub API 取消标星
    unstar_repo(&request.owner, &request.repo_name).await?;

    // 在本地数据库中软删除仓库
    soft_delete_repo(&request.owner, &request.repo_name).await?;

    Ok(())
}

/// 软删除仓库（标记为已取消标星）
///
/// 在本地数据库中将仓库标记为已删除状态，但不实际删除数据。
///
/// # Arguments
/// - `request`: 软删除请求
///
/// # Returns
/// - `Result<(), String>`: 成功返回空，失败返回错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_soft_delete_repo(request: SoftDeleteRepoRequest) -> Result<(), String> {
    // 参数校验：确保 owner 不为空
    if request.owner.trim().is_empty() {
        return Err("Owner cannot be empty".to_string());
    }
    // 参数校验：确保 repo_name 不为空
    if request.repo_name.trim().is_empty() {
        return Err("Repo name cannot be empty".to_string());
    }
    // 调用仓库仓储方法软删除仓库
    soft_delete_repo(&request.owner, &request.repo_name).await?;
    Ok(())
}

/// 恢复已删除的仓库
///
/// 取消仓库的软删除标记，使其重新显示在仓库列表中。
///
/// # Arguments
/// - `request`: 恢复请求
///
/// # Returns
/// - `Result<(), String>`: 成功返回空，失败返回错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_restore_repo(request: SoftDeleteRepoRequest) -> Result<(), String> {
    // 参数校验：确保 owner 不为空
    if request.owner.trim().is_empty() {
        return Err("Owner cannot be empty".to_string());
    }
    // 参数校验：确保 repo_name 不为空
    if request.repo_name.trim().is_empty() {
        return Err("Repo name cannot be empty".to_string());
    }
    // 调用仓库仓储方法恢复仓库
    restore_repo(&request.owner, &request.repo_name).await?;
    Ok(())
}
