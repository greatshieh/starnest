//! 仓库仓储模块
//!
//! 该模块负责仓库相关的数据访问操作，封装了与 SQLite 数据库的交互逻辑。
//! 提供仓库的增删改查、同步、搜索等功能，并实现了缓存机制以提高查询性能。

use crate::models::dto::{GitHubRepoResponse, PaginatedRepos, RepoFilters};
use crate::models::entity::{RepoCategory, Repository, RepositoryEntity};
use crate::repos::traits::RepoRepo;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json;
use sqlx::{Pool, Row, Sqlite};
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::sleep;

/// 根据仓库状态计算 status 字段
///
/// 规则：
/// 1. archived 为 true → "archived"（已归档）
/// 2. pushed_at 超过1年 → "deprecated"（已废弃）
/// 3. pushed_at 超过90天但不超过1年 → "inactive"（不活跃）
/// 4. 其他 → "active"（活跃）
///
/// # Arguments
/// - `archived`: 是否归档
/// - `pushed_at`: 最后推送时间（RFC3339格式）
///
/// # Returns
/// - `String`: 仓库状态（archived/deprecated/inactive/active）
fn calculate_repo_status(archived: bool, pushed_at: &str) -> String {
    // 如果仓库已归档，直接返回 archived
    if archived {
        return "archived".to_string();
    }

    // 解析推送时间并计算状态
    match DateTime::parse_from_rfc3339(pushed_at) {
        Ok(pushed_dt) => {
            let now = Utc::now();
            let ninety_days_ago = now - chrono::Duration::days(90);
            let one_year_ago = now - chrono::Duration::days(365);
            let pushed_utc = pushed_dt.with_timezone(&Utc);

            // 根据时间间隔判断状态
            if pushed_utc < one_year_ago {
                "deprecated".to_string()
            } else if pushed_utc < ninety_days_ago {
                "inactive".to_string()
            } else {
                "active".to_string()
            }
        }
        // 解析失败时默认返回 active
        Err(_) => "active".to_string(),
    }
}

/// 缓存条目结构体
///
/// 用于存储带有时间戳的缓存数据，支持过期判断。
struct CacheEntry<T> {
    /// 缓存的数据
    data: T,
    /// 缓存创建时间戳
    timestamp: DateTime<Utc>,
}

/// 仓库缓存管理器
///
/// 管理仓库列表和过滤器的缓存，支持过期自动清理。
struct RepoCache {
    /// 过滤器缓存（带读写锁）
    filters: RwLock<Option<CacheEntry<RepoFilters>>>,
    /// 仓库列表缓存（带读写锁），key 为缓存键
    repos: RwLock<HashMap<String, CacheEntry<PaginatedRepos>>>,
}

impl RepoCache {
    /// 创建新的缓存管理器实例
    fn new() -> Self {
        RepoCache {
            filters: RwLock::new(None),
            repos: RwLock::new(HashMap::new()),
        }
    }

    /// 生成仓库列表缓存的唯一键
    ///
    /// # Arguments
    /// - `page`: 页码
    /// - `page_size`: 每页数量
    /// - `sort`: 排序字段
    /// - `language`: 语言过滤条件（可选）
    /// - `categories`: 分类过滤条件（可选，支持多选）
    /// - `sort_order`: 排序方向（asc/desc）
    ///
    /// # Returns
    /// - `String`: 缓存键
    fn get_cache_key(
        page: i64,
        page_size: i64,
        sort: &str,
        language: Option<&str>,
        categories: Option<&[String]>,
        sort_order: &str,
    ) -> String {
        let categories_str = categories.map(|cats| cats.join(",")).unwrap_or_default();
        format!(
            "page={}_size={}_sort={}_lang={}_cats={}_order={}",
            page,
            page_size,
            sort,
            language.unwrap_or(""),
            categories_str,
            sort_order
        )
    }

    /// 获取仓库列表缓存（2分钟内有效）
    ///
    /// # Arguments
    /// - `page`: 页码
    /// - `page_size`: 每页数量
    /// - `sort`: 排序字段
    /// - `language`: 语言过滤条件（可选）
    /// - `categories`: 分类过滤条件（可选，支持多选）
    /// - `sort_order`: 排序方向（asc/desc）
    ///
    /// # Returns
    /// - `Option<PaginatedRepos>`: 缓存的仓库列表（如果未过期）
    async fn get_repos(
        &self,
        page: i64,
        page_size: i64,
        sort: &str,
        language: Option<&str>,
        categories: Option<&[String]>,
        sort_order: &str,
    ) -> Option<PaginatedRepos> {
        let key = Self::get_cache_key(page, page_size, sort, language, categories, sort_order);
        if let Some(entry) = self.repos.read().await.get(&key) {
            // 检查缓存是否在2分钟内有效
            if (Utc::now() - entry.timestamp).num_minutes() < 2 {
                return Some(entry.data.clone());
            }
        }
        None
    }

    /// 设置仓库列表缓存
    ///
    /// # Arguments
    /// - `page`: 页码
    /// - `page_size`: 每页数量
    /// - `sort`: 排序字段
    /// - `language`: 语言过滤条件（可选）
    /// - `categories`: 分类过滤条件（可选，支持多选）
    /// - `sort_order`: 排序方向（asc/desc）
    /// - `repos`: 仓库列表数据
    async fn set_repos(
        &self,
        page: i64,
        page_size: i64,
        sort: &str,
        language: Option<&str>,
        categories: Option<&[String]>,
        sort_order: &str,
        repos: PaginatedRepos,
    ) {
        let key = Self::get_cache_key(page, page_size, sort, language, categories, sort_order);
        self.repos.write().await.insert(
            key,
            CacheEntry {
                data: repos,
                timestamp: Utc::now(),
            },
        );
    }

    /// 清空所有缓存
    async fn invalidate(&self) {
        *self.filters.write().await = None;
        *self.repos.write().await = HashMap::new();
    }
}

/// 全局仓库缓存实例（单例）
static REPO_CACHE: std::sync::OnceLock<RepoCache> = std::sync::OnceLock::new();

/// 获取全局仓库缓存实例
///
/// # Returns
/// - `&'static RepoCache`: 全局缓存实例引用
fn get_cache() -> &'static RepoCache {
    REPO_CACHE.get_or_init(RepoCache::new)
}

/// 仓库仓库实现结构体
///
/// 封装仓库相关的数据访问操作，持有数据库连接池引用。
pub struct RepoRepoImpl {
    /// SQLite 数据库连接池引用
    pool: &'static Pool<Sqlite>,
}

impl RepoRepoImpl {
    /// 创建仓库仓库实例
    ///
    /// # Arguments
    /// - `pool`: SQLite 数据库连接池引用
    ///
    /// # Returns
    /// - `Self`: 仓库仓库实例
    pub fn new(pool: &'static Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RepoRepo for RepoRepoImpl {
    /// 获取仓库列表（分页）
    ///
    /// 根据分页参数、排序方式、语言和分类过滤条件获取仓库列表。
    /// 支持缓存机制，未删除的仓库列表会被缓存2分钟。
    ///
    /// # Arguments
    /// - `page`: 页码（从1开始）
    /// - `page_size`: 每页数量
    /// - `sort`: 排序字段（star_count/fork_count/update_time/recent_stars）
    /// - `language`: 语言过滤条件（可选）
    /// - `categories`: 分类过滤条件（可选，支持多选）
    /// - `deleted`: 是否查询已删除的仓库
    /// - `sort_order`: 排序方向（asc/desc）
    ///
    /// # Returns
    /// - `Result<PaginatedRepos, String>`: 分页仓库列表或错误信息
    async fn get_repos(
        &self,
        page: i64,
        page_size: i64,
        sort: &str,
        language: Option<&str>,
        categories: Option<&[String]>,
        deleted: bool,
        sort_order: &str,
    ) -> Result<PaginatedRepos, String> {
        let cache = get_cache();

        // 如果查询未删除的仓库，先尝试从缓存获取
        if !deleted {
            if let Some(cached) = cache
                .get_repos(page, page_size, sort, language, categories, sort_order)
                .await
            {
                return Ok(cached);
            }
        }

        // 计算分页偏移量
        let offset = (page - 1) * page_size;

        // 根据排序字段执行不同的查询
        let (repo_entities, total) = match sort {
            "star_count" => {
                query_repos_by_sort(
                    self.pool,
                    "stargazers_count",
                    offset,
                    page_size,
                    language,
                    categories,
                    deleted,
                    sort_order,
                )
                .await?
            }
            "fork_count" => {
                query_repos_by_sort(
                    self.pool,
                    "forks_count",
                    offset,
                    page_size,
                    language,
                    categories,
                    deleted,
                    sort_order,
                )
                .await?
            }
            "update_time" => {
                query_repos_by_sort(
                    self.pool,
                    "pushed_at",
                    offset,
                    page_size,
                    language,
                    categories,
                    deleted,
                    sort_order,
                )
                .await?
            }
            "recent_stars" | _ => {
                query_repos_by_sort(
                    self.pool,
                    "starred_at",
                    offset,
                    page_size,
                    language,
                    categories,
                    deleted,
                    sort_order,
                )
                .await?
            }
        };

        // 将实体转换为 Repository 类型，并获取关联的分类
        let mut repos = Vec::with_capacity(repo_entities.len());
        for entity in repo_entities {
            let mut repo = Repository::from(entity);
            repo.categories = get_categories_by_repo_id(self.pool, repo.github_id).await?;
            repos.push(repo);
        }

        // 构建分页结果
        let result = PaginatedRepos {
            repos,
            total,
            page,
            page_size,
        };

        // 如果查询未删除的仓库，缓存结果
        if !deleted {
            cache
                .set_repos(
                    page,
                    page_size,
                    sort,
                    language,
                    categories,
                    sort_order,
                    result.clone(),
                )
                .await;
        }

        Ok(result)
    }

    /// 获取仓库过滤器（语言和分类）
    ///
    /// 获取所有仓库使用的语言和分类列表。
    /// 支持缓存机制，过滤器会被缓存5分钟。
    /// 支持按categories/language过滤获取关联选项。
    ///
    /// # Arguments
    /// - `deleted`: 是否查询已删除的仓库
    /// - `categories`: 可选的分类过滤条件（支持多选）
    /// - `language`: 可选的语言过滤条件
    ///
    /// # Returns
    /// - `Result<RepoFilters, String>`: 过滤器信息或错误信息
    async fn get_repo_filters(
        &self,
        deleted: bool,
        _categories: Option<&[String]>,
        _language: Option<&str>,
    ) -> Result<RepoFilters, String> {
        // 构建删除状态条件
        let deleted_condition = if deleted {
            "deleted_at IS NOT NULL"
        } else {
            "deleted_at IS NULL"
        };

        // 查询语言列表（去重）- 从JSON数组中提取语言
        let languages: Vec<String> = {
            let query = format!(
                "SELECT json_group_array(DISTINCT j1.key) AS language_list
                FROM repositories
                CROSS JOIN json_each(repositories.language) AS j1
                WHERE {}
                    AND repositories.language IS NOT NULL
                    AND repositories.language != ''
                    AND repositories.language != '[]'",
                deleted_condition
            );

            let result: Option<String> = sqlx::query_scalar(&query)
                .fetch_optional(self.pool)
                .await
                .map_err(|e| format!("Failed to fetch languages: {:?}", e))?;

            match result {
                Some(json_str) => serde_json::from_str(&json_str).unwrap_or_else(|_| Vec::new()),
                None => Vec::new(),
            }
        };

        // 查询分类列表（去重）
        let categories: Vec<String> = {
            let query = "SELECT DISTINCT name FROM categories";

            sqlx::query_scalar::<_, String>(&query)
                .fetch_all(self.pool)
                .await
                .map_err(|e| format!("Failed to fetch categories: {:?}", e))?
        };

        // 构建过滤器结果
        let filters = RepoFilters {
            languages,
            categories,
        };

        Ok(filters)
    }

    /// 搜索仓库（全文搜索）
    ///
    /// 使用 FTS（全文搜索）索引搜索仓库，支持按关键词搜索。
    ///
    /// # Arguments
    /// - `query`: 搜索关键词
    /// - `page`: 页码
    /// - `page_size`: 每页数量
    /// - `deleted`: 是否查询已删除的仓库
    ///
    /// # Returns
    /// - `Result<PaginatedRepos, String>`: 搜索结果或错误信息
    async fn search_repos(
        &self,
        query: &str,
        page: i64,
        page_size: i64,
        deleted: bool,
    ) -> Result<PaginatedRepos, String> {
        // 计算分页偏移量
        let offset = (page - 1) * page_size;

        // 构建删除状态条件
        let deleted_condition = if deleted {
            "r.deleted_at IS NOT NULL"
        } else {
            "r.deleted_at IS NULL"
        };

        // 执行 FTS 全文搜索
        let repo_entities = sqlx::query_as::<_, RepositoryEntity>(&format!(
            r#"
            SELECT
                r.id, r.github_id, r.full_name, r.name, r.description,
                r.stargazers_count, r.forks_count, r.language, r.topics,
                r.pushed_at, r.created_at, r.html_url, r.clone_url, r.homepage,
                r.open_issues_count, r.open_pr, r.total_pr, r.license, r.starred_at,
                r.owner_login, r.owner_avatar_url, r.learning_status,
                r.is_favorite, r.deleted_at, r.archived, r.status
            FROM repositories r
            JOIN repos_fts ON r.id = repos_fts.rowid
            WHERE repos_fts MATCH ? AND {}
            ORDER BY rank
            LIMIT ? OFFSET ?
            "#,
            deleted_condition
        ))
        .bind(query)
        .bind(page_size)
        .bind(offset)
        .fetch_all(self.pool)
        .await
        .map_err(|e| format!("Failed to search repos: {:?}", e))?;

        // 将实体转换为 Repository 类型，并获取关联的分类
        let mut repos = Vec::with_capacity(repo_entities.len());
        for entity in repo_entities {
            let mut repo = Repository::from(entity);
            repo.categories = get_categories_by_repo_id(self.pool, repo.id).await?;
            repos.push(repo);
        }

        // 查询搜索结果总数
        let total: i64 = sqlx::query_scalar(&format!(
            r#"
            SELECT COUNT(*) FROM repositories r
            JOIN repos_fts ON r.id = repos_fts.rowid
            WHERE repos_fts MATCH ? AND {}
            "#,
            deleted_condition
        ))
        .bind(query)
        .fetch_one(self.pool)
        .await
        .map_err(|e| format!("Failed to count search results: {:?}", e))?;

        Ok(PaginatedRepos {
            repos,
            total,
            page,
            page_size,
        })
    }

    /// 同步仓库到数据库
    ///
    /// 将从 GitHub API 获取的仓库数据同步到本地数据库。
    /// 支持增量同步：已存在的仓库更新，不存在的仓库插入。
    ///
    /// # Arguments
    /// - `repos`: GitHub 仓库响应列表
    /// - `cancel_flag`: 取消标志（用于中断同步）
    ///
    /// # Returns
    /// - `Result<(usize, usize, usize, Vec<String>), String>`: (同步总数, 更新数, 新增数, 所有标签) 或错误信息
    async fn sync_repos_to_db(
        &self,
        repos: Vec<GitHubRepoResponse>,
    ) -> Result<(usize, usize, usize, Vec<String>), String> {
        let mut synced_count = 0; // 同步总数
        let mut updated_count = 0; // 更新数量
        let mut new_count = 0; // 新增数量
        let mut all_topics: HashSet<String> = HashSet::new(); // 收集所有标签

        for repo in repos {
            let repo_id = repo.id;

            // 查询仓库是否已存在
            let existing: Option<i64> = execute_db_with_retry(|| async {
                sqlx::query_scalar("SELECT id FROM repositories WHERE github_id = ?")
                    .bind(repo_id)
                    .fetch_optional(self.pool)
                    .await
                    .map_err(|e| format!("{}", e))
            })
            .await?;

            // 提取许可证名称和标签
            let license_name = repo.license.as_ref().map(|l| l.name.clone());
            all_topics.extend(repo.topics.iter().cloned());

            // 序列化标签为 JSON
            let topics_json = if repo.topics.is_empty() {
                None
            } else {
                serde_json::to_string(&repo.topics).ok()
            };

            // 更新或插入仓库
            if existing.is_some() {
                // 更新仓库
                update_repo(self.pool, &repo, &topics_json, &license_name)
                    .await
                    .map_err(|e| format!("update repo failed: {:?}", e))?;
                updated_count += 1;
            } else {
                // 新增仓库
                insert_repo(self.pool, &repo, &topics_json, &license_name)
                    .await
                    .map_err(|e| format!("insert repo failed: {:?}", e))?;
                new_count += 1;
            }

            synced_count += 1;
        }

        // 将标签集合转换为向量
        let topics: Vec<String> = all_topics.into_iter().collect();

        // 清空缓存
        let cache = get_cache();
        cache.invalidate().await;

        Ok((synced_count, updated_count, new_count, topics))
    }

    /// 同步 FTS 索引
    ///
    /// 将未索引的仓库数据同步到全文搜索索引表。
    ///
    /// # Returns
    /// - `Result<i64, String>`: 同步的行数或错误信息
    async fn sync_fts(&self) -> Result<i64, String> {
        let result = sqlx::query(
            r#"
            INSERT OR IGNORE INTO repos_fts(rowid, owner_login, name, description, language, topics)
            SELECT 
                r.id, 
                r.owner_login, 
                r.name, 
                COALESCE(r.description, ''),
                COALESCE(
                    (SELECT TRIM(REPLACE(REPLACE(json_group_array(DISTINCT j.key), '[', ''), ']', '') || ',') 
                     FROM json_each(r.language) AS j), 
                    ''
                ),
                COALESCE(r.topics, '')
            FROM repositories r
            WHERE r.id NOT IN (SELECT rowid FROM repos_fts) AND r.deleted_at IS NULL
            "#,
        )
        .execute(self.pool)
        .await
        .map_err(|e| format!("Failed to sync FTS: {:?}", e))?;

        Ok(result.rows_affected() as i64)
    }

    /// 获取最近一周更新的仓库
    ///
    /// 返回最近7天内有更新（pushed_at）的仓库列表。
    ///
    /// # Returns
    /// - `Result<Vec<Repository>, String>`: 仓库列表或错误信息
    async fn get_recent_updated_repos(
        &self,
    ) -> Result<Vec<crate::models::entity::Repository>, String> {
        let seven_days_ago = Utc::now() - chrono::Duration::days(7);

        let entities: Vec<RepositoryEntity> = sqlx::query_as(
            r#"
            SELECT
                id, github_id, full_name, name, description,
                stargazers_count, forks_count, language, topics,
                pushed_at, created_at, html_url, clone_url, homepage,
                open_issues_count, open_pr, total_pr, license, starred_at,
                owner_login, owner_avatar_url, learning_status,
                is_favorite, deleted_at, archived, status
            FROM repositories
            WHERE deleted_at IS NULL AND pushed_at >= ?
            ORDER BY pushed_at DESC
            "#,
        )
        .bind(seven_days_ago.to_rfc3339_opts(chrono::SecondsFormat::Secs, true))
        .fetch_all(self.pool)
        .await
        .map_err(|e| format!("Failed to get recent updated repos: {:?}", e))?;

        // 将实体转换为 Repository 类型，并获取关联的分类
        let mut repos = Vec::with_capacity(entities.len());
        for entity in entities {
            let mut repo = Repository::from(entity);
            repo.categories = get_categories_by_repo_id(self.pool, repo.github_id).await?;
            repos.push(repo);
        }

        Ok(repos)
    }

    /// 获取状态分类仓库（归档、不活跃、废弃）
    ///
    /// 返回状态为 archived、inactive 或 deprecated 的仓库列表。
    ///
    /// # Returns
    /// - `Result<Vec<Repository>, String>`: 仓库列表或错误信息
    async fn get_repo_status_categories(
        &self,
    ) -> Result<Vec<crate::models::entity::Repository>, String> {
        let entities: Vec<RepositoryEntity> = sqlx::query_as(
            r#"
            SELECT
                id, github_id, full_name, name, description,
                stargazers_count, forks_count, language, topics,
                pushed_at, created_at, html_url, clone_url, homepage,
                open_issues_count, open_pr, total_pr, license, starred_at,
                owner_login, owner_avatar_url, learning_status,
                is_favorite, archived, deleted_at, status
            FROM repositories
            WHERE deleted_at IS NULL AND status IN ('archived', 'inactive', 'deprecated')
            ORDER BY
                CASE status
                    WHEN 'archived' THEN 1
                    WHEN 'inactive' THEN 2
                    WHEN 'deprecated' THEN 3
                    ELSE 4
                END ASC,
                pushed_at DESC
            "#,
        )
        .fetch_all(self.pool)
        .await
        .map_err(|e| format!("Failed to get repos by status: {:?}", e))?;

        // 将实体转换为 Repository 类型，并获取关联的分类
        let mut repos = Vec::with_capacity(entities.len());
        for entity in entities {
            let mut repo = Repository::from(entity);
            repo.categories = get_categories_by_repo_id(self.pool, repo.github_id).await?;
            repos.push(repo);
        }

        Ok(repos)
    }

    /// 获取仓库统计信息
    ///
    /// 返回仪表盘所需的统计数据：总仓库数、分类数、未分类仓库数、最近标星数。
    ///
    /// # Returns
    /// - `Result<(i64, i64, i64, i64), String>`: (总仓库数, 分类数, 未分类仓库数, 最近标星数) 或错误信息
    async fn get_repo_stats(&self) -> Result<(i64, i64, i64, i64), String> {
        // 1. 获取总仓库数（未删除的）
        let total_repos: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM repositories WHERE deleted_at IS NULL")
                .fetch_one(self.pool)
                .await
                .map_err(|e| format!("Failed to get total repos count: {:?}", e))?;

        // 2. 获取分类标签数量
        let category_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM categories")
            .fetch_one(self.pool)
            .await
            .map_err(|e| format!("Failed to get category count: {:?}", e))?;

        // 3. 获取有分类关联的仓库数量
        let categorized_count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(DISTINCT repo_id) FROM repo_category_relation
            "#,
        )
        .fetch_one(self.pool)
        .await
        .map_err(|e| format!("Failed to get categorized repos count: {:?}", e))?;

        // 未分类仓库数量 = 总仓库数 - 有分类关联的仓库数
        let uncategorized_count = total_repos.0 - categorized_count.0;

        // 4. 获取7天内标星的仓库数量
        let seven_days_ago = Utc::now() - chrono::Duration::days(7);
        let recent_starred_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM repositories WHERE deleted_at IS NULL AND starred_at >= ?",
        )
        .bind(seven_days_ago.to_rfc3339_opts(chrono::SecondsFormat::Secs, true))
        .fetch_one(self.pool)
        .await
        .map_err(|e| format!("Failed to get recent starred repos count: {:?}", e))?;

        Ok((
            total_repos.0,
            category_count.0,
            uncategorized_count,
            recent_starred_count.0,
        ))
    }
}

/// 根据仓库 GitHub ID 获取关联的分类列表
///
/// # Arguments
/// - `pool`: SQLite 数据库连接池引用
/// - `github_id`: 仓库的 GitHub ID
///
/// # Returns
/// - `Result<Vec<RepoCategory>, String>`: 分类列表或错误信息
async fn get_categories_by_repo_id(
    pool: &Pool<Sqlite>,
    github_id: i64,
) -> Result<Vec<RepoCategory>, String> {
    let rows = sqlx::query("SELECT c.id, c.name, c.color FROM categories c JOIN repo_category_relation r ON c.id = r.category_id WHERE r.repo_id = ? ORDER BY c.name")
        .bind(github_id)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to get categories: {:?}", e))?;

    // 将查询结果映射为 RepoCategory 向量
    let categories: Vec<RepoCategory> = rows
        .into_iter()
        .map(|row| RepoCategory {
            id: row.get(0),
            name: row.get(1),
            color: row.get(2),
        })
        .collect();

    Ok(categories)
}

/// 根据排序字段查询仓库列表
///
/// # Arguments
/// - `pool`: SQLite 数据库连接池引用
/// - `sort_field`: 排序字段
/// - `offset`: 分页偏移量
/// - `page_size`: 每页数量
/// - `language`: 语言过滤条件（可选）
/// - `categories`: 分类过滤条件（可选，支持多选）
/// - `deleted`: 是否查询已删除的仓库
/// - `sort_order`: 排序方向（asc/desc）
///
/// # Returns
/// - `Result<(Vec<RepositoryEntity>, i64), String>`: (仓库实体列表, 总数) 或错误信息
async fn query_repos_by_sort(
    pool: &Pool<Sqlite>,
    sort_field: &str,
    offset: i64,
    page_size: i64,
    language: Option<&str>,
    categories: Option<&[String]>,
    deleted: bool,
    sort_order: &str,
) -> Result<(Vec<RepositoryEntity>, i64), String> {
    // 构建删除状态条件
    let deleted_condition = if deleted {
        "deleted_at IS NOT NULL"
    } else {
        "deleted_at IS NULL"
    };

    // 验证排序方向，默认使用 DESC
    let order_by = if sort_order.eq_ignore_ascii_case("asc") {
        "ASC"
    } else {
        "DESC"
    };

    // 构建 WHERE 子句
    let mut where_clause = vec![deleted_condition.to_string()];
    let mut params: Vec<String> = Vec::new();

    // 添加语言过滤条件
    if let Some(lang) = language {
        where_clause.push(
            "EXISTS (SELECT 1 FROM json_each(repositories.language) AS j WHERE j.key = ?)"
                .to_string(),
        );
        params.push(lang.to_string());
    }

    // 添加分类过滤条件（支持多选，AND 关系）
    if let Some(cats) = categories {
        for cat in cats {
            where_clause.push("github_id IN (SELECT repo_id FROM repo_category_relation WHERE category_id = (SELECT id FROM categories WHERE name = ?))".to_string());
            params.push(cat.to_string());
        }
    }

    let where_str = where_clause.join(" AND ");

    // 构建查询
    let repos_query_str = format!(
        "SELECT
            id, github_id, full_name, name, description,
            stargazers_count, forks_count, language, topics,
            pushed_at, created_at, html_url, clone_url, homepage,
            open_issues_count, open_pr, total_pr, license, starred_at,
            owner_login, owner_avatar_url, learning_status,
            is_favorite, deleted_at, archived, status
        FROM repositories
        WHERE {}
        ORDER BY {} {}
        LIMIT ? OFFSET ?",
        where_str, sort_field, order_by
    );
    let repos = sqlx::query_as::<_, RepositoryEntity>(&repos_query_str);

    // 绑定参数
    let mut repos_query = repos;
    for param in &params {
        repos_query = repos_query.bind(param);
    }
    repos_query = repos_query.bind(page_size).bind(offset);

    let repos_result = repos_query
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to get repos: {:?}", e))?;

    // 查询总数
    let total_query_str = format!("SELECT COUNT(*) FROM repositories WHERE {}", where_str);
    let total_query = sqlx::query_as::<_, (i64,)>(&total_query_str);

    let mut total_sqlx_query = total_query;
    for param in &params {
        total_sqlx_query = total_sqlx_query.bind(param);
    }

    let total: (i64,) = total_sqlx_query
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to count repos: {:?}", e))?;

    Ok((repos_result, total.0))
}

/// 更新仓库信息
///
/// # Arguments
/// - `pool`: SQLite 数据库连接池引用
/// - `repo`: GitHub 仓库响应数据
/// - `topics_json`: 标签 JSON 字符串（可选）
/// - `license_name`: 许可证名称（可选）
/// - `cancel_flag`: 取消标志
///
/// # Returns
/// - `Result<(), String>`: 成功返回空，失败返回错误信息
async fn update_repo(
    pool: &Pool<Sqlite>,
    repo: &GitHubRepoResponse,
    topics_json: &Option<String>,
    license_name: &Option<String>,
) -> Result<(), String> {
    // 计算仓库状态
    let status = calculate_repo_status(repo.archived, &repo.pushed_at);

    // 执行带重试的数据库更新操作
    execute_db_with_retry(|| async {
        sqlx::query(
            r#"
                UPDATE repositories SET
                    full_name = ?,
                    name = ?,
                    description = ?,
                    stargazers_count = ?,
                    forks_count = ?,
                    topics = ?,
                    language = ?,
                    pushed_at = ?,
                    html_url = ?,
                    clone_url = ?,
                    homepage = ?,
                    open_issues_count = ?,
                    license = ?,
                    starred_at = ?,
                    owner_login = ?,
                    owner_avatar_url = ?,
                    archived = ?,
                    status = ?
                WHERE github_id = ?
                "#,
        )
        .bind(&repo.full_name)
        .bind(&repo.name)
        .bind(&repo.description)
        .bind(repo.stargazers_count)
        .bind(repo.forks_count)
        .bind(topics_json)
        .bind(&repo.language)
        .bind(&repo.pushed_at)
        .bind(&repo.html_url)
        .bind(&repo.clone_url)
        .bind(&repo.homepage)
        .bind(repo.open_issues)
        .bind(license_name)
        .bind(&repo.starred_at)
        .bind(&repo.owner.login)
        .bind(&repo.owner.avatar_url)
        .bind(repo.archived)
        .bind(&status)
        .bind(repo.id)
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(|e| format!("{}", e))
    })
    .await
}

/// 插入新仓库
///
/// # Arguments
/// - `pool`: SQLite 数据库连接池引用
/// - `repo`: GitHub 仓库响应数据
/// - `topics_json`: 标签 JSON 字符串（可选）
/// - `license_name`: 许可证名称（可选）
/// - `cancel_flag`: 取消标志
///
/// # Returns
/// - `Result<(), String>`: 成功返回空，失败返回错误信息
async fn insert_repo(
    pool: &Pool<Sqlite>,
    repo: &GitHubRepoResponse,
    topics_json: &Option<String>,
    license_name: &Option<String>,
) -> Result<(), String> {
    // 计算仓库状态
    let status = calculate_repo_status(repo.archived, &repo.pushed_at);

    // 执行带重试的数据库插入操作
    execute_db_with_retry(
        || async {
            sqlx::query(
                r#"
                INSERT INTO repositories (
                    github_id, full_name, name, description, stargazers_count, forks_count,
                    topics, language, pushed_at, created_at, html_url, clone_url, homepage,
                    open_issues_count, license, starred_at, owner_login, owner_avatar_url, archived, status
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(repo.id)
            .bind(&repo.full_name)
            .bind(&repo.name)
            .bind(&repo.description)
            .bind(repo.stargazers_count)
            .bind(repo.forks_count)
            .bind(topics_json)
            .bind(&repo.language)
            .bind(&repo.pushed_at)
            .bind(&repo.created_at)
            .bind(&repo.html_url)
            .bind(&repo.clone_url)
            .bind(&repo.homepage)
            .bind(repo.open_issues)
            .bind(license_name)
            .bind(&repo.starred_at)
            .bind(&repo.owner.login)
            .bind(&repo.owner.avatar_url)
            .bind(repo.archived)
            .bind(&status)
            .execute(pool)
            .await
            .map(|_| ())
            .map_err(|e| format!("{}", e))
        },
    )
    .await
}

/// 带重试机制的数据库操作执行器
///
/// 当数据库操作失败时（如数据库繁忙、IO错误、网络问题），会自动重试最多3次。
/// 重试间隔会指数增长（500ms, 1000ms, 2000ms）。
///
/// # Type Parameters
/// - `F`: 操作函数类型
/// - `Fut`: 返回的 Future 类型
/// - `T`: 返回值类型
///
/// # Arguments
/// - `f`: 要执行的数据库操作函数
/// - `cancel_flag`: 取消标志（用于中断重试）
///
/// # Returns
/// - `Result<T, String>`: 操作结果或错误信息
async fn execute_db_with_retry<'a, F, Fut, T>(mut f: F) -> Result<T, String>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, String>> + 'a,
{
    const MAX_RETRIES: usize = 3; // 最大重试次数
    let mut delay = Duration::from_millis(500); // 初始重试延迟

    for attempt in 0..=MAX_RETRIES {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                // 如果是最后一次尝试，直接返回错误
                if attempt == MAX_RETRIES {
                    return Err(e);
                }

                // 如果是数据库相关错误，进行重试
                if e.contains("database") || e.contains("io") || e.contains("tls") {
                    sleep(delay).await;
                    delay = delay * 2; // 指数退避
                } else {
                    // 非数据库错误，直接返回
                    return Err(e);
                }
            }
        }
    }

    unreachable!("max_retries loop should have returned");
}

/// 获取仓库仓库实例
///
/// 获取数据库连接池并创建仓库仓库实例，返回装箱后的 trait 对象。
///
/// # Returns
/// - `Box<dyn RepoRepo + Send + Sync>`: 仓库仓库实例
pub async fn get_repo_repo() -> Box<dyn RepoRepo + Send + Sync> {
    let pool = crate::db::sqlite::get_pool()
        .map_err(|e| format!("Failed to get database pool: {}", e))
        .unwrap();
    Box::new(RepoRepoImpl::new(pool))
}

/// 同步仓库到数据库（公开接口）
///
/// 将从 GitHub API 获取的仓库数据同步到本地数据库。
///
/// # Arguments
/// - `repos`: GitHub 仓库响应列表
/// - `cancel_flag`: 取消标志（用于中断同步）
///
/// # Returns
/// - `Result<(usize, usize, usize, Vec<String>), String>`: (同步总数, 更新数, 新增数, 所有标签) 或错误信息
pub async fn sync_repos_to_db(
    repos: Vec<GitHubRepoResponse>,
) -> Result<(usize, usize, usize, Vec<String>), String> {
    let repo_repo = get_repo_repo().await;
    repo_repo.sync_repos_to_db(repos).await
}

/// 软删除仓库（标记为已取消标星）
///
/// 将仓库标记为已删除（设置 deleted_at 为当前时间），而不是真正删除记录。
///
/// # Arguments
/// - `owner`: 仓库所有者
/// - `repo_name`: 仓库名称
///
/// # Returns
/// - `Result<(), String>`: 成功返回空，失败返回错误信息
pub async fn soft_delete_repo(owner: &str, repo_name: &str) -> Result<(), String> {
    let pool = crate::db::sqlite::get_pool()
        .map_err(|e| format!("Failed to get database pool: {}", e))
        .unwrap();

    // 构建完整仓库名（owner/repo_name）
    let full_name = format!("{}/{}", owner, repo_name);

    // 执行软删除
    let result = sqlx::query(
        r#"
        UPDATE repositories
        SET deleted_at = datetime('now', 'localtime')
        WHERE full_name = ? AND deleted_at IS NULL
        "#,
    )
    .bind(full_name)
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to soft delete repo: {:?}", e))?;

    // 检查是否有记录被更新
    if result.rows_affected() == 0 {
        return Err("Repo not found or already deleted".to_string());
    }

    // 清空缓存
    let cache = get_cache();
    cache.invalidate().await;

    Ok(())
}

/// 恢复已删除的仓库
///
/// 将已软删除的仓库恢复（清除 deleted_at 字段）。
///
/// # Arguments
/// - `owner`: 仓库所有者
/// - `repo_name`: 仓库名称
///
/// # Returns
/// - `Result<(), String>`: 成功返回空，失败返回错误信息
pub async fn restore_repo(owner: &str, repo_name: &str) -> Result<(), String> {
    let pool = crate::db::sqlite::get_pool()
        .map_err(|e| format!("Failed to get database pool: {}", e))
        .unwrap();

    // 构建完整仓库名（owner/repo_name）
    let full_name = format!("{}/{}", owner, repo_name);

    // 执行恢复操作
    let result = sqlx::query(
        r#"
        UPDATE repositories
        SET deleted_at = NULL
        WHERE full_name = ? AND deleted_at IS NOT NULL
        "#,
    )
    .bind(full_name)
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to restore repo: {:?}", e))?;

    // 检查是否有记录被更新
    if result.rows_affected() == 0 {
        return Err("Repo not found or not deleted".to_string());
    }

    // 清空缓存
    let cache = get_cache();
    cache.invalidate().await;

    Ok(())
}
