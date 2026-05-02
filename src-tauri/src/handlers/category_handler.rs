//! 分类处理器模块
//!
//! 该模块负责处理前端发起的分类相关命令请求，作为 Tauri IPC 的入口层。
//! 所有命令通过 Tauri 的 `#[command]` 宏暴露给前端调用，内部调用对应的服务层处理业务逻辑。

use crate::models::dto::{CategoryPageRequest, UpdateCategoryRequest};
use crate::services::category_service::get_category_service;
use tauri::command;

/// 获取所有分类及其关联的仓库数量
///
/// 返回所有分类的列表，每个分类包含其 ID、名称、颜色和关联的仓库数量。
///
/// # Returns
/// - `Result<Vec<CategoryWithCount>, String>`: 分类列表或错误信息
#[command]
pub async fn cmd_get_categories() -> Result<Vec<crate::models::dto::CategoryWithCount>, String> {
    // 获取分类服务单例实例
    let category_service = get_category_service().await;
    // 调用服务层方法获取所有分类
    category_service.get_all_categories().await
}

/// 分页获取分类列表
///
/// 根据分页请求参数查询分类，支持搜索关键字过滤和排序。
///
/// # Arguments
/// - `request`: 分页请求参数，包含页码、每页大小、搜索关键字、排序字段和排序方向
///
/// # Returns
/// - `Result<CategoryPageResponse, String>`: 分页响应或错误信息
#[command]
pub async fn cmd_get_categories_paged(
    request: CategoryPageRequest,
) -> Result<crate::models::dto::CategoryPageResponse, String> {
    // 获取分类服务单例实例
    let category_service = get_category_service().await;
    // 调用服务层方法进行分页查询
    category_service.get_categories_paged(request).await
}

/// 更新分类信息
///
/// 根据分类ID更新分类名称和/或颜色。
///
/// # Arguments
/// - `request`: 更新分类请求，包含分类ID和可选的名称、颜色
///
/// # Returns
/// - `Result<CategoryWithCount, String>`: 更新后的分类信息或错误信息
#[command]
pub async fn cmd_update_category(
    request: UpdateCategoryRequest,
) -> Result<crate::models::dto::CategoryWithCount, String> {
    // 获取分类服务单例实例
    let category_service = get_category_service().await;
    // 调用服务层方法更新分类
    category_service
        .update_category(
            request.category_id,
            request.name.as_deref(),
            request.color.as_deref(),
        )
        .await
}

/// 根据仓库ID获取关联的分类（已废弃）
///
/// 该接口目前被注释掉，暂未启用。
// #[command]
// pub async fn cmd_get_repo_categories(repo_id: i64) -> Result<Vec<CategoryWithCount>, String> {
//     let category_service = get_category_service().await;
//     category_service.get_categories_by_repo_id(repo_id).await
// }

/// 为仓库更新分类关联
///
/// 根据请求中的仓库ID和分类ID列表，更新仓库与分类的关联关系。
/// 会删除不再关联的分类，并添加新的分类关联。
///
/// # Arguments
/// - `request`: 更新分类关联的请求对象，包含仓库ID和分类ID列表
///
/// # Returns
/// - `Result<(), String>`: 成功返回空，失败返回错误信息
#[command]
pub async fn cmd_update_repo_categories(
    request: crate::models::dto::UpdateRepoCategoriesRequest,
) -> Result<(), String> {
    // 获取分类服务单例实例
    let category_service = get_category_service().await;
    // 调用服务层方法更新仓库分类关联
    category_service.update_repo_categories(request).await
}

/// 创建分类
///
/// 根据指定的名称和颜色创建一个新的分类。
///
/// # Arguments
/// - `name`: 分类名称
/// - `color`: 分类颜色（十六进制格式，如 #7800ce）
///
/// # Returns
/// - `Result<CategoryWithCount, String>`: 创建成功的分类信息或错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_create_category(
    name: String,
    color: String,
) -> Result<crate::models::dto::CategoryWithCount, String> {
    // 获取分类服务单例实例
    let category_service = get_category_service().await;
    // 调用服务层方法创建分类，传入名称和颜色的引用
    category_service.create_category(&name, &color).await
}

/// 删除分类
///
/// 根据分类ID删除指定的分类，同时删除该分类与所有仓库的关联关系。
///
/// # Arguments
/// - `category_id`: 要删除的分类ID
///
/// # Returns
/// - `Result<(), String>`: 成功返回空，失败返回错误信息
#[command(rename_all = "snake_case")]
pub async fn cmd_delete_category(category_id: i64) -> Result<(), String> {
    // 获取分类服务单例实例
    let category_service = get_category_service().await;
    // 调用服务层方法删除分类
    category_service.delete_category(category_id).await
}
