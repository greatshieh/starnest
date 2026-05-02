use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::fs;
use std::fs::File;
use std::sync::OnceLock;
use std::time::Duration;

use super::config;

static DB_POOL: OnceLock<Pool<Sqlite>> = OnceLock::new();

pub fn set_pool(pool: Pool<Sqlite>) -> Result<(), String> {
    match DB_POOL.set(pool) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to set pool: {:?}", e)),
    }
}

pub fn get_pool() -> Result<&'static Pool<Sqlite>, String> {
    Ok(DB_POOL.get().unwrap())
}

pub async fn init_db(config: &config::Config) -> Result<(), String> {
    let db_path = config::get_database_path(&config)?;
    let app_data_dir = db_path.parent().ok_or("Failed to get parent directory")?;

    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir)
            .map_err(|e| format!("Failed to create directory: {:?}", e))?;
    }

    let db_url = format!("sqlite://{}", db_path.to_str().unwrap_or_default());

    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url)
            .await
            .expect("Failed to create database file");
    }

    let pool = match SqlitePoolOptions::new()
        .max_connections(5)
        .idle_timeout(Duration::from_secs(60))
        .connect(&db_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            let _ = File::create(&db_path);
            return Err(format!("Failed to connect: {:?}", e));
        }
    };

    sqlx::query("PRAGMA journal_mode = WAL;")
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to set WAL mode: {:?}", e))?;

    sqlx::query("PRAGMA synchronous = NORMAL;")
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to set synchronous mode: {:?}", e))?;

    sqlx::query("PRAGMA busy_timeout = 5000;")
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to set busy timeout: {:?}", e))?;

    // 初始化数据库表结构
    init_tables(&pool)
        .await
        .map_err(|e| format!("Failed to initialize tables: {:?}", e))?;

    set_pool(pool).map_err(|e| format!("Failed to set pool: {:?}", e))?;

    Ok(())
}

pub async fn init_tables(pool: &Pool<Sqlite>) -> Result<(), String> {
    // 创建 repositories 表
    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS repositories (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        github_id INTEGER UNIQUE NOT NULL,
        full_name TEXT NOT NULL,
        name TEXT NOT NULL,
        description TEXT,
        stargazers_count INTEGER NOT NULL,
        forks_count INTEGER NOT NULL,
        topics TEXT,
        language TEXT,
        pushed_at TEXT NOT NULL,
        created_at TEXT NOT NULL,
        html_url TEXT NOT NULL,
        clone_url TEXT NOT NULL,
        homepage TEXT,
        open_issues_count INTEGER NOT NULL,
        license TEXT,
        starred_at TEXT NOT NULL,
        owner_login TEXT NOT NULL,
        owner_avatar_url TEXT NOT NULL,
        learning_status TEXT NOT NULL DEFAULT 'not_started',
        is_favorite BOOLEAN NOT NULL DEFAULT 0,
        open_pr INTEGER NOT NULL DEFAULT 0,
        total_pr INTEGER NOT NULL DEFAULT 0,
        archived BOOLEAN NOT NULL DEFAULT 0,
        deleted_at TEXT,
        status TEXT NOT NULL DEFAULT 'active' CHECK(status IN ('active', 'inactive', 'deprecated', 'archived'))
    )
    "#,
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create repositories table: {:?}", e))?;

    // 创建 repositories 表索引
    sqlx::query(
        r#"CREATE INDEX IF NOT EXISTS idx_repositories_full_name ON repositories(full_name)"#,
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create idx_repositories_full_name index: {:?}", e))?;

    sqlx::query(
        r#"CREATE INDEX IF NOT EXISTS idx_repositories_language ON repositories(language)"#,
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create idx_repositories_language index: {:?}", e))?;

    sqlx::query(
        r#"CREATE INDEX IF NOT EXISTS idx_repositories_starred_at ON repositories(starred_at)"#,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        format!(
            "Failed to create idx_repositories_starred_at index: {:?}",
            e
        )
    })?;

    // 创建全文搜索虚拟表
    sqlx::query(
        r#"
    CREATE VIRTUAL TABLE IF NOT EXISTS repos_fts USING fts5(
        owner_login,
        name,
        description,
        language,
        topics,
        content='repositories',
        content_rowid='id'
    )
    "#,
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create repos_fts virtual table: {:?}", e))?;

    // 创建 FTS 插入触发器
    sqlx::query(
        r#"
    CREATE TRIGGER IF NOT EXISTS repos_fts_insert AFTER INSERT ON repositories
    BEGIN
        INSERT INTO repos_fts(rowid, owner_login, name, description, language, topics)
        VALUES (new.id, new.owner_login, new.name, new.description, new.language, new.topics);
    END
    "#,
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create repos_fts_insert trigger: {:?}", e))?;

    // 创建 FTS 更新触发器
    sqlx::query(
        r#"
    CREATE TRIGGER IF NOT EXISTS repos_fts_update AFTER UPDATE ON repositories
    BEGIN
        INSERT INTO repos_fts(repos_fts, rowid, owner_login, name, description, language, topics)
        VALUES ('delete', old.id, old.owner_login, old.name, old.description, old.language, old.topics);
        INSERT INTO repos_fts(rowid, owner_login, name, description, language, topics)
        VALUES (new.id, new.owner_login, new.name, new.description, new.language, new.topics);
    END
    "#,
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create repos_fts_update trigger: {:?}", e))?;

    // 创建 FTS 删除触发器
    sqlx::query(
        r#"
    CREATE TRIGGER IF NOT EXISTS repos_fts_delete AFTER DELETE ON repositories
    BEGIN
        INSERT INTO repos_fts(repos_fts, rowid, owner_login, name, description, language, topics)
        VALUES ('delete', old.id, old.owner_login, old.name, old.description, old.language, old.topics);
    END
    "#,
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create repos_fts_delete trigger: {:?}", e))?;

    // 同步现有数据到 FTS
    sqlx::query(
        r#"
    INSERT OR IGNORE INTO repos_fts(rowid, owner_login, name, description, language, topics)
    SELECT id, owner_login, name, COALESCE(description, ''), COALESCE(language, ''), COALESCE(topics, '')
    FROM repositories
    WHERE id NOT IN (SELECT rowid FROM repos_fts)
    "#,
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to sync existing data to repos_fts: {:?}", e))?;

    // 创建 repo_notes 表
    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS repo_notes (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        repo_id INTEGER NOT NULL,
        note_name TEXT NOT NULL,
        folder TEXT NOT NULL,
        created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
        updated_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
        FOREIGN KEY (repo_id) REFERENCES repositories(github_id) ON DELETE CASCADE
    )
    "#,
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create repo_notes table: {:?}", e))?;

    // 创建 repo_notes 表索引
    sqlx::query(r#"CREATE INDEX IF NOT EXISTS idx_repo_notes_repo_id ON repo_notes(repo_id)"#)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to create idx_repo_notes_repo_id index: {:?}", e))?;

    sqlx::query(r#"CREATE INDEX IF NOT EXISTS idx_repo_notes_note_name ON repo_notes(note_name)"#)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to create idx_repo_notes_note_name index: {:?}", e))?;

    // 创建 categories 表
    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS categories (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE,
        color TEXT DEFAULT '#7800ce',
        created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
        updated_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
    )
    "#,
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create categories table: {:?}", e))?;

    // 创建 categories 表索引
    sqlx::query(r#"CREATE INDEX IF NOT EXISTS idx_categories_name ON categories(name)"#)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to create idx_categories_name index: {:?}", e))?;

    // 创建 repo_category_relation 表
    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS repo_category_relation (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        repo_id INTEGER NOT NULL,
        category_id INTEGER NOT NULL,
        UNIQUE(repo_id, category_id)
    )
    "#,
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create repo_category_relation table: {:?}", e))?;

    // 创建 repo_category_relation 表索引
    sqlx::query(r#"CREATE INDEX IF NOT EXISTS idx_repo_category_repo_id ON repo_category_relation(repo_id)"#)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to create idx_repo_category_repo_id index: {:?}", e))?;

    sqlx::query(r#"CREATE INDEX IF NOT EXISTS idx_repo_category_category_id ON repo_category_relation(category_id)"#)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to create idx_repo_category_category_id index: {:?}", e))?;

    // 创建 repo_events 表
    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS repo_events (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        repo_id INTEGER NOT NULL,
        created_at TEXT NOT NULL,
        event TEXT NOT NULL,
        UNIQUE(repo_id, created_at, event)
    )
    "#,
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create repo_events table: {:?}", e))?;

    // 创建 repo_events 表索引
    sqlx::query(r#"CREATE INDEX IF NOT EXISTS idx_repo_events_repo_id ON repo_events(repo_id)"#)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to create idx_repo_events_repo_id index: {:?}", e))?;

    sqlx::query(
        r#"CREATE INDEX IF NOT EXISTS idx_repo_events_created_at ON repo_events(created_at)"#,
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create idx_repo_events_created_at index: {:?}", e))?;

    // 创建 collections 表
    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS collections (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        description TEXT,
        color TEXT NOT NULL DEFAULT '#6b7280',
        created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
        updated_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
    )
    "#,
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create collections table: {:?}", e))?;

    // 创建 collections 表索引
    sqlx::query(r#"CREATE INDEX IF NOT EXISTS idx_collections_name ON collections(name)"#)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to create idx_collections_name index: {:?}", e))?;

    // 创建 repo_collection_relation 表
    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS repo_collection_relation (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        github_id INTEGER NOT NULL,
        collection_id INTEGER NOT NULL,
        UNIQUE(github_id, collection_id)
    )
    "#,
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create repo_collection_relation table: {:?}", e))?;

    // 创建 repo_collection_relation 表索引
    sqlx::query(r#"CREATE INDEX IF NOT EXISTS idx_repo_collection_github_id ON repo_collection_relation(github_id)"#)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to create idx_repo_collection_github_id index: {:?}", e))?;

    sqlx::query(r#"CREATE INDEX IF NOT EXISTS idx_repo_collection_collection_id ON repo_collection_relation(collection_id)"#)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to create idx_repo_collection_collection_id index: {:?}", e))?;

    Ok(())
}
