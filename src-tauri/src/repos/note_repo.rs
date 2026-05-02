//! 笔记仓储 - 笔记相关的数据访问操作

use crate::models::entity::RepoNote;
use crate::repos::config_repo::{
    get_note_storage_path, load_config_internal as load_config,
};
use crate::repos::traits::NoteRepo;
use async_trait::async_trait;
use sqlx::Pool;
use sqlx::Sqlite;
use std::fs;

/// 笔记仓库实现
pub struct NoteRepoImpl {
    pool: &'static Pool<Sqlite>,
}

#[async_trait]
impl NoteRepo for NoteRepoImpl {
    /// 创建笔记
    async fn create_note(
        &self,
        repo_id: i64,
        note_name: &str,
        folder: &str,
        content: &str,
    ) -> Result<RepoNote, String> {
        let exists: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM repo_notes WHERE repo_id = ? AND note_name = ?")
                .bind(repo_id)
                .bind(note_name)
                .fetch_one(self.pool)
                .await
                .map_err(|e| format!("Failed to check note name: {:?}", e))?;

        if exists.0 > 0 {
            return Err(format!("Note with name '{}' already exists", note_name));
        }

        let result = sqlx::query(
            "INSERT INTO repo_notes (repo_id, note_name, folder, created_at, updated_at)
             VALUES (?, ?, ?, datetime('now', 'localtime'), datetime('now', 'localtime'))",
        )
        .bind(repo_id)
        .bind(note_name)
        .bind(folder)
        .execute(self.pool)
        .await
        .map_err(|e| format!("Failed to insert note into database: {:?}", e))?;

        let note_id = result.last_insert_rowid();

        self.save_note_file(folder, note_name, content).await?;

        self.get_note_by_id(note_id).await
    }

    /// 更新笔记
    async fn update_note(
        &self,
        id: i64,
        folder: &str,
        original_name: &str,
        new_name: &str,
        content: &str,
    ) -> Result<RepoNote, String> {
        if original_name != new_name {
            let exists: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM repo_notes WHERE folder = ? AND note_name = ?",
            )
            .bind(folder)
            .bind(new_name)
            .fetch_one(self.pool)
            .await
            .map_err(|e| format!("Failed to check note name: {:?}", e))?;

            if exists.0 > 0 {
                return Err(format!("Note with name '{}' already exists", new_name));
            }

            let config = load_config()?;
            let note_base_path = get_note_storage_path(&config)?;
            let repo_dir = note_base_path.join(folder);
            let old_note_file = repo_dir.join(original_name);

            if old_note_file.exists() {
                fs::remove_file(&old_note_file)
                    .map_err(|e| format!("Failed to remove old note file: {:?}", e))?;
            }
        }

        sqlx::query(
            "UPDATE repo_notes SET note_name = ?, updated_at = datetime('now', 'localtime') WHERE id = ?",
        )
        .bind(new_name)
        .bind(id)
        .execute(self.pool)
        .await
        .map_err(|e| format!("Failed to update note in database: {:?}", e))?;

        self.save_note_file(folder, new_name, content).await?;

        self.get_note_by_id(id).await
    }

    /// 根据ID获取笔记
    async fn get_note_by_id(&self, id: i64) -> Result<RepoNote, String> {
        let note = sqlx::query_as::<_, RepoNote>("SELECT * FROM repo_notes WHERE id = ?")
            .bind(id)
            .fetch_one(self.pool)
            .await
            .map_err(|e| format!("Failed to get note: {:?}", e))?;

        Ok(note)
    }

    /// 根据仓库ID获取笔记列表
    async fn get_notes_by_repo_id(&self, repo_id: i64) -> Result<Vec<RepoNote>, String> {
        let notes = sqlx::query_as::<_, RepoNote>(
            "SELECT id, repo_id, note_name, folder, created_at, updated_at 
             FROM repo_notes WHERE repo_id = ? ORDER BY updated_at DESC",
        )
        .bind(repo_id)
        .fetch_all(self.pool)
        .await
        .map_err(|e| format!("Failed to get notes: {:?}", e))?;

        Ok(notes)
    }

    /// 读取笔记内容
    async fn read_note_content(&self, folder: &str, note_name: &str) -> Result<String, String> {
        let config = load_config()?;
        let note_base_path = get_note_storage_path(&config)?;
        let repo_dir = note_base_path.join(folder);
        let note_file = repo_dir.join(format!("{}.md", note_name));

        if !note_file.exists() {
            return Err(format!("Note file not found: {}", note_file.display()));
        }

        fs::read_to_string(&note_file).map_err(|e| format!("Failed to read note file: {:?}", e))
    }

    /// 获取默认笔记名称
    async fn get_default_note_name(&self, folder: &str) -> Result<String, String> {
        let config = load_config()?;
        let note_base_path = get_note_storage_path(&config)?;
        let repo_dir = note_base_path.join(folder);

        let mut count = 0;
        if repo_dir.exists() {
            if let Ok(entries) = fs::read_dir(&repo_dir) {
                for entry in entries.flatten() {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    if file_name.starts_with("new-note-") && file_name.ends_with(".md") {
                        if let Some(num_str) = file_name
                            .strip_prefix("new-note-")
                            .and_then(|s| s.strip_suffix(".md"))
                        {
                            if let Ok(num) = num_str.parse::<u32>() {
                                if num > count {
                                    count = num;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(format!("new-note-{}", count + 1))
    }
}

impl NoteRepoImpl {
    /// 创建笔记仓库实例
    pub fn new(pool: &'static Pool<Sqlite>) -> Self {
        Self { pool }
    }

    /// 保存笔记文件
    async fn save_note_file(
        &self,
        folder: &str,
        note_name: &str,
        content: &str,
    ) -> Result<(), String> {
        let config = load_config()?;
        let note_base_path = get_note_storage_path(&config)?;
        let repo_dir = note_base_path.join(folder);

        if !repo_dir.exists() {
            fs::create_dir_all(&repo_dir)
                .map_err(|e| format!("Failed to create repo directory: {:?}", e))?;
        }

        let note_file = repo_dir.join(format!("{}.md", note_name));
        fs::write(&note_file, content).map_err(|e| format!("Failed to write note file: {:?}", e))
    }
}

/// 获取笔记仓库实例
pub async fn get_note_repo() -> Box<dyn NoteRepo + Send + Sync> {
    let pool = crate::db::sqlite::get_pool()
        .map_err(|e| format!("Failed to get database pool: {}", e))
        .unwrap();
    Box::new(NoteRepoImpl::new(pool))
}
