//! 合集处理器模块
//!
//! 该模块负责处理前端发起的合集相关命令请求，作为 Tauri IPC 的入口层。
//! 所有命令通过 Tauri 的 `#[command]` 宏暴露给前端调用，内部调用对应的服务层处理业务逻辑。

use crate::models::dto::{
    CollectionPageRequest, UpdateCollectionReposRequest, UpdateCollectionRequest,
};
use crate::services::collection_service::get_collection_service;
use tauri::command;

/// 创建合集
#[command(rename_all = "snake_case")]
pub async fn cmd_create_collection(
    name: String,
    description: String,
    color: String,
) -> Result<crate::models::dto::CollectionWithRepoCount, String> {
    let collection_service = get_collection_service().await;
    collection_service
        .create_collection(&name, &description, &color)
        .await
}

/// 更新合集
#[command(rename_all = "snake_case")]
pub async fn cmd_update_collection(
    request: UpdateCollectionRequest,
) -> Result<crate::models::dto::CollectionWithRepoCount, String> {
    let collection_service = get_collection_service().await;
    collection_service
        .update_collection(
            request.collection_id,
            request.name.as_deref(),
            request.description.as_deref(),
            request.color.as_deref(),
        )
        .await
}

/// 删除合集
#[command(rename_all = "snake_case")]
pub async fn cmd_delete_collection(collection_id: i64) -> Result<(), String> {
    let collection_service = get_collection_service().await;
    collection_service.delete_collection(collection_id).await
}

/// 根据ID获取合集
#[command]
pub async fn cmd_get_collection_by_id(
    collection_id: i64,
) -> Result<crate::models::dto::CollectionWithRepoCount, String> {
    let collection_service = get_collection_service().await;
    collection_service.get_collection_by_id(collection_id).await
}

/// 获取所有合集
#[command(rename_all = "snake_case")]
pub async fn cmd_get_all_collections(
) -> Result<Vec<crate::models::dto::CollectionWithRepoCount>, String> {
    let collection_service = get_collection_service().await;
    collection_service.get_all_collections().await
}

/// 分页获取合集列表
#[command]
pub async fn cmd_get_collections_paged(
    request: CollectionPageRequest,
) -> Result<crate::models::dto::CollectionPageResponse, String> {
    let collection_service = get_collection_service().await;
    collection_service.get_collections_paged(request).await
}

/// 获取合集关联的所有仓库
#[command(rename_all = "snake_case")]
pub async fn cmd_get_repos_by_collection(collection_id: i64) -> Result<serde_json::Value, String> {
    let collection_service = get_collection_service().await;
    collection_service
        .get_repos_by_collection(collection_id)
        .await
}

/// 获取仓库关联的所有合集
#[command(rename_all = "snake_case")]
pub async fn cmd_get_collections_by_repo(
    github_id: i64,
) -> Result<Vec<crate::models::entity::Collection>, String> {
    let collection_service = get_collection_service().await;
    collection_service.get_collections_by_repo(github_id).await
}

/// 更新合集与仓库的关联关系
#[command(rename_all = "snake_case")]
pub async fn cmd_update_collection_repos(
    request: UpdateCollectionReposRequest,
) -> Result<(), String> {
    let collection_service = get_collection_service().await;
    collection_service.update_collection_repos(request).await
}

/// 检查仓库是否已在合集中
#[command(rename_all = "snake_case")]
pub async fn cmd_is_repo_in_collection(github_id: i64, collection_id: i64) -> Result<bool, String> {
    let collection_service = get_collection_service().await;
    collection_service
        .is_repo_in_collection(github_id, collection_id)
        .await
}
