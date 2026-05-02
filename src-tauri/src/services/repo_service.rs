//! 仓库服务模块
//!
//! 该模块负责处理仓库相关的业务逻辑，作为业务层协调仓储层和处理器层之间的数据流转。
//! 提供仓库的查询、搜索、统计等核心功能。

use crate::infrastructure::external_api::github_client::get_repo_readme;
use crate::repos::traits::RepoRepo;
use crate::services::traits::RepoService;
use async_trait::async_trait;
use serde_json::Value;
use tokio::sync::OnceCell;

/// 仓库服务实现结构体
///
/// 封装仓库相关的业务逻辑，依赖仓库仓储接口进行数据访问。
pub struct RepoServiceImpl {
    /// 仓库仓储接口，用于数据库操作
    repo_repo: Box<dyn RepoRepo + Send + Sync>,
}

/// 仓库服务单例实例
///
/// 使用 OnceCell 确保全局只有一个仓库服务实例，避免重复创建。
static REPO_SERVICE: OnceCell<Box<dyn RepoService + Send + Sync>> = OnceCell::const_new();

#[async_trait]
impl RepoService for RepoServiceImpl {
    /// 分页获取仓库列表
    ///
    /// 根据分页参数、排序方式、语言和分类筛选条件获取仓库列表。
    ///
    /// # Arguments
    /// - `page`: 页码（可选，默认1）
    /// - `page_size`: 每页数量（可选，默认30）
    /// - `sort`: 排序字段（可选，默认 'recent_stars'）
    /// - `language`: 语言筛选（可选）
    /// - `categories`: 分类筛选（可选，支持多选）
    /// - `deleted`: 是否包含已删除的仓库（可选，默认false）
    /// - `sort_order`: 排序方向（可选，asc/desc，默认desc）
    ///
    /// # Returns
    /// - `Result<Value, String>`: 仓库列表数据（JSON格式）或错误信息
    async fn get_repos(
        &self,
        page: Option<i64>,
        page_size: Option<i64>,
        sort: Option<String>,
        language: Option<String>,
        categories: Option<Vec<String>>,
        deleted: Option<bool>,
        sort_order: Option<String>,
    ) -> Result<Value, String> {
        // 设置默认参数值
        let page = page.unwrap_or(1);
        let page_size = page_size.unwrap_or(30);
        let sort = sort.unwrap_or_else(|| "recent_stars".to_string());
        let language = language.filter(|s| !s.is_empty());
        let categories = categories.filter(|v| !v.is_empty());
        let deleted = deleted.unwrap_or(false);
        let sort_order = sort_order.unwrap_or_else(|| "desc".to_string());

        // 委托给仓储层获取仓库列表
        let repos = self
            .repo_repo
            .get_repos(
                page,
                page_size,
                &sort,
                language.as_deref(),
                categories.as_deref(),
                deleted,
                &sort_order,
            )
            .await?;
        // 将结果序列化为 JSON 返回
        Ok(serde_json::to_value(repos).unwrap())
    }

    /// 获取仓库筛选条件
    ///
    /// 返回当前数据库中可用的语言列表和分类列表，用于前端筛选器。
    /// 支持按categories/language过滤获取关联选项。
    ///
    /// # Arguments
    /// - `deleted`: 是否包含已删除的仓库（可选，默认false）
    /// - `categories`: 可选的分类过滤条件（支持多选）
    /// - `language`: 可选的语言过滤条件
    ///
    /// # Returns
    /// - `Result<Value, String>`: 筛选条件数据（JSON格式）或错误信息
    async fn get_repo_filters(
        &self,
        deleted: Option<bool>,
        categories: Option<Vec<String>>,
        language: Option<String>,
    ) -> Result<Value, String> {
        // 设置默认参数值
        let deleted = deleted.unwrap_or(false);
        let categories = categories.filter(|v| !v.is_empty());
        let language = language.filter(|s| !s.is_empty());
        // 委托给仓储层获取筛选条件
        let filters = self
            .repo_repo
            .get_repo_filters(deleted, categories.as_deref(), language.as_deref())
            .await?;
        // 将结果序列化为 JSON 返回
        Ok(serde_json::to_value(filters).unwrap())
    }

    /// 搜索仓库
    ///
    /// 根据关键词搜索仓库名称、描述等字段。
    ///
    /// # Arguments
    /// - `query`: 搜索关键词
    /// - `page`: 页码（可选，默认1）
    /// - `page_size`: 每页数量（可选，默认30）
    /// - `deleted`: 是否包含已删除的仓库（可选，默认false）
    ///
    /// # Returns
    /// - `Result<Value, String>`: 搜索结果（JSON格式）或错误信息
    async fn search_repos(
        &self,
        query: &str,
        page: Option<i64>,
        page_size: Option<i64>,
        deleted: Option<bool>,
    ) -> Result<Value, String> {
        // 设置默认参数值
        let page = page.unwrap_or(1);
        let page_size = page_size.unwrap_or(30);
        let deleted = deleted.unwrap_or(false);

        // 委托给仓储层搜索仓库
        let result = self
            .repo_repo
            .search_repos(query, page, page_size, deleted)
            .await?;
        // 将结果序列化为 JSON 返回
        Ok(serde_json::to_value(result).unwrap())
    }

    /// 同步全文搜索索引
    ///
    /// 更新 FTS（全文搜索）索引，确保搜索功能能够检索最新的仓库数据。
    ///
    /// # Returns
    /// - `Result<String, String>`: 成功消息或错误信息
    async fn sync_fts(&self) -> Result<String, String> {
        // 委托给仓储层同步 FTS 索引
        let count = self.repo_repo.sync_fts().await?;
        // 返回同步结果消息
        Ok(format!("Synced {} records to FTS table", count))
    }

    /// 获取仓库 README 内容
    ///
    /// 从 GitHub 获取指定仓库的 README.md 文件内容。
    ///
    /// # Arguments
    /// - `owner`: 仓库所有者（用户名或组织名）
    /// - `repo_name`: 仓库名称
    ///
    /// # Returns
    /// - `Result<String, String>`: README 内容（Markdown 格式）或错误信息
    async fn get_repo_readme(&self, owner: &str, repo_name: &str) -> Result<String, String> {
        // 委托给 GitHub 客户端获取 README
        get_repo_readme(owner, repo_name).await
    }

    /// 获取最近一周更新的仓库
    ///
    /// 返回最近一周内有推送活动的仓库列表，按更新时间降序排列。
    ///
    /// # Returns
    /// - `Result<Value, String>`: 最近更新的仓库列表（JSON格式）或错误信息
    async fn get_recent_updated_repos(&self) -> Result<Value, String> {
        // 委托给仓储层获取最近更新的仓库
        let repos = self.repo_repo.get_recent_updated_repos().await?;
        // 将结果序列化为 JSON 返回
        Ok(serde_json::to_value(repos).unwrap())
    }

    /// 获取状态分类仓库（归档、低活跃、废弃）
    ///
    /// 返回按状态分类的仓库统计，包括归档、低活跃、废弃三种状态。
    ///
    /// # Returns
    /// - `Result<Value, String>`: 状态分类统计数据（JSON格式）或错误信息
    async fn get_repo_status_categories(&self) -> Result<Value, String> {
        // 委托给仓储层获取状态分类
        let repos = self.repo_repo.get_repo_status_categories().await?;
        // 将结果序列化为 JSON 返回
        let result = serde_json::json!(repos);
        Ok(result)
    }

    /// 获取仓库统计信息
    ///
    /// 返回仪表盘所需的统计数据，包括总仓库数、分类数、未分类仓库数、最近标星数。
    ///
    /// # Returns
    /// - `Result<Value, String>`: 统计数据（JSON格式）或错误信息
    async fn get_repo_stats(&self) -> Result<Value, String> {
        // 委托给仓储层获取统计信息
        let (total_repos, category_count, uncategorized_count, recent_starred_count) =
            self.repo_repo.get_repo_stats().await?;

        // 构建统计结果 JSON 对象
        let result = serde_json::json!({
            "total_repos": total_repos,
            "category_count": category_count,
            "uncategorized_count": uncategorized_count,
            "recent_starred_count": recent_starred_count,
        });

        Ok(result)
    }
}

impl RepoServiceImpl {
    /// 创建仓库服务实例
    ///
    /// # Arguments
    /// - `repo_repo`: 仓库仓储实例
    ///
    /// # Returns
    /// - `Self`: 仓库服务实例
    pub fn new(repo_repo: Box<dyn RepoRepo + Send + Sync>) -> Self {
        Self { repo_repo }
    }
}

/// 获取仓库服务单例实例
///
/// 使用 OnceCell 懒加载创建仓库服务单例，确保全局只有一个实例。
///
/// # Returns
/// - `&'static Box<dyn RepoService + Send + Sync>`: 仓库服务单例引用
pub async fn get_repo_service() -> &'static Box<dyn RepoService + Send + Sync> {
    REPO_SERVICE
        .get_or_init(|| async {
            // 获取仓库仓储实例
            let repo_repo = crate::repos::repo_repo::get_repo_repo().await;
            // 创建仓库服务实例并装箱
            let service: Box<dyn RepoService + Send + Sync> =
                Box::new(RepoServiceImpl::new(repo_repo));
            service
        })
        .await
}
