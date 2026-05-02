//! 服务层接口定义

use crate::models::config::{AuthConfig, Config};
use crate::models::dto::{
    CategoryPageRequest, CategoryPageResponse, CategoryWithCount, CollectionPageRequest,
    CollectionPageResponse, CollectionWithRepoCount, UpdateCollectionReposRequest,
    UpdateRepoCategoriesRequest,
};
use crate::models::entity::Collection;
use async_trait::async_trait;
use serde_json::Value;

/// 笔记服务接口
#[async_trait]
pub trait NoteService {
    /// 保存笔记
    async fn save_note(
        &self,
        id: i64,
        github_id: i64,
        owner: &str,
        repo_name: &str,
        note_name: &str,
        content: &str,
    ) -> Result<Value, String>;

    /// 读取笔记
    async fn read_note(
        &self,
        owner: String,
        repo_name: String,
        note_name: String,
    ) -> Result<String, String>;

    /// 获取仓库笔记列表
    async fn get_notes_by_repo(&self, github_id: i64) -> Result<Value, String>;

    /// 获取默认笔记名称
    async fn get_default_note_name_service(
        &self,
        owner: &str,
        repo_name: &str,
    ) -> Result<String, String>;
}

/// 分类服务接口
#[async_trait]
pub trait CategoryService {
    /// 获取所有分类及其关联的仓库数量
    async fn get_all_categories(&self) -> Result<Vec<CategoryWithCount>, String>;

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
        request: UpdateRepoCategoriesRequest,
    ) -> Result<(), String>;

    /// 创建分类
    async fn create_category(&self, name: &str, color: &str) -> Result<CategoryWithCount, String>;

    /// 删除分类
    async fn delete_category(&self, category_id: i64) -> Result<(), String>;
}

/// 认证服务接口
#[async_trait]
pub trait AuthService {
    /// 登录
    async fn login(&self, token: &str) -> Result<Value, String>;
    /// 登出
    async fn logout(&self) -> Result<(), String>;
    /// 获取认证配置
    async fn get_auth_config(&self) -> Result<Option<AuthConfig>, String>;
}

/// 配置服务接口
#[async_trait]
pub trait ConfigService {
    /// 获取配置
    async fn get_config(&self) -> Result<Value, String>;
    /// 保存配置
    async fn save_config(&self, config: Config) -> Result<(), String>;
    /// 获取默认配置
    async fn get_default_config(&self) -> Result<Value, String>;
    /// 重置配置（保留认证信息）
    async fn reset_config(&self) -> Result<(), String>;
}

/// 仓库服务接口
#[async_trait]
pub trait RepoService: Send + Sync {
    /// 获取仓库列表
    /// - deleted: 是否查询已删除的记录，默认 false（查询未删除）
    /// - sort_order: 排序方向（asc/desc）
    /// - categories: 分类过滤条件（可选，支持多选）
    async fn get_repos(
        &self,
        page: Option<i64>,
        page_size: Option<i64>,
        sort: Option<String>,
        language: Option<String>,
        categories: Option<Vec<String>>,
        deleted: Option<bool>,
        sort_order: Option<String>,
    ) -> Result<Value, String>;

    /// 获取仓库过滤器
    /// - deleted: 是否查询已删除的记录，默认 false（查询未删除）
    /// - categories: 可选的分类过滤条件（支持多选）
    /// - language: 可选的语言过滤条件
    async fn get_repo_filters(
        &self,
        deleted: Option<bool>,
        categories: Option<Vec<String>>,
        language: Option<String>,
    ) -> Result<Value, String>;

    /// 搜索仓库
    /// - deleted: 是否查询已删除的记录，默认 false（查询未删除）
    async fn search_repos(
        &self,
        query: &str,
        page: Option<i64>,
        page_size: Option<i64>,
        deleted: Option<bool>,
    ) -> Result<Value, String>;

    /// 同步 FTS
    async fn sync_fts(&self) -> Result<String, String>;

    /// 获取 README
    async fn get_repo_readme(&self, owner: &str, repo_name: &str) -> Result<String, String>;

    /// 获取最近一周更新的仓库
    async fn get_recent_updated_repos(&self) -> Result<Value, String>;

    /// 获取状态分类仓库（归档、低活跃、废弃）
    async fn get_repo_status_categories(&self) -> Result<Value, String>;

    /// 获取统计信息
    async fn get_repo_stats(&self) -> Result<Value, String>;
}

/// 合集服务接口
#[async_trait]
pub trait CollectionService: Send + Sync {
    /// 创建合集
    async fn create_collection(
        &self,
        name: &str,
        description: &str,
        color: &str,
    ) -> Result<CollectionWithRepoCount, String>;

    /// 更新合集
    async fn update_collection(
        &self,
        collection_id: i64,
        name: Option<&str>,
        description: Option<&str>,
        color: Option<&str>,
    ) -> Result<CollectionWithRepoCount, String>;

    /// 删除合集
    async fn delete_collection(&self, collection_id: i64) -> Result<(), String>;

    /// 根据ID获取合集
    async fn get_collection_by_id(
        &self,
        collection_id: i64,
    ) -> Result<CollectionWithRepoCount, String>;

    /// 获取所有合集
    async fn get_all_collections(&self) -> Result<Vec<CollectionWithRepoCount>, String>;

    /// 分页获取合集列表
    async fn get_collections_paged(
        &self,
        request: CollectionPageRequest,
    ) -> Result<CollectionPageResponse, String>;

    /// 获取合集关联的所有仓库
    async fn get_repos_by_collection(&self, collection_id: i64) -> Result<Value, String>;

    /// 获取仓库关联的所有合集
    async fn get_collections_by_repo(&self, github_id: i64) -> Result<Vec<Collection>, String>;

    /// 更新合集与仓库的关联关系
    async fn update_collection_repos(
        &self,
        request: UpdateCollectionReposRequest,
    ) -> Result<(), String>;

    /// 检查仓库是否已在合集中
    async fn is_repo_in_collection(
        &self,
        github_id: i64,
        collection_id: i64,
    ) -> Result<bool, String>;
}
