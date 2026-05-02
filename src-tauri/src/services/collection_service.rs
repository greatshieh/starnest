//! 合集服务模块
//!
//! 该模块负责处理合集相关的业务逻辑，作为业务层协调仓储层和处理器层之间的数据流转。
//! 提供合集的增删改查以及仓库合集关联管理功能。

use crate::models::dto::{
    CollectionPageRequest, CollectionPageResponse, CollectionWithRepoCount,
    UpdateCollectionReposRequest,
};
use crate::models::entity::Collection;
use crate::repos::traits::CollectionRepo;
use crate::services::traits::CollectionService;
use async_trait::async_trait;
use serde_json::Value;
use tokio::sync::OnceCell;

/// 合集服务实现结构体
///
/// 封装合集相关的业务逻辑，依赖合集仓储接口进行数据访问。
pub struct CollectionServiceImpl {
    /// 合集仓储接口，用于数据库操作
    collection_repo: Box<dyn CollectionRepo + Send + Sync>,
}

/// 合集服务单例实例
///
/// 使用 OnceCell 确保全局只有一个合集服务实例，避免重复创建。
static COLLECTION_SERVICE: OnceCell<Box<dyn CollectionService + Send + Sync>> =
    OnceCell::const_new();

#[async_trait]
impl CollectionService for CollectionServiceImpl {
    /// 创建合集
    async fn create_collection(
        &self,
        name: &str,
        description: &str,
        color: &str,
    ) -> Result<CollectionWithRepoCount, String> {
        let collection = self
            .collection_repo
            .create_collection(name, description, color)
            .await?;

        Ok(CollectionWithRepoCount {
            id: collection.id,
            name: collection.name,
            description: collection.description,
            color: collection.color,
            repo_count: 0,
            created_at: collection.created_at,
            updated_at: collection.updated_at,
        })
    }

    /// 更新合集
    async fn update_collection(
        &self,
        collection_id: i64,
        name: Option<&str>,
        description: Option<&str>,
        color: Option<&str>,
    ) -> Result<CollectionWithRepoCount, String> {
        self.collection_repo
            .update_collection(collection_id, name, description, color)
            .await
    }

    /// 删除合集
    async fn delete_collection(&self, collection_id: i64) -> Result<(), String> {
        self.collection_repo.delete_collection(collection_id).await
    }

    /// 根据ID获取合集
    async fn get_collection_by_id(
        &self,
        collection_id: i64,
    ) -> Result<CollectionWithRepoCount, String> {
        self.collection_repo
            .get_collection_by_id(collection_id)
            .await
    }

    /// 获取所有合集
    async fn get_all_collections(&self) -> Result<Vec<CollectionWithRepoCount>, String> {
        self.collection_repo.get_all_collections().await
    }

    /// 分页获取合集列表
    async fn get_collections_paged(
        &self,
        request: CollectionPageRequest,
    ) -> Result<CollectionPageResponse, String> {
        self.collection_repo.get_collections_paged(request).await
    }

    /// 获取合集关联的所有仓库
    async fn get_repos_by_collection(&self, collection_id: i64) -> Result<Value, String> {
        let repos = self
            .collection_repo
            .get_repos_by_collection(collection_id)
            .await?;
        Ok(serde_json::to_value(repos).unwrap())
    }

    /// 获取仓库关联的所有合集
    async fn get_collections_by_repo(&self, github_id: i64) -> Result<Vec<Collection>, String> {
        self.collection_repo
            .get_collections_by_repo(github_id)
            .await
    }

    /// 更新合集与仓库的关联关系
    async fn update_collection_repos(
        &self,
        request: UpdateCollectionReposRequest,
    ) -> Result<(), String> {
        self.collection_repo
            .update_collection_repos(request.collection_id, &request.github_ids)
            .await
    }

    /// 检查仓库是否已在合集中
    async fn is_repo_in_collection(
        &self,
        github_id: i64,
        collection_id: i64,
    ) -> Result<bool, String> {
        self.collection_repo
            .is_repo_in_collection(github_id, collection_id)
            .await
    }
}

impl CollectionServiceImpl {
    /// 创建合集服务实例
    pub fn new(collection_repo: Box<dyn CollectionRepo + Send + Sync>) -> Self {
        Self { collection_repo }
    }
}

/// 获取合集服务单例实例
pub async fn get_collection_service() -> &'static Box<dyn CollectionService + Send + Sync> {
    COLLECTION_SERVICE
        .get_or_init(|| async {
            let collection_repo = crate::repos::collection_repo::get_collection_repo().await;
            let service: Box<dyn CollectionService + Send + Sync> =
                Box::new(CollectionServiceImpl::new(collection_repo));
            service
        })
        .await
}
