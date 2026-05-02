//! 合集仓储模块
//!
//! 该模块负责合集相关的数据访问操作，封装了与 SQLite 数据库的交互逻辑。
//! 提供合集的增删改查以及仓库合集关联管理功能。

use crate::models::dto::{CollectionPageRequest, CollectionPageResponse, CollectionWithRepoCount};
use crate::models::entity::{Collection, Repository, RepositoryEntity};
use crate::repos::traits::CollectionRepo;
use async_trait::async_trait;
use sqlx::Pool;
use sqlx::Row;
use sqlx::Sqlite;

/// 合集仓库实现结构体
///
/// 封装合集相关的数据访问操作，持有数据库连接池引用。
pub struct CollectionRepoImpl {
    /// SQLite 数据库连接池引用
    pool: &'static Pool<Sqlite>,
}

#[async_trait]
impl CollectionRepo for CollectionRepoImpl {
    /// 创建合集
    async fn create_collection(&self, name: &str, description: &str, color: &str) -> Result<Collection, String> {
        let collection = sqlx::query_as::<_, Collection>(
            r#"
        INSERT INTO collections (name, description, color, created_at, updated_at)
        VALUES (?, ?, ?, datetime('now', 'localtime'), datetime('now', 'localtime'))
        RETURNING *
        "#,
        )
        .bind(name)
        .bind(description)
        .bind(color)
        .fetch_one(self.pool)
        .await
        .map_err(|e| format!("Failed to create collection: {:?}", e))?;

        Ok(collection)
    }

    /// 更新合集
    async fn update_collection(
        &self,
        collection_id: i64,
        name: Option<&str>,
        description: Option<&str>,
        color: Option<&str>,
    ) -> Result<CollectionWithRepoCount, String> {
        let mut update_fields = Vec::new();
        let mut params: Vec<String> = Vec::new();

        if let Some(n) = name {
            update_fields.push("name = ?");
            params.push(n.to_string());
        }
        if let Some(d) = description {
            update_fields.push("description = ?");
            params.push(d.to_string());
        }
        if let Some(c) = color {
            update_fields.push("color = ?");
            params.push(c.to_string());
        }

        if update_fields.is_empty() {
            return Err("No update fields provided".to_string());
        }

        update_fields.push("updated_at = datetime('now', 'localtime')");

        let query = format!(
            r#"
            UPDATE collections
            SET {}
            WHERE id = ?
            "#,
            update_fields.join(", ")
        );

        let mut sqlx_query = sqlx::query(&query);
        for param in &params {
            sqlx_query = sqlx_query.bind(param);
        }
        sqlx_query = sqlx_query.bind(collection_id);

        sqlx_query
            .execute(self.pool)
            .await
            .map_err(|e| format!("Failed to update collection: {:?}", e))?;

        self.get_collection_by_id(collection_id).await
    }

    /// 删除合集（不会删除关联的仓库，只删除关联关系）
    async fn delete_collection(&self, collection_id: i64) -> Result<(), String> {
        sqlx::query("DELETE FROM repo_collection_relation WHERE collection_id = ?")
            .bind(collection_id)
            .execute(self.pool)
            .await
            .map_err(|e| format!("Failed to delete relations: {:?}", e))?;

        sqlx::query("DELETE FROM collections WHERE id = ?")
            .bind(collection_id)
            .execute(self.pool)
            .await
            .map_err(|e| format!("Failed to delete collection: {:?}", e))?;

        Ok(())
    }

    /// 根据ID获取合集
    async fn get_collection_by_id(
        &self,
        collection_id: i64,
    ) -> Result<CollectionWithRepoCount, String> {
        let row = sqlx::query(
            r#"
            SELECT
                c.id,
                c.name,
                c.description,
                c.color,
                COUNT(r.github_id) as repo_count,
                c.created_at,
                c.updated_at
            FROM collections c
            LEFT JOIN repo_collection_relation r ON c.id = r.collection_id
            WHERE c.id = ?
            GROUP BY c.id, c.name, c.description, c.color, c.created_at, c.updated_at
            "#,
        )
        .bind(collection_id)
        .fetch_one(self.pool)
        .await
        .map_err(|e| format!("Failed to get collection: {:?}", e))?;

        Ok(CollectionWithRepoCount {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
            color: row.get(3),
            repo_count: row.get(4),
            created_at: row.get(5),
            updated_at: row.get(6),
        })
    }

    /// 获取所有合集（带仓库数量）
    async fn get_all_collections(&self) -> Result<Vec<CollectionWithRepoCount>, String> {
        let rows = sqlx::query(
            r#"
            SELECT
                c.id,
                c.name,
                c.description,
                c.color,
                COUNT(r.github_id) as repo_count,
                c.created_at,
                c.updated_at
            FROM collections c
            LEFT JOIN repo_collection_relation r ON c.id = r.collection_id
            GROUP BY c.id, c.name, c.description, c.color, c.created_at, c.updated_at
            ORDER BY c.created_at DESC
            "#,
        )
        .fetch_all(self.pool)
        .await
        .map_err(|e| format!("Failed to get collections: {:?}", e))?;

        let collections: Vec<CollectionWithRepoCount> = rows
            .into_iter()
            .map(|row| CollectionWithRepoCount {
                id: row.get(0),
                name: row.get(1),
                description: row.get(2),
                color: row.get(3),
                repo_count: row.get(4),
                created_at: row.get(5),
                updated_at: row.get(6),
            })
            .collect();

        Ok(collections)
    }

    /// 分页获取合集列表（支持关键字搜索）
    async fn get_collections_paged(
        &self,
        request: CollectionPageRequest,
    ) -> Result<CollectionPageResponse, String> {
        let page = request.page;
        let page_size = request.page_size;
        let offset = (page - 1) * page_size;

        let order_column = match request.sort_by.as_str() {
            "repo_count" => "repo_count",
            "name" => "c.name",
            "updated_at" => "c.updated_at",
            "created_at" => "c.created_at",
            _ => "c.created_at",
        };

        let order_direction = match request.sort_order.as_str() {
            "asc" => "ASC",
            _ => "DESC",
        };

        if let Some(keyword) = request.search_keyword {
            if !keyword.is_empty() {
                let count_query = sqlx::query(
                    r#"
                    SELECT COUNT(*)
                    FROM collections c
                    LEFT JOIN repo_collection_relation r ON c.id = r.collection_id
                    WHERE c.name LIKE ? OR c.description LIKE ?
                    GROUP BY c.id
                    "#,
                )
                .bind(format!("%{}%", keyword))
                .bind(format!("%{}%", keyword))
                .fetch_all(self.pool)
                .await
                .map_err(|e| format!("Failed to count collections: {:?}", e))?;

                let total = count_query.len() as i64;

                let rows = sqlx::query(&format!(
                    r#"
                        SELECT
                            c.id,
                            c.name,
                            c.description,
                            c.color,
                            COUNT(r.github_id) as repo_count,
                            c.created_at,
                            c.updated_at
                        FROM collections c
                        LEFT JOIN repo_collection_relation r ON c.id = r.collection_id
                        WHERE c.name LIKE ? OR c.description LIKE ?
                        GROUP BY c.id, c.name, c.description, c.color, c.created_at, c.updated_at
                        ORDER BY {} {}
                        LIMIT ? OFFSET ?
                        "#,
                    order_column, order_direction
                ))
                .bind(format!("%{}%", keyword))
                .bind(format!("%{}%", keyword))
                .bind(page_size)
                .bind(offset)
                .fetch_all(self.pool)
                .await
                .map_err(|e| format!("Failed to get collections: {:?}", e))?;

                let collections: Vec<CollectionWithRepoCount> = rows
                    .into_iter()
                    .map(|row| CollectionWithRepoCount {
                        id: row.get(0),
                        name: row.get(1),
                        description: row.get(2),
                        color: row.get(3),
                        repo_count: row.get(4),
                        created_at: row.get(5),
                        updated_at: row.get(6),
                    })
                    .collect();

                let total_pages = (total + page_size - 1) / page_size;

                return Ok(CollectionPageResponse {
                    collections,
                    total,
                    page,
                    page_size,
                    total_pages,
                });
            }
        }

        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM collections")
            .fetch_one(self.pool)
            .await
            .map_err(|e| format!("Failed to count collections: {:?}", e))?;

        let rows = sqlx::query(&format!(
            r#"
                SELECT
                    c.id,
                    c.name,
                    c.description,
                    c.color,
                    COUNT(r.github_id) as repo_count,
                    c.created_at,
                    c.updated_at
                FROM collections c
                LEFT JOIN repo_collection_relation r ON c.id = r.collection_id
                GROUP BY c.id, c.name, c.description, c.color, c.created_at, c.updated_at
                ORDER BY {} {}
                LIMIT ? OFFSET ?
                "#,
            order_column, order_direction
        ))
        .bind(page_size)
        .bind(offset)
        .fetch_all(self.pool)
        .await
        .map_err(|e| format!("Failed to get collections: {:?}", e))?;

        let collections: Vec<CollectionWithRepoCount> = rows
            .into_iter()
            .map(|row| CollectionWithRepoCount {
                id: row.get(0),
                name: row.get(1),
                description: row.get(2),
                color: row.get(3),
                repo_count: row.get(4),
                created_at: row.get(5),
                updated_at: row.get(6),
            })
            .collect();

        let total_pages = (count + page_size - 1) / page_size;

        Ok(CollectionPageResponse {
            collections,
            total: count,
            page,
            page_size,
            total_pages,
        })
    }

    /// 获取合集关联的所有仓库
    async fn get_repos_by_collection(&self, collection_id: i64) -> Result<Vec<Repository>, String> {
        let repo_ids = sqlx::query_scalar::<_, i64>(
            "SELECT github_id FROM repo_collection_relation WHERE collection_id = ?",
        )
        .bind(collection_id)
        .fetch_all(self.pool)
        .await
        .map_err(|e| format!("Failed to get repo ids: {:?}", e))?;

        if repo_ids.is_empty() {
            return Ok(vec![]);
        }

        let placeholders = repo_ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");

        let query = format!(
            r#"
            SELECT 
                id, github_id, full_name, name, description,
                stargazers_count, forks_count, language, topics,
                pushed_at, created_at, html_url, clone_url, homepage,
                open_issues_count, open_pr, total_pr, license, starred_at,
                owner_login, owner_avatar_url, learning_status,
                is_favorite, deleted_at, archived, status
            FROM repositories
            WHERE github_id IN ({}) AND deleted_at IS NULL
            ORDER BY starred_at DESC
            "#,
            placeholders
        );

        let mut sqlx_query = sqlx::query_as::<_, RepositoryEntity>(&query);
        for &github_id in &repo_ids {
            sqlx_query = sqlx_query.bind(github_id);
        }

        let entities = sqlx_query
            .fetch_all(self.pool)
            .await
            .map_err(|e| format!("Failed to get repos: {:?}", e))?;

        let repos: Vec<Repository> = entities.into_iter().map(Repository::from).collect();

        Ok(repos)
    }

    /// 获取仓库关联的所有合集
    async fn get_collections_by_repo(&self, github_id: i64) -> Result<Vec<Collection>, String> {
        let collections = sqlx::query_as::<_, Collection>(
            r#"
            SELECT c.* FROM collections c
            JOIN repo_collection_relation r ON c.id = r.collection_id
            WHERE r.github_id = ?
            ORDER BY c.name
            "#,
        )
        .bind(github_id)
        .fetch_all(self.pool)
        .await
        .map_err(|e| format!("Failed to get collections: {:?}", e))?;

        Ok(collections)
    }

    /// 更新合集与仓库的关联关系
    async fn update_collection_repos(
        &self,
        collection_id: i64,
        github_ids: &[i64],
    ) -> Result<(), String> {
        let rows =
            sqlx::query("SELECT github_id FROM repo_collection_relation WHERE collection_id = ?")
                .bind(collection_id)
                .fetch_all(self.pool)
                .await
                .map_err(|e| format!("Failed to get current relations: {:?}", e))?;

        let current_github_ids: std::collections::HashSet<_> =
            rows.into_iter().map(|row| row.get(0)).collect();
        let new_github_ids: std::collections::HashSet<_> = github_ids.iter().cloned().collect();

        for &github_id in &current_github_ids {
            if !new_github_ids.contains(&github_id) {
                sqlx::query(
                    "DELETE FROM repo_collection_relation WHERE collection_id = ? AND github_id = ?",
                )
                .bind(collection_id)
                .bind(github_id)
                .execute(self.pool)
                .await
                .map_err(|e| format!("Failed to delete relation: {:?}", e))?;
            }
        }

        for &github_id in &new_github_ids {
            if !current_github_ids.contains(&github_id) {
                sqlx::query(
                    r#"
                INSERT OR IGNORE INTO repo_collection_relation (github_id, collection_id)
                VALUES (?, ?)
                "#,
                )
                .bind(github_id)
                .bind(collection_id)
                .execute(self.pool)
                .await
                .map_err(|e| format!("Failed to insert relation: {:?}", e))?;
            }
        }

        Ok(())
    }

    /// 检查仓库是否已在合集中
    async fn is_repo_in_collection(
        &self,
        github_id: i64,
        collection_id: i64,
    ) -> Result<bool, String> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM repo_collection_relation WHERE github_id = ? AND collection_id = ?",
        )
        .bind(github_id)
        .bind(collection_id)
        .fetch_one(self.pool)
        .await
        .map_err(|e| format!("Failed to check relation: {:?}", e))?;

        Ok(count > 0)
    }
}

impl CollectionRepoImpl {
    /// 创建合集仓库实例
    pub fn new(pool: &'static Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

/// 获取合集仓库实例
pub async fn get_collection_repo() -> Box<dyn CollectionRepo + Send + Sync> {
    let pool = crate::db::sqlite::get_pool()
        .map_err(|e| format!("Failed to get database pool: {}", e))
        .unwrap();
    Box::new(CollectionRepoImpl::new(pool))
}
