//! 分类服务模块
//!
//! 该模块负责处理分类相关的业务逻辑，作为业务层协调仓储层和处理器层之间的数据流转。
//! 提供分类的增删改查以及仓库分类关联管理功能。

use crate::models::dto::{
    CategoryPageRequest, CategoryPageResponse, CategoryWithCount, UpdateRepoCategoriesRequest,
};
use crate::repos::traits::CategoryRepo;
use crate::services::traits::CategoryService;
use async_trait::async_trait;
use tokio::sync::OnceCell;

/// 分类服务实现结构体
///
/// 封装分类相关的业务逻辑，依赖分类仓储接口进行数据访问。
pub struct CategoryServiceImpl {
    /// 分类仓储接口，用于数据库操作
    category_repo: Box<dyn CategoryRepo + Send + Sync>,
}

/// 分类服务单例实例
///
/// 使用 OnceCell 确保全局只有一个分类服务实例，避免重复创建。
static CATEGORY_SERVICE: OnceCell<Box<dyn CategoryService + Send + Sync>> = OnceCell::const_new();

#[async_trait]
impl CategoryService for CategoryServiceImpl {
    /// 获取所有分类及其关联的仓库数量
    ///
    /// 查询数据库中所有分类，并统计每个分类关联的仓库数量。
    ///
    /// # Returns
    /// - `Result<Vec<CategoryWithCount>, String>`: 分类列表（含仓库数量）或错误信息
    async fn get_all_categories(&self) -> Result<Vec<CategoryWithCount>, String> {
        // 委托给仓储层获取所有分类及其仓库数量
        self.category_repo.get_all_categories_with_count().await
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
        // 委托给仓储层进行分页查询
        self.category_repo.get_categories_paged(request).await
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
        // 委托给仓储层更新分类
        self.category_repo
            .update_category(category_id, name, color)
            .await
    }

    /// 为仓库更新分类关联
    ///
    /// 根据请求中的仓库ID和分类ID列表，更新仓库与分类的关联关系。
    /// 会先删除不再关联的分类，再添加新的分类关联。
    ///
    /// # Arguments
    /// - `request`: 更新分类关联的请求对象
    ///
    /// # Returns
    /// - `Result<(), String>`: 成功返回空，失败返回错误信息
    async fn update_repo_categories(
        &self,
        request: UpdateRepoCategoriesRequest,
    ) -> Result<(), String> {
        // 委托给仓储层更新仓库分类关联
        self.category_repo
            .update_repo_categories(request.repo_id, &request.category_ids)
            .await
    }

    /// 创建分类
    ///
    /// 根据指定的名称和颜色创建一个新的分类，并返回创建后的分类信息。
    ///
    /// # Arguments
    /// - `name`: 分类名称
    /// - `color`: 分类颜色（十六进制格式）
    ///
    /// # Returns
    /// - `Result<CategoryWithCount, String>`: 创建成功的分类信息或错误信息
    async fn create_category(&self, name: &str, color: &str) -> Result<CategoryWithCount, String> {
        // 委托给仓储层创建分类
        let category = self.category_repo.create_category(name, color).await?;

        // 将实体转换为 DTO 返回，初始 repo_count 设为 0
        Ok(CategoryWithCount {
            id: category.id,
            name: category.name,
            color: category.color,
            repo_count: 0,
            updated_at: category.updated_at,
        })
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
    async fn delete_category(&self, category_id: i64) -> Result<(), String> {
        // 委托给仓储层删除分类
        self.category_repo.delete_category(category_id).await
    }
}

impl CategoryServiceImpl {
    /// 创建分类服务实例
    ///
    /// # Arguments
    /// - `category_repo`: 分类仓储实例
    ///
    /// # Returns
    /// - `Self`: 分类服务实例
    pub fn new(category_repo: Box<dyn CategoryRepo + Send + Sync>) -> Self {
        Self { category_repo }
    }
}

/// 获取分类服务单例实例
///
/// 使用 OnceCell 懒加载创建分类服务单例，确保全局只有一个实例。
///
/// # Returns
/// - `&'static Box<dyn CategoryService + Send + Sync>`: 分类服务单例引用
pub async fn get_category_service() -> &'static Box<dyn CategoryService + Send + Sync> {
    CATEGORY_SERVICE
        .get_or_init(|| async {
            // 获取分类仓储实例
            let category_repo = crate::repos::category_repo::get_category_repo().await;
            // 创建分类服务实例并装箱
            let service: Box<dyn CategoryService + Send + Sync> =
                Box::new(CategoryServiceImpl::new(category_repo));
            service
        })
        .await
}
