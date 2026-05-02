//! 数据传输对象
//! 定义前后端数据交换的结构

use serde::{Deserialize, Serialize};

/// 获取仓库请求
#[derive(Debug, Serialize, Deserialize)]
pub struct GetReposRequest {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort: Option<String>,
    pub language: Option<String>,
    pub categories: Option<Vec<String>>,
    pub deleted: Option<bool>,
    pub sort_order: Option<String>,
}

/// 获取仓库筛选条件请求
#[derive(Debug, Serialize, Deserialize)]
pub struct GetRepoFiltersRequest {
    pub deleted: Option<bool>,
    pub categories: Option<Vec<String>>,
    pub language: Option<String>,
}

/// 搜索仓库请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchReposRequest {
    pub query: String,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub deleted: Option<bool>,
}

/// 获取 README 请求
#[derive(Debug, Serialize, Deserialize)]
pub struct GetReadmeRequest {
    pub owner: String,
    pub repo_name: String,
}

/// 同步仓库请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncReposRequest {
    pub token: String,
}

/// 获取仓库事件按日期请求
#[derive(Debug, Serialize, Deserialize)]
pub struct GetRepoEventsByDateRequest {
    pub repo_id: i64,
}

/// 从 GitHub 获取活动请求
#[derive(Debug, Serialize, Deserialize)]
pub struct FetchRepoActivitiesRequest {
    pub owner: String,
    pub repo_name: String,
}

/// 标星/取消标星仓库请求
#[derive(Debug, Serialize, Deserialize)]
pub struct StarRepoRequest {
    pub owner: String,
    pub repo_name: String,
}

/// 软删除/恢复仓库请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SoftDeleteRepoRequest {
    pub owner: String,
    pub repo_name: String,
}
pub type RestoreRepoRequest = SoftDeleteRepoRequest;

/// 同步仓库响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncReposResponse {
    pub synced_count: usize,
    pub updated_count: usize,
    pub new_count: usize,
    pub last_sync: String,
    pub topics: Vec<String>,
    pub total_count: i64,
}

/// 搜索仓库响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchReposResponse {
    pub repos: Vec<crate::models::entity::Repository>,
    pub total: i64,
    pub page: i64,
}

/// 获取仓库响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetReposResponse {
    pub repos: Vec<crate::models::entity::Repository>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

/// 仓库统计响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepoStatsResponse {
    pub total_repos: i64,
    pub category_count: i64,
    pub uncategorized_count: i64,
    pub recent_starred_count: i64,
}

/// 登录请求
// #[derive(Debug, Serialize, Deserialize)]
// pub struct LoginRequest {
//     pub token: String,
// }

/// 登录响应
// #[derive(Debug, Serialize, Deserialize)]
// pub struct LoginResponse {
//     pub name: String,
//     pub avatar: String,
// }

/// 仓库筛选条件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepoFilters {
    pub languages: Vec<String>,
    pub categories: Vec<String>,
}

/// 获取仓库请求
// #[derive(Debug, Serialize, Deserialize)]
// pub struct GetReposRequest {
//     pub filters: RepoFilters,
//     pub page: i32,
//     pub limit: i32,
//     pub sort: String,
//     pub order: String,
// }

/// 同步仓库响应
// #[derive(Debug, Serialize, Deserialize)]
// pub struct SyncReposResponse {
//     pub total: i32,
//     pub synced: i32,
//     pub failed: i32,
//     pub message: String,
// }

/// 保存笔记请求
// #[derive(Debug, Serialize, Deserialize)]
// pub struct SaveNoteRequest {
//     pub id: i64,
//     pub github_id: i64,
//     pub owner: String,
//     pub repo_name: String,
//     pub note_name: String,
//     pub content: String,
// }

/// GitHub Starred 响应（包含 starred_at）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubStarredResponse {
    pub starred_at: String,
    pub repo: GitHubRepoResponse,
}

/// GitHub 仓库响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubRepoResponse {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    pub owner: GitHubOwnerResponse,
    pub description: Option<String>,
    pub stargazers_count: i64,
    pub forks_count: i64,
    pub language: Option<String>,
    pub topics: Vec<String>,
    pub archived: bool,
    pub created_at: String,
    pub pushed_at: String,
    pub license: Option<GitHubLicenseResponse>,
    pub html_url: String,
    pub clone_url: String,
    pub homepage: Option<String>,
    pub open_issues: i64,
    pub starred_at: Option<String>,
}

/// GitHub 许可证响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubLicenseResponse {
    pub name: String,
}

/// GitHub 所有者响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubOwnerResponse {
    pub login: String,
    pub avatar_url: String,
}

/// GitHub 用户响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubUserResponse {
    pub login: String,
    pub name: Option<String>,
    pub avatar_url: String,
}

/// 分页仓库响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaginatedRepos {
    pub repos: Vec<crate::models::entity::Repository>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

/// 仓库同步结果
#[derive(Debug, Serialize, Deserialize)]
pub struct RepoSyncResult {
    pub success: bool,
    pub message: String,
    pub synced_count: usize,
    pub updated_count: usize,
    pub new_count: usize,
    pub last_sync: String,
    pub topics: Vec<String>,
    pub total_count: i64,
}

/// 分类响应（包含仓库数量和更新时间）
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CategoryWithCount {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub repo_count: i64,
    pub updated_at: String,
}

/// 分类分页请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryPageRequest {
    pub page: i64,
    pub page_size: i64,
    pub search_keyword: Option<String>,
    pub sort_by: String,
    pub sort_order: String,
}

/// 分类分页响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryPageResponse {
    pub categories: Vec<CategoryWithCount>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub total_pages: i64,
}

/// 更新分类请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCategoryRequest {
    pub category_id: i64,
    pub name: Option<String>,
    pub color: Option<String>,
}

/// 仓库分类关联请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRepoCategoriesRequest {
    pub repo_id: i64,
    pub category_ids: Vec<i64>,
}

/// GitHub 活动响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubActivity {
    #[serde(rename = "id")]
    pub activity_id: String,
    #[serde(rename = "type")]
    pub activity_type: String,
    pub actor: GitHubActor,
    pub repo: GitHubActivityRepo,
    pub payload: serde_json::Value,
    pub created_at: String,
}

/// GitHub 活动执行者
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubActor {
    pub id: i64,
    pub login: String,
    pub avatar_url: String,
}

/// GitHub 活动仓库信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubActivityRepo {
    pub id: i64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
}

/// 事件类型统计（前端需要的格式）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventTypeCount {
    pub r#type: String,
    pub count: u32,
}

/// 按日期聚合的事件统计（前端需要的格式）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DailyEventStats {
    pub date: String,
    pub events: Vec<EventTypeCount>,
}

/// 合集分页请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionPageRequest {
    pub page: i64,
    pub page_size: i64,
    pub search_keyword: Option<String>,
    pub sort_by: String,
    pub sort_order: String,
}

/// 合集响应（包含仓库数量）
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CollectionWithRepoCount {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub color: String,
    pub repo_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

/// 合集分页响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionPageResponse {
    pub collections: Vec<CollectionWithRepoCount>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub total_pages: i64,
}

/// 更新合集请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCollectionRequest {
    pub collection_id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
}

/// 更新合集仓库关联请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCollectionReposRequest {
    pub collection_id: i64,
    pub github_ids: Vec<i64>,
}
