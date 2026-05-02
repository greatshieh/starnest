//! 笔记服务 - 处理笔记相关业务逻辑

use crate::models::entity::RepoNote;
use crate::repos::traits::NoteRepo;
use crate::services::traits::NoteService;
use async_trait::async_trait;
use serde_json::Value;
use tokio::sync::OnceCell;

/// 笔记服务实现
pub struct NoteServiceImpl {
    note_repo: Box<dyn NoteRepo + Send + Sync>,
}

/// 笔记服务单例
static NOTE_SERVICE: OnceCell<Box<dyn NoteService + Send + Sync>> = OnceCell::const_new();

#[async_trait]
impl NoteService for NoteServiceImpl {
    /// 保存笔记
    async fn save_note(
        &self,
        id: i64,
        github_id: i64,
        owner: &str,
        repo_name: &str,
        note_name: &str,
        content: &str,
    ) -> Result<Value, String> {
        let folder = format!("{}-{}", owner, repo_name);

        let note = if id == 0 {
            self.note_repo
                .create_note(github_id, note_name, &folder, content)
                .await?
        } else {
            let original_note = self
                .get_note_by_id_with_name(id, &folder, note_name)
                .await?;
            self.note_repo
                .update_note(id, &folder, &original_note.note_name, note_name, content)
                .await?
        };

        Ok(serde_json::json!({
            "success": true,
            "note_name": note.note_name,
            "folder": note.folder,
        }))
    }

    /// 获取笔记内容服务
    async fn read_note(
        &self,
        owner: String,
        repo_name: String,
        note_name: String,
    ) -> Result<String, String> {
        let folder = format!("{}-{}", owner, repo_name);
        self.note_repo.read_note_content(&folder, &note_name).await
    }

    /// 获取仓库笔记列表服务
    async fn get_notes_by_repo(&self, github_id: i64) -> Result<Value, String> {
        let notes = self.note_repo.get_notes_by_repo_id(github_id).await?;

        let result: Vec<Value> = notes
            .into_iter()
            .map(|note| {
                serde_json::json!({
                    "id": note.id,
                    "note_name": note.note_name,
                    "folder": note.folder,
                    "created_at": note.created_at,
                    "updated_at": note.updated_at,
                })
            })
            .collect();

        Ok(serde_json::json!(result))
    }

    /// 获取默认笔记名称服务
    async fn get_default_note_name_service(
        &self,
        owner: &str,
        repo_name: &str,
    ) -> Result<String, String> {
        let folder = format!("{}-{}", owner, repo_name);
        self.note_repo.get_default_note_name(&folder).await
    }
}

impl NoteServiceImpl {
    pub fn new(note_repo: Box<dyn NoteRepo + Send + Sync>) -> Self {
        Self { note_repo }
    }

    async fn get_note_by_id_with_name(
        &self,
        id: i64,
        folder: &str,
        _expected_name: &str,
    ) -> Result<RepoNote, String> {
        self.note_repo
            .get_note_by_id(id)
            .await
            .map(|note| RepoNote {
                folder: folder.to_string(),
                ..note
            })
    }
}

/// 获取笔记服务实例
pub async fn get_note_service() -> &'static Box<dyn NoteService + Send + Sync> {
    NOTE_SERVICE
        .get_or_init(|| async {
            let note_repo = crate::repos::note_repo::get_note_repo().await;
            let service: Box<dyn NoteService + Send + Sync> =
                Box::new(NoteServiceImpl::new(note_repo));
            service
        })
        .await
}
