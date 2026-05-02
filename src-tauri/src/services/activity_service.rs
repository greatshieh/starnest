//! 活动服务 - 实现仓库活动相关业务逻辑

use crate::infrastructure::external_api::github_client::get_all_repo_activities;
use crate::models::dto::{DailyEventStats, EventTypeCount, GitHubActivity};
use sqlx::{Pool, Row, Sqlite};
use std::collections::BTreeMap;

pub struct ActivityServiceImpl {
    db_pool: Pool<Sqlite>,
}

impl ActivityServiceImpl {
    pub fn new(db_pool: Pool<Sqlite>) -> Self {
        Self { db_pool }
    }

    /// 获取仓库ID
    async fn get_repo_id(&self, owner: &str, repo_name: &str) -> Result<i64, String> {
        let full_name = format!("{}/{}", owner, repo_name);
        let result: Option<i64> =
            sqlx::query_scalar("SELECT github_id FROM repositories WHERE full_name = ?")
                .bind(&full_name)
                .fetch_optional(&self.db_pool)
                .await
                .map_err(|e| format!("Failed to get repo_id: {:?}", e))?;

        result.ok_or_else(|| format!("Repository not found: {}", full_name))
    }

    /// 保存事件到数据库（repo_id, created_at, event）
    async fn save_events_to_db(
        &self,
        repo_id: i64,
        activities: &[GitHubActivity],
    ) -> Result<(), String> {
        for activity in activities {
            let event_type = &activity.activity_type;
            let created_at = &activity.created_at;

            sqlx::query(
                "INSERT OR IGNORE INTO repo_events (repo_id, created_at, event) VALUES (?, ?, ?)",
            )
            .bind(repo_id)
            .bind(created_at)
            .bind(event_type)
            .execute(&self.db_pool)
            .await
            .map_err(|e| format!("Failed to save event: {:?}", e))?;
        }

        Ok(())
    }

    /// 从数据库获取按日期聚合的事件统计（前端需要的格式）
    /// 返回格式：{date: [{type, count}]}，事件按最新发生时间排序
    pub async fn get_events_by_date(&self, repo_id: i64) -> Result<Vec<DailyEventStats>, String> {
        // 查询事件，按日期和事件类型分组，获取每个类型的最新时间和数量
        let rows = sqlx::query(
            r#"
            SELECT 
                DATE(e.created_at) as date,
                e.event as event_type,
                COUNT(*) as count,
                MAX(e.created_at) as latest_time
            FROM repo_events e
            WHERE e.repo_id = ?
            GROUP BY DATE(e.created_at), e.event
            ORDER BY date DESC, latest_time DESC
            "#,
        )
        .bind(repo_id)
        .fetch_all(&self.db_pool)
        .await
        .map_err(|e| format!("Failed to query events: {:?}", e))?;

        // 按日期分组
        let mut daily_map: BTreeMap<String, Vec<(String, u32, String)>> = BTreeMap::new();
        for row in rows {
            let date: String = row.get("date");
            let event_type: String = row.get("event_type");
            let count: i64 = row.get("count");
            let latest_time: String = row.get("latest_time");

            daily_map.entry(date).or_insert_with(Vec::new).push((
                event_type,
                count as u32,
                latest_time,
            ));
        }

        // 转换为前端需要的格式
        let daily_stats: Vec<DailyEventStats> = daily_map
            .into_iter()
            .rev() // 按日期倒序
            .map(|(date, events)| {
                // 按最新时间排序事件类型
                let mut sorted_events = events;
                sorted_events.sort_by(|a, b| b.2.cmp(&a.2));

                let event_counts: Vec<EventTypeCount> = sorted_events
                    .into_iter()
                    .map(|(event_type, count, _)| EventTypeCount {
                        r#type: event_type,
                        count,
                    })
                    .collect();

                DailyEventStats {
                    date,
                    events: event_counts,
                }
            })
            .collect();

        Ok(daily_stats)
    }

    /// 从GitHub获取活动并保存到数据库
    pub async fn fetch_and_save_activities(
        &self,
        owner: &str,
        repo_name: &str,
    ) -> Result<(), String> {
        // 获取所有活动（使用Link header翻页）
        let activities = get_all_repo_activities(owner, repo_name).await?;

        // 获取仓库ID
        let repo_id = self.get_repo_id(owner, repo_name).await?;

        // 保存事件到数据库
        self.save_events_to_db(repo_id, &activities).await?;

        Ok(())
    }
}

/// 获取按日期聚合的事件统计（公开函数）
pub async fn get_events_by_date(repo_id: i64) -> Result<Vec<DailyEventStats>, String> {
    let db_pool = crate::db::sqlite::get_pool()
        .map_err(|e| format!("Failed to get database pool: {}", e))
        .unwrap();
    let service = ActivityServiceImpl::new(db_pool.clone());
    service.get_events_by_date(repo_id).await
}

/// 从GitHub获取活动并保存到数据库（公开函数）
pub async fn fetch_and_save_activities(owner: &str, repo_name: &str) -> Result<(), String> {
    let db_pool = crate::db::sqlite::get_pool()
        .map_err(|e| format!("Failed to get database pool: {}", e))
        .unwrap();
    let service = ActivityServiceImpl::new(db_pool.clone());
    service.fetch_and_save_activities(owner, repo_name).await
}
