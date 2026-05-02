//! 分类仓储模块
//!
//! 该模块负责分类相关的数据访问操作，封装了与 SQLite 数据库的交互逻辑。
//! 提供分类的增删改查以及仓库分类关联管理功能。

use crate::models::dto::{CategoryPageRequest, CategoryPageResponse, CategoryWithCount};
use crate::models::entity::Category;
use crate::repos::traits::CategoryRepo;
use async_trait::async_trait;
use sqlx::Pool;
use sqlx::Row;
use sqlx::Sqlite;

/// 分类仓库实现结构体
///
/// 封装分类相关的数据访问操作，持有数据库连接池引用。
pub struct CategoryRepoImpl {
    /// SQLite 数据库连接池引用
    pool: &'static Pool<Sqlite>,
}

#[async_trait]
impl CategoryRepo for CategoryRepoImpl {
    /// 获取所有分类及其关联的仓库数量
    ///
    /// 查询数据库中所有分类，并通过 LEFT JOIN 统计每个分类关联的仓库数量。
    /// 结果按仓库数量降序排列，仓库数量相同时按创建时间降序排列。
    ///
    /// # Returns
    /// - `Result<Vec<CategoryWithCount>, String>`: 分类列表（含仓库数量）或错误信息
    async fn get_all_categories_with_count(&self) -> Result<Vec<CategoryWithCount>, String> {
        // 执行 SQL 查询，获取分类及其关联仓库数量
        let rows = sqlx::query(
            r#"
            SELECT 
                c.id, 
                c.name, 
                c.color, 
                COUNT(r.category_id) as repo_count,
                c.updated_at
            FROM categories c
            LEFT JOIN repo_category_relation r ON c.id = r.category_id
            GROUP BY c.id, c.name, c.color, c.updated_at
            ORDER BY 
                repo_count DESC,
                c.created_at DESC
            "#,
        )
        .fetch_all(self.pool)
        .await
        .map_err(|e| format!("Failed to get categories: {:?}", e))?;

        // 将查询结果映射为 CategoryWithCount 向量
        let categories: Vec<CategoryWithCount> = rows
            .into_iter()
            .map(|row| CategoryWithCount {
                id: row.get(0),
                name: row.get(1),
                color: row.get(2),
                repo_count: row.get(3),
                updated_at: row.get(4),
            })
            .collect();

        Ok(categories)
    }

    /// 分页获取分类列表
    ///
    /// 根据分页请求参数查询分类，支持搜索关键字过滤和排序。
    ///
    /// # Arguments
    /// - `request`: 分页请求参数
    ///
    /// # Returns
    /// - `Result<CategoryPageResponse, String>`: 分页响应或错误信息
    async fn get_categories_paged(
        &self,
        request: CategoryPageRequest,
    ) -> Result<CategoryPageResponse, String> {
        let page = request.page;
        let page_size = request.page_size;
        let offset = (page - 1) * page_size;

        // 构建排序字段
        let order_column = match request.sort_by.as_str() {
            "count" => "repo_count",
            "name" => "c.name",
            "updated_at" => "c.updated_at",
            _ => "repo_count",
        };

        // 构建排序方向
        let order_direction = match request.sort_order.as_str() {
            "asc" => "ASC",
            _ => "DESC",
        };

        if let Some(keyword) = request.search_keyword {
            if !keyword.is_empty() {
                // 带搜索关键字的查询
                let count_query = sqlx::query(
                    r#"
                    SELECT COUNT(*) 
                    FROM categories c
                    LEFT JOIN repo_category_relation r ON c.id = r.category_id
                    WHERE c.name LIKE ?
                    GROUP BY c.id
                    "#,
                )
                .bind(format!("%{}%", keyword))
                .fetch_all(self.pool)
                .await
                .map_err(|e| format!("Failed to count categories: {:?}", e))?;

                let total = count_query.len() as i64;

                let rows = sqlx::query(&format!(
                    r#"
                        SELECT 
                            c.id, 
                            c.name, 
                            c.color, 
                            COUNT(r.category_id) as repo_count,
                            c.updated_at
                        FROM categories c
                        LEFT JOIN repo_category_relation r ON c.id = r.category_id
                        WHERE c.name LIKE ?
                        GROUP BY c.id, c.name, c.color, c.updated_at
                        ORDER BY {} {}
                        LIMIT ? OFFSET ?
                        "#,
                    order_column, order_direction
                ))
                .bind(format!("%{}%", keyword))
                .bind(page_size)
                .bind(offset)
                .fetch_all(self.pool)
                .await
                .map_err(|e| format!("Failed to get categories: {:?}", e))?;

                let categories: Vec<CategoryWithCount> = rows
                    .into_iter()
                    .map(|row| CategoryWithCount {
                        id: row.get(0),
                        name: row.get(1),
                        color: row.get(2),
                        repo_count: row.get(3),
                        updated_at: row.get(4),
                    })
                    .collect();

                let total_pages = (total + page_size - 1) / page_size;

                return Ok(CategoryPageResponse {
                    categories,
                    total,
                    page,
                    page_size,
                    total_pages,
                });
            }
        }

        // 不带搜索关键字的查询
        let count = sqlx::query_scalar("SELECT COUNT(*) FROM categories")
            .fetch_one(self.pool)
            .await
            .map_err(|e| format!("Failed to count categories: {:?}", e))?;

        let rows = sqlx::query(&format!(
            r#"
                SELECT 
                    c.id, 
                    c.name, 
                    c.color, 
                    COUNT(r.category_id) as repo_count,
                    c.updated_at
                FROM categories c
                LEFT JOIN repo_category_relation r ON c.id = r.category_id
                GROUP BY c.id, c.name, c.color, c.updated_at
                ORDER BY {} {}
                LIMIT ? OFFSET ?
                "#,
            order_column, order_direction
        ))
        .bind(page_size)
        .bind(offset)
        .fetch_all(self.pool)
        .await
        .map_err(|e| format!("Failed to get categories: {:?}", e))?;

        let categories: Vec<CategoryWithCount> = rows
            .into_iter()
            .map(|row| CategoryWithCount {
                id: row.get(0),
                name: row.get(1),
                color: row.get(2),
                repo_count: row.get(3),
                updated_at: row.get(4),
            })
            .collect();

        let total_pages = (count + page_size - 1) / page_size;

        Ok(CategoryPageResponse {
            categories,
            total: count,
            page,
            page_size,
            total_pages,
        })
    }

    /// 更新分类信息
    ///
    /// 根据分类ID更新分类名称和/或颜色。
    ///
    /// # Arguments
    /// - `category_id`: 分类ID
    /// - `name`: 新的分类名称（可选）
    /// - `color`: 新的分类颜色（可选）
    ///
    /// # Returns
    /// - `Result<CategoryWithCount, String>`: 更新后的分类信息或错误信息
    async fn update_category(
        &self,
        category_id: i64,
        name: Option<&str>,
        color: Option<&str>,
    ) -> Result<CategoryWithCount, String> {
        let mut update_fields = Vec::new();
        let mut params: Vec<(String, sqlx::types::JsonValue)> = Vec::new();

        if let Some(n) = name {
            update_fields.push("name = ?");
            params.push((
                "name".to_string(),
                sqlx::types::JsonValue::String(n.to_string()),
            ));
        }
        if let Some(c) = color {
            update_fields.push("color = ?");
            params.push((
                "color".to_string(),
                sqlx::types::JsonValue::String(c.to_string()),
            ));
        }

        if update_fields.is_empty() {
            return Err("No update fields provided".to_string());
        }

        let query = format!(
            r#"
            UPDATE categories 
            SET {}, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
            update_fields.join(", ")
        );

        let mut sqlx_query = sqlx::query(&query);
        for (_, value) in &params {
            match value {
                sqlx::types::JsonValue::String(s) => {
                    sqlx_query = sqlx_query.bind(s);
                }
                _ => {}
            }
        }
        sqlx_query = sqlx_query.bind(category_id);

        sqlx_query
            .execute(self.pool)
            .await
            .map_err(|e| format!("Failed to update category: {:?}", e))?;

        // 获取更新后的分类信息
        let row = sqlx::query(
            r#"
            SELECT 
                c.id, 
                c.name, 
                c.color, 
                COUNT(r.category_id) as repo_count,
                c.updated_at
            FROM categories c
            LEFT JOIN repo_category_relation r ON c.id = r.category_id
            WHERE c.id = ?
            GROUP BY c.id, c.name, c.color, c.updated_at
            "#,
        )
        .bind(category_id)
        .fetch_one(self.pool)
        .await
        .map_err(|e| format!("Failed to get category: {:?}", e))?;

        Ok(CategoryWithCount {
            id: row.get(0),
            name: row.get(1),
            color: row.get(2),
            repo_count: row.get(3),
            updated_at: row.get(4),
        })
    }

    // /// 根据仓库ID获取关联的分类（已废弃）
    // ///
    // /// 查询指定仓库关联的所有分类。该方法目前被注释掉，暂未启用。
    // async fn get_categories_by_repo_id(&self, repo_id: i64) -> Result<Vec<Category>, String> {
    //     let categories = sqlx::query_as::<_, Category>(
    //         r#"
    //         SELECT c.* FROM categories c
    //         JOIN repo_category_relation r ON c.id = r.category_id
    //         WHERE r.repo_id = ?
    //         ORDER BY c.name
    //         "#,
    //     )
    //     .bind(repo_id)
    //     .fetch_all(self.pool)
    //     .await
    //     .map_err(|e| format!("Failed to get categories: {:?}", e))?;
    //
    //     Ok(categories)
    // }

    /// 为仓库更新分类关联
    ///
    /// 根据仓库ID和分类ID列表，更新仓库与分类的关联关系。
    /// 采用增量更新策略：先删除不再关联的分类，再添加新的分类关联。
    ///
    /// # Arguments
    /// - `repo_id`: 仓库的 GitHub ID
    /// - `category_ids`: 分类ID列表
    ///
    /// # Returns
    /// - `Result<(), String>`: 成功返回空，失败返回错误信息
    async fn update_repo_categories(
        &self,
        repo_id: i64,
        category_ids: &[i64],
    ) -> Result<(), String> {
        // 1. 获取当前关联的分类ID
        let rows = sqlx::query("SELECT category_id FROM repo_category_relation WHERE repo_id = ?")
            .bind(repo_id)
            .fetch_all(self.pool)
            .await
            .map_err(|e| format!("Failed to get current relations: {:?}", e))?;

        // 将当前分类ID转换为 HashSet，便于快速查找
        let current_category_ids: std::collections::HashSet<_> =
            rows.into_iter().map(|row| row.get(0)).collect();
        // 将新分类ID转换为 HashSet
        let new_category_ids: std::collections::HashSet<_> = category_ids.iter().cloned().collect();

        // 2. 删除不再关联的分类
        for &category_id in &current_category_ids {
            if !new_category_ids.contains(&category_id) {
                sqlx::query(
                    "DELETE FROM repo_category_relation WHERE repo_id = ? AND category_id = ?",
                )
                .bind(repo_id)
                .bind(category_id)
                .execute(self.pool)
                .await
                .map_err(|e| format!("Failed to delete relation: {:?}", e))?;
            }
        }

        // 3. 添加新关联的分类（使用 INSERT OR IGNORE 避免重复插入）
        for &category_id in &new_category_ids {
            if !current_category_ids.contains(&category_id) {
                sqlx::query(
                    r#"
                INSERT OR IGNORE INTO repo_category_relation (repo_id, category_id)
                VALUES (?, ?)
                "#,
                )
                .bind(repo_id)
                .bind(category_id)
                .execute(self.pool)
                .await
                .map_err(|e| format!("Failed to insert relation: {:?}", e))?;
            }
        }

        Ok(())
    }

    /// 创建分类
    ///
    /// 根据指定的名称和颜色创建一个新的分类，并返回创建后的分类实体。
    ///
    /// # Arguments
    /// - `name`: 分类名称
    /// - `color`: 分类颜色（十六进制格式）
    ///
    /// # Returns
    /// - `Result<Category, String>`: 创建成功的分类实体或错误信息
    async fn create_category(&self, name: &str, color: &str) -> Result<Category, String> {
        // 执行 INSERT 语句并返回插入的记录
        let category = sqlx::query_as::<_, Category>(
            r#"
        INSERT INTO categories (name, color, created_at, updated_at)
        VALUES (?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING *
        "#,
        )
        .bind(name)
        .bind(color)
        .fetch_one(self.pool)
        .await
        .map_err(|e| format!("Failed to create category: {:?}", e))?;

        Ok(category)
    }

    /// 删除分类
    ///
    /// 根据分类ID删除指定的分类，同时删除该分类与所有仓库的关联关系。
    /// 操作顺序：先删除关联关系，再删除分类本身，避免外键约束冲突。
    ///
    /// # Arguments
    /// - `category_id`: 要删除的分类ID
    ///
    /// # Returns
    /// - `Result<(), String>`: 成功返回空，失败返回错误信息
    async fn delete_category(&self, category_id: i64) -> Result<(), String> {
        // 1. 删除该分类与所有仓库的关联关系
        sqlx::query("DELETE FROM repo_category_relation WHERE category_id = ?")
            .bind(category_id)
            .execute(self.pool)
            .await
            .map_err(|e| format!("Failed to delete relations: {:?}", e))?;

        // 2. 删除分类本身
        sqlx::query("DELETE FROM categories WHERE id = ?")
            .bind(category_id)
            .execute(self.pool)
            .await
            .map_err(|e| format!("Failed to delete category: {:?}", e))?;

        Ok(())
    }
}

impl CategoryRepoImpl {
    /// 创建分类仓库实例
    ///
    /// # Arguments
    /// - `pool`: SQLite 数据库连接池引用
    ///
    /// # Returns
    /// - `Self`: 分类仓库实例
    pub fn new(pool: &'static Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

/// 获取分类仓库实例
///
/// 获取数据库连接池并创建分类仓库实例，返回装箱后的 trait 对象。
///
/// # Returns
/// - `Box<dyn CategoryRepo + Send + Sync>`: 分类仓库实例
pub async fn get_category_repo() -> Box<dyn CategoryRepo + Send + Sync> {
    // 获取数据库连接池
    let pool = crate::db::sqlite::get_pool()
        .map_err(|e| format!("Failed to get database pool: {}", e))
        .unwrap();
    // 创建分类仓库实例并装箱
    Box::new(CategoryRepoImpl::new(pool))
}
