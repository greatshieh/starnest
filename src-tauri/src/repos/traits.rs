//! 仓库层接口定义

use crate::models::config::{AuthConfig, Config};
use crate::models::dto::{
    CategoryPageRequest, CategoryPageResponse, CategoryWithCount, CollectionPageRequest,
    CollectionPageResponse, CollectionWithRepoCount, GitHubRepoResponse, PaginatedRepos,
    RepoFilters,
};
use crate::models::entity::{Category, Collection, RepoNote};
use async_trait::async_trait;

/// 笔记仓库接口
#[async_trait]
pub trait NoteRepo {
    /// 创建笔记
    async fn create_note(
        &self,
        repo_id: i64,
        note_name: &str,
        folder: &str,
        content: &str,
    ) -> Result<RepoNote, String>;

    /// 更新笔记
    async fn update_note(
        &self,
        id: i64,
        folder: &str,
        original_name: &str,
        new_name: &str,
        content: &str,
    ) -> Result<RepoNote, String>;

    /// 根据ID获取笔记
    async fn get_note_by_id(&self, id: i64) -> Result<RepoNote, String>;

    /// 根据仓库ID获取笔记列表
    async fn get_notes_by_repo_id(&self, repo_id: i64) -> Result<Vec<RepoNote>, String>;

    /// 读取笔记内容
    async fn read_note_content(&self, folder: &str, note_name: &str) -> Result<String, String>;

    /// 获取默认笔记名称
    async fn get_default_note_name(&self, folder: &str) -> Result<String, String>;
}

/// 分类仓库接口
#[async_trait]
pub trait CategoryRepo {
    /// 获取所有分类及其关联的仓库数量
    async fn get_all_categories_with_count(&self) -> Result<Vec<CategoryWithCount>, String>;

    /// 分页获取分类列表
    async fn get_categories_paged(
        &self,
        request: CategoryPageRequest,
    ) -> Result<CategoryPageResponse, String>;

    /// 更新分类信息
    async fn update_category(
        &self,
        category_id: i64,
        name: Option<&str>,
        color: Option<&str>,
    ) -> Result<CategoryWithCount, String>;

    /// 为仓库更新分类关联
    async fn update_repo_categories(
        &self,
        repo_id: i64,
        category_ids: &[i64],
    ) -> Result<(), String>;

    /// 创建分类
    async fn create_category(&self, name: &str, color: &str) -> Result<Category, String>;

    /// 删除分类
    async fn delete_category(&self, category_id: i64) -> Result<(), String>;
}

/// 认证仓库接口
#[async_trait]
pub trait AuthRepo {
    /// 保存认证配置
    async fn save_auth_config(&self, auth: AuthConfig) -> Result<(), String>;
    /// 清除认证配置
    async fn clear_auth_config(&self) -> Result<(), String>;
    /// 获取认证配置
    async fn get_auth_config(&self) -> Result<Option<AuthConfig>, String>;
}

/// 配置仓库接口
#[async_trait]
pub trait ConfigRepo {
    /// 获取配置
    async fn load_config(&self) -> Result<Config, String>;
    /// 保存配置
    async fn save_config(&self, config: &Config) -> Result<(), String>;
    /// 获取默认基础路径
    // async fn get_default_base_path(&self) -> Result<std::path::PathBuf, String>;
    /// 获取笔记存储路径
    async fn get_note_storage_path(&self, config: &Config) -> Result<std::path::PathBuf, String>;
    /// 获取数据库文件路径
    async fn get_database_path(&self, config: &Config) -> Result<std::path::PathBuf, String>;
}

/// 仓库仓储接口
#[async_trait]
pub trait RepoRepo: Send + Sync {
    /// 获取分页仓库列表
    /// - deleted: 是否查询已删除的记录，默认 false（查询未删除）
    /// - sort_order: 排序方向（asc/desc）
    /// - categories: 分类过滤条件（可选，支持多选）
    async fn get_repos(
        &self,
        page: i64,
        page_size: i64,
        sort: &str,
        language: Option<&str>,
        categories: Option<&[String]>,
        deleted: bool,
        sort_order: &str,
    ) -> Result<PaginatedRepos, String>;

    /// 获取仓库过滤器
    /// - deleted: 是否查询已删除的记录，默认 false（查询未删除）
    /// - categories: 可选的分类过滤条件（支持多选）
    /// - language: 可选的语言过滤条件
    async fn get_repo_filters(
        &self,
        deleted: bool,
        categories: Option<&[String]>,
        language: Option<&str>,
    ) -> Result<RepoFilters, String>;

    /// 搜索仓库
    /// - deleted: 是否查询已删除的记录，默认 false（查询未删除）
    async fn search_repos(
        &self,
        query: &str,
        page: i64,
        page_size: i64,
        deleted: bool,
    ) -> Result<PaginatedRepos, String>;

    /// 同步仓库到数据库
    async fn sync_repos_to_db(
        &self,
        repos: Vec<GitHubRepoResponse>,
    ) -> Result<(usize, usize, usize, Vec<String>), String>;

    /// 同步 FTS 表
    async fn sync_fts(&self) -> Result<i64, String>;

    /// 获取最近一周更新的仓库
    async fn get_recent_updated_repos(
        &self,
    ) -> Result<Vec<crate::models::entity::Repository>, String>;

    /// 获取状态分类仓库（归档、低活跃、废弃）
    async fn get_repo_status_categories(
        &self,
    ) -> Result<Vec<crate::models::entity::Repository>, String>;

    /// 获取统计信息
    /// - 总仓库数（deleted_at IS NULL）
    /// - 分类标签数量
    /// - 未分类仓库数量
    /// - 7天内标星的仓库数量
    async fn get_repo_stats(&self) -> Result<(i64, i64, i64, i64), String>;
}

/// 合集仓库接口
#[async_trait]
pub trait CollectionRepo: Send + Sync {
    /// 创建合集
    async fn create_collection(&self, name: &str, description: &str, color: &str) -> Result<Collection, String>;

    /// 更新合集
    async fn update_collection(
        &self,
        collection_id: i64,
        name: Option<&str>,
        description: Option<&str>,
        color: Option<&str>,
    ) -> Result<CollectionWithRepoCount, String>;

    /// 删除合集（不会删除关联的仓库）
    async fn delete_collection(&self, collection_id: i64) -> Result<(), String>;

    /// 根据ID获取合集
    async fn get_collection_by_id(
        &self,
        collection_id: i64,
    ) -> Result<CollectionWithRepoCount, String>;

    /// 获取所有合集（带仓库数量）
    async fn get_all_collections(&self) -> Result<Vec<CollectionWithRepoCount>, String>;

    /// 分页获取合集列表（支持关键字搜索）
    async fn get_collections_paged(
        &self,
        request: CollectionPageRequest,
    ) -> Result<CollectionPageResponse, String>;

    /// 获取合集关联的所有仓库
    async fn get_repos_by_collection(
        &self,
        collection_id: i64,
    ) -> Result<Vec<crate::models::entity::Repository>, String>;

    /// 获取仓库关联的所有合集
    async fn get_collections_by_repo(&self, github_id: i64) -> Result<Vec<Collection>, String>;

    /// 更新合集与仓库的关联关系
    async fn update_collection_repos(
        &self,
        collection_id: i64,
        github_ids: &[i64],
    ) -> Result<(), String>;

    /// 检查仓库是否已在合集中
    async fn is_repo_in_collection(
        &self,
        github_id: i64,
        collection_id: i64,
    ) -> Result<bool, String>;
}
