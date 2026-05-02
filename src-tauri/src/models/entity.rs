//! 实体模型
//! 定义数据库表结构对应的实体

use serde::{Deserialize, Serialize};

/// 仓库实体（用于数据库查询）
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct RepositoryEntity {
    pub id: i64,
    pub github_id: i64,
    pub full_name: String,
    pub name: String,
    pub description: String,
    pub stargazers_count: i64,
    pub forks_count: i64,
    pub language: String,
    pub topics: String,
    pub pushed_at: String,
    pub created_at: String,
    pub html_url: String,
    pub clone_url: String,
    pub homepage: String,
    pub open_issues_count: i64,
    pub license: String,
    pub starred_at: String,
    pub owner_login: String,
    pub owner_avatar_url: String,
    pub learning_status: String,
    pub is_favorite: bool,
    pub open_pr: i64,
    pub total_pr: i64,
    pub archived: bool,
    pub deleted_at: Option<String>,
    pub status: String,
}

/// 仓库实体（用于 API 返回）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Repository {
    pub id: i64,
    pub github_id: i64,
    pub full_name: String,
    pub name: String,
    pub description: String,
    pub stargazers_count: i64,
    pub forks_count: i64,
    pub language: String,
    pub topics: String,
    pub pushed_at: String,
    pub created_at: String,
    pub html_url: String,
    pub clone_url: String,
    pub homepage: String,
    pub open_issues_count: i64,
    pub license: String,
    pub starred_at: String,
    pub owner_login: String,
    pub owner_avatar_url: String,
    pub learning_status: String,
    pub is_favorite: bool,
    pub open_pr: i64,
    pub total_pr: i64,
    pub archived: bool,
    pub deleted_at: Option<String>,
    pub categories: Vec<RepoCategory>,
    pub status: String,
}

/// 分类实体（用于仓库关联）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepoCategory {
    pub id: i64,
    pub name: String,
    pub color: String,
}

impl From<RepositoryEntity> for Repository {
    fn from(entity: RepositoryEntity) -> Self {
        Self {
            id: entity.id,
            github_id: entity.github_id,
            full_name: entity.full_name,
            name: entity.name,
            description: entity.description,
            stargazers_count: entity.stargazers_count,
            forks_count: entity.forks_count,
            language: entity.language,
            topics: entity.topics,
            pushed_at: entity.pushed_at,
            created_at: entity.created_at,
            html_url: entity.html_url,
            clone_url: entity.clone_url,
            homepage: entity.homepage,
            open_issues_count: entity.open_issues_count,
            license: entity.license,
            starred_at: entity.starred_at,
            owner_login: entity.owner_login,
            owner_avatar_url: entity.owner_avatar_url,
            learning_status: entity.learning_status,
            is_favorite: entity.is_favorite,
            open_pr: entity.open_pr,
            total_pr: entity.total_pr,
            archived: entity.archived,
            deleted_at: entity.deleted_at,
            categories: vec![RepoCategory {
                id: 0,
                name: String::new(),
                color: String::new(),
            }],
            status: entity.status,
        }
    }
}

/// 笔记实体
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct RepoNote {
    pub id: i64,
    pub repo_id: i64,
    pub note_name: String,
    pub folder: String,
    pub created_at: String,
    pub updated_at: String,
}

/// 分类实体
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub created_at: String,
    pub updated_at: String,
}

/// 合集实体
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Collection {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub color: String,
    pub created_at: String,
    pub updated_at: String,
}
