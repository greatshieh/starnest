//! GitHub API 客户端
//!
//! 封装 GitHub API 的调用逻辑。

use crate::infrastructure::utils::hash::compute_sha256_hash;
use crate::models::dto::{
    GitHubRepoResponse, GitHubStarredResponse, GitHubUserResponse, RepoSyncResult,
};
use crate::repos::config_repo::load_config_internal as load_config;
use base64::{engine::general_purpose, Engine};
use chrono::{Duration, Utc};
use dirs;
use reqwest::Client;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Listener, Runtime};

/// 解析 Link 头中的指定关系的 URL
fn parse_link_url(link_header: &str, rel: &str) -> Option<String> {
    for part in link_header.split(',') {
        let part = part.trim();
        if part.contains(&format!("rel=\"{}\"", rel)) {
            if let Some(start) = part.find('<') {
                if let Some(end) = part.find('>') {
                    return Some(part[start + 1..end].to_string());
                }
            }
        }
    }
    None
}

/// 获取用户信息
pub async fn get_user_info(token: &str) -> Result<GitHubUserResponse, String> {
    let client = Client::new();

    let response = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "GitHub-Star-Manager")
        .send()
        .await
        .map_err(|e| format!("Failed to send user request: {:?}", e))?
        .json::<GitHubUserResponse>()
        .await
        .map_err(|e| format!("Failed to parse user response: {:?}", e))?;

    Ok(response)
}

/// 获取用户收藏的仓库列表
pub async fn get_starred_repos<R: Runtime>(
    token: &str,
    app_handle: &AppHandle<R>,
    cancel_flag: &Arc<AtomicBool>,
) -> Result<(Vec<GitHubRepoResponse>, u32), String> {
    let client = Client::new();
    let mut repos = Vec::new();
    let per_page = 100;

    let first_url = format!(
        "https://api.github.com/user/starred?page=1&per_page={}",
        per_page
    );
    let first_response = client
        .get(&first_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "GitHub-Star-Manager")
        .header("Accept", "application/vnd.github.v3.star+json")
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {:?}", e))?;

    if !first_response.status().is_success() {
        return Err(format!(
            "GitHub API returned error: {}",
            first_response.status()
        ));
    }

    let link_header = first_response.headers().get(reqwest::header::LINK).cloned();

    let first_page_starred: Vec<GitHubStarredResponse> = first_response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {:?}", e))?;
    let mut first_page_repos: Vec<GitHubRepoResponse> = first_page_starred
        .into_iter()
        .map(|s| {
            let mut repo = s.repo;
            repo.starred_at = Some(s.starred_at);
            repo
        })
        .collect();
    if first_page_repos.is_empty() {
        return Ok((vec![], 0));
    }

    // 并发获取第一页仓库的语言信息（最多10个并发）
    first_page_repos = fetch_repo_languages_concurrent(token, first_page_repos, 10).await;

    repos.extend(first_page_repos);

    let _next_url: Option<String> = if let Some(ref link) = link_header {
        if let Ok(link_str) = link.to_str() {
            parse_link_url(link_str, "next")
        } else {
            None
        }
    } else {
        None
    };

    let total_count = if let Some(link) = link_header {
        if let Ok(link_str) = link.to_str() {
            if let Some(last_url) = parse_link_url(link_str, "last") {
                if let Some(page_pos) = last_url.find("page=") {
                    let page_str = &last_url[page_pos + 5..];
                    let end_pos = page_str
                        .find(|c| c == '&' || c == '?')
                        .unwrap_or(page_str.len());
                    if let Ok(last_page) = page_str[..end_pos].parse::<u32>() {
                        let last_response = client
                            .get(&last_url)
                            .header("Authorization", format!("Bearer {}", token))
                            .header("User-Agent", "GitHub-Star-Manager")
                            .header("Accept", "application/vnd.github.v3.star+json")
                            .send()
                            .await
                            .map_err(|e| format!("Failed to send request: {:?}", e))?;

                        if last_response.status().is_success() {
                            let last_page_starred: Vec<GitHubStarredResponse> = last_response
                                .json()
                                .await
                                .map_err(|e| format!("Failed to parse response: {:?}", e))?;
                            let last_page_repos: Vec<GitHubRepoResponse> = last_page_starred
                                .into_iter()
                                .map(|s| {
                                    let mut repo = s.repo;
                                    repo.starred_at = Some(s.starred_at);
                                    repo
                                })
                                .collect();
                            (last_page - 1) * per_page + last_page_repos.len() as u32
                        } else {
                            repos.len() as u32
                        }
                    } else {
                        repos.len() as u32
                    }
                } else {
                    repos.len() as u32
                }
            } else {
                repos.len() as u32
            }
        } else {
            repos.len() as u32
        }
    } else {
        repos.len() as u32
    };

    emit_sync_progress(app_handle, "fetching", repos.len(), total_count);

    let mut page = 2;
    loop {
        if cancel_flag.load(Ordering::Relaxed) {
            return Err("Sync cancelled".to_string());
        }

        let url = format!(
            "https://api.github.com/user/starred?page={}&per_page={}",
            page, per_page
        );
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", "GitHub-Star-Manager")
            .header("Accept", "application/vnd.github.v3.star+json")
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {:?}", e))?;

        if !response.status().is_success() {
            return Err(format!("GitHub API returned error: {}", response.status()));
        }

        let page_starred: Vec<GitHubStarredResponse> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {:?}", e))?;
        let mut page_repos: Vec<GitHubRepoResponse> = page_starred
            .into_iter()
            .map(|s| {
                let mut repo = s.repo;
                repo.starred_at = Some(s.starred_at);
                repo
            })
            .collect();
        if page_repos.is_empty() {
            break;
        }

        // 并发获取当前页仓库的语言信息（最多10个并发）
        page_repos = fetch_repo_languages_concurrent(token, page_repos, 10).await;

        repos.extend(page_repos);

        emit_sync_progress(app_handle, "fetching", repos.len(), total_count);

        page += 1;
    }

    Ok((repos, total_count))
}

/// 并发获取仓库语言信息
///
/// # 参数
/// - `token`: GitHub 访问令牌
/// - `repos`: 仓库列表
/// - `concurrency`: 并发数
///
/// # 返回值
/// 返回带有语言信息的仓库列表
async fn fetch_repo_languages_concurrent(
    token: &str,
    repos: Vec<GitHubRepoResponse>,
    concurrency: usize,
) -> Vec<GitHubRepoResponse> {
    let mut repos_with_language = Vec::with_capacity(repos.len());

    // 将仓库列表分批处理
    let chunks: Vec<Vec<GitHubRepoResponse>> =
        repos
            .into_iter()
            .enumerate()
            .fold(Vec::new(), |mut acc, (i, repo)| {
                if i % concurrency == 0 {
                    acc.push(Vec::new());
                }
                acc.last_mut().unwrap().push(repo);
                acc
            });

    for chunk in chunks {
        // 创建并发任务
        let futures: Vec<_> = chunk
            .into_iter()
            .map(|mut repo| {
                let token_clone = token.to_string();
                async move {
                    // 如果 language 已经有值，调用 API 获取详细的语言分布
                    if repo.language.is_some() {
                        let owner = repo.owner.login.clone();
                        let repo_name = repo.name.clone();

                        match get_repo_language(&token_clone, &owner, &repo_name).await {
                            Ok(language) => {
                                repo.language = Some(language);
                            }
                            Err(e) => {
                                eprintln!(
                                    "Failed to fetch language for {}/{}: {}",
                                    owner, repo_name, e
                                )
                            }
                        }
                    }

                    repo
                }
            })
            .collect();

        // 等待当前批次所有任务完成
        let results = futures::future::join_all(futures).await;
        repos_with_language.extend(results);
    }

    repos_with_language
}

/// 获取仓库的语言分布（归一化处理，总和保证为 100）
///
/// # 参数
/// - `token`: GitHub 访问令牌
/// - `owner`: 仓库所有者
/// - `repo_name`: 仓库名称
///
/// # 返回值
/// 返回语言分布的 JSON 字符串，格式为 `{"Language1": percentage, "Language2": percentage, ...}`
async fn get_repo_language(token: &str, owner: &str, repo_name: &str) -> Result<String, String> {
    let client = Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/languages",
        owner, repo_name
    );

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "GitHub-Star-Manager")
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {:?}", e))?;

    if !response.status().is_success() {
        return Err(format!("GitHub API returned error: {}", response.status()));
    }

    let languages: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {:?}", e))?;

    let mut lang_bytes: Vec<(String, u64)> = Vec::new();

    if let serde_json::Value::Object(map) = languages {
        for (lang, bytes) in map {
            if let Some(byte_count) = bytes.as_u64() {
                lang_bytes.push((lang, byte_count));
            }
        }
    }

    if lang_bytes.is_empty() {
        return Err("No language found".to_string());
    }

    // 如果只有一种语言，直接返回 100
    if lang_bytes.len() == 1 {
        let result = serde_json::json!({
            lang_bytes[0].0.clone(): 100
        });
        return Ok(result.to_string());
    }

    // 计算总字节数
    let total_bytes: u64 = lang_bytes.iter().map(|(_, bytes)| bytes).sum();

    if total_bytes == 0 {
        return Err("Total bytes is zero".to_string());
    }

    // 计算每种语言的精确百分比
    let lang_percentages: Vec<(String, f64)> = lang_bytes
        .into_iter()
        .map(|(lang, bytes)| {
            let percentage = (bytes as f64 / total_bytes as f64) * 100.0;
            (lang, percentage)
        })
        .collect();

    // 分离小语言（百分比 < 1）和大语言
    let mut small_langs: Vec<(String, f64)> = Vec::new();
    let mut large_langs: Vec<(String, f64)> = Vec::new();

    for (lang, percentage) in lang_percentages {
        if percentage < 1.0 {
            small_langs.push((lang, percentage));
        } else {
            large_langs.push((lang, percentage));
        }
    }

    // 处理小语言合并逻辑
    let mut combined_langs: Vec<(String, f64)> = large_langs;

    if small_langs.len() >= 2 {
        // 两种或以上小语言，合并为 other
        let other_sum: f64 = small_langs.iter().map(|(_, p)| p).sum();
        combined_langs.push(("other".to_string(), other_sum));
    } else if small_langs.len() == 1 {
        // 只有一种小语言，不合并
        combined_langs.push(small_langs[0].clone());
    }

    // 阶段二：向下取整 + 剩余分配算法确保总和为 100

    // 计算向下取整值和小数部分
    let mut floored_values: std::collections::HashMap<String, u64> =
        std::collections::HashMap::new();
    let mut fractional_parts: std::collections::HashMap<String, f64> =
        std::collections::HashMap::new();

    for (lang, percentage) in &combined_langs {
        let floored = percentage.floor() as u64;
        floored_values.insert(lang.clone(), floored);
        fractional_parts.insert(lang.clone(), percentage - percentage.floor());
    }

    // 计算当前总和和差值
    let current_sum: u64 = floored_values.values().sum();
    let diff: i64 = 100i64 - current_sum as i64;

    // 按小数部分降序排序
    let mut sorted_langs: Vec<(String, f64)> = combined_langs
        .into_iter()
        .map(|(lang, _p)| {
            let frac = fractional_parts[&lang];
            (lang, frac)
        })
        .collect();
    sorted_langs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // 分配剩余点数
    for i in 0..diff.max(0) as usize {
        let lang = &sorted_langs[i % sorted_langs.len()].0;
        *floored_values.get_mut(lang).unwrap() += 1;
    }

    // 阶段三：排序输出（降序排列，other 放在最后，删除值为 0 的语言）
    let result_langs: Vec<(String, u64)> = floored_values.into_iter().collect();

    // 分离 other 和其他语言，同时过滤掉值为 0 的语言
    let mut other_entry: Option<(String, u64)> = None;
    let mut sorted_result: Vec<(String, u64)> = Vec::new();

    for (lang, value) in result_langs {
        // 过滤掉值为 0 的语言
        if value == 0 {
            continue;
        }

        if lang == "other" {
            other_entry = Some((lang, value));
        } else {
            sorted_result.push((lang, value));
        }
    }

    // 其他语言按值降序排序
    sorted_result.sort_by(|a, b| b.1.cmp(&a.1));

    // 将 other 放在最后（如果 other 存在且值不为 0）
    if let Some(other) = other_entry {
        sorted_result.push(other);
    }

    // 构建最终结果
    let mut result: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
    for (lang, value) in sorted_result {
        result.insert(lang, serde_json::Value::Number(value.into()));
    }

    let result_json = serde_json::Value::Object(result);
    Ok(result_json.to_string())
}

/// 发送同步进度事件
fn emit_sync_progress<R: Runtime>(
    app_handle: &AppHandle<R>,
    stage: &str,
    current: usize,
    total: u32,
) {
    app_handle
        .emit(
            "sync_progress",
            serde_json::json!({
                "stage": stage,
                "current": current,
                "total": total,
                "percentage": (current as f64 / total as f64 * 100.0).round()
            }),
        )
        .ok();
}

/// 获取仓库 README
pub async fn get_repo_readme(owner: &str, repo_name: &str) -> Result<String, String> {
    let config = load_config().ok();
    let cache_key = generate_readme_cache_key(owner, repo_name);

    if let Some(cache_base) = dirs::cache_dir() {
        let readme_cache_dir = cache_base.join("starnest").join("readme");
        let readme_file = readme_cache_dir.join(&cache_key);

        if readme_file.exists() {
            if let Ok(content) = std::fs::read_to_string(&readme_file) {
                return Ok(content);
            }
        }
    }

    let token = config
        .as_ref()
        .and_then(|c| c.auth.as_ref().map(|a| a.access_token.clone()));

    let client = Client::new();

    let url = format!(
        "https://api.github.com/repos/{}/{}/readme",
        owner, repo_name
    );

    let response = client
        .get(&url)
        .header("User-Agent", "GitHub-Star-Manager")
        .header(
            "Authorization",
            token.map_or_else(|| "".to_string(), |t| format!("Bearer {}", t)),
        )
        .header("Accept", "vnd.github+json")
        .header("X-GitHub-Api-Version", "2026-03-10")
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {:?}", e))?;

    if response.status() == 404 {
        return Err("README not found".to_string());
    }

    if !response.status().is_success() {
        return Err(format!("GitHub API returned error: {}", response.status()));
    }

    let readme_data: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {:?}", e))?;

    let mut content = readme_data
        .get("content")
        .and_then(|c| c.as_str())
        .ok_or_else(|| "Failed to get README content".to_string())?
        .to_string();

    content.retain(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=');

    let decoded_content = general_purpose::STANDARD
        .decode(content)
        .map_err(|e| format!("Failed to decode base64 content: {:?}", e))?;

    let content_str = String::from_utf8_lossy(&decoded_content).to_string();

    if let Some(cache_base) = dirs::cache_dir() {
        let readme_cache_dir = cache_base.join("starnest").join("readme");
        let readme_file = readme_cache_dir.join(&cache_key);

        if let Some(parent) = readme_file.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        std::fs::write(&readme_file, &content_str).ok();
    }

    Ok(content_str)
}

/// 生成 README 缓存键
fn generate_readme_cache_key(owner: &str, repo_name: &str) -> String {
    let input = format!("{}/{}", owner, repo_name);
    compute_sha256_hash(&input)
}

/// 获取仓库所有活动事件（使用 Link header 翻页，限制为一个月内）
pub async fn get_all_repo_activities(
    owner: &str,
    repo_name: &str,
) -> Result<Vec<crate::models::dto::GitHubActivity>, String> {
    // 计算一个月前的时间，用于判断是否停止采集
    let one_month_ago = Utc::now() - Duration::days(30);
    let one_month_ago_str = one_month_ago.to_rfc3339();

    let config = load_config().ok();
    let token = config
        .as_ref()
        .and_then(|c| c.auth.as_ref().map(|a| a.access_token.clone()));

    let client = Client::new();
    let mut all_activities = Vec::new();

    let mut url = format!(
        "https://api.github.com/repos/{}/{}/events?page={}&per_page=100",
        owner, repo_name, 1
    );

    // 允许的事件类型
    let allowed_types = ["PushEvent", "ReleaseEvent", "CreateEvent", "DeleteEvent"];

    loop {
        let mut request = client.get(&url).header("User-Agent", "GitHub-Star-Manager");

        // 只有在有 token 的情况下才添加 Authorization 头
        if let Some(t) = &token {
            request = request.header("Authorization", format!("Bearer {}", t));
        }

        let response = request
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {:?}", e))?;

        if !response.status().is_success() {
            return Err(format!("GitHub API returned error: {}", response.status()));
        }

        // 解析 Link header 获取下一页 URL
        let link_header = response.headers().get(reqwest::header::LINK).cloned();
        let next_url: Option<String> = if let Some(link) = link_header {
            if let Ok(link_str) = link.to_str() {
                parse_link_url(link_str, "next")
            } else {
                None
            }
        } else {
            None
        };

        let activities: Vec<crate::models::dto::GitHubActivity> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {:?}", e))?;

        if activities.is_empty() {
            break;
        }

        // 过滤事件类型和时间
        let mut has_newer_events = false;
        for activity in activities {
            // 过滤事件类型
            if !allowed_types.contains(&activity.activity_type.as_str()) {
                continue;
            }

            // 过滤早于一个月前的事件
            if activity.created_at.as_str().cmp(one_month_ago_str.as_str())
                != std::cmp::Ordering::Less
            {
                all_activities.push(activity);
                has_newer_events = true;
            }
        }

        // 如果当前页所有事件都早于一个月前，停止采集
        if !has_newer_events {
            break;
        }

        // 获取下一页 URL，如果没有则停止
        match next_url {
            Some(next) => {
                url = next;
            }
            None => {
                break;
            }
        }
    }

    Ok(all_activities)
}

/// 同步仓库的完整流程
pub async fn sync_repos<R: Runtime>(
    token: &str,
    app_handle: &AppHandle<R>,
) -> Result<RepoSyncResult, String> {
    let cancel_flag = Arc::new(AtomicBool::new(false));
    let cancel_flag_clone = Arc::clone(&cancel_flag);

    let _unlisten = app_handle.listen("cancel_sync", move |_| {
        cancel_flag_clone.store(true, Ordering::Relaxed);
    });

    let _pool =
        crate::db::sqlite::get_pool().map_err(|e| format!("Failed to get database pool: {}", e))?;

    let (repos, total_count) = get_starred_repos(token, app_handle, &cancel_flag).await?;

    if cancel_flag.load(Ordering::Relaxed) {
        return Err("Sync cancelled".to_string());
    }

    let (synced_count, updated_count, new_count, topics) =
        crate::repos::repo_repo::sync_repos_to_db(repos)
            .await
            .map_err(|e| format!("Failed to sync to DB: {:?}", e))?;

    let last_sync = Utc::now().to_rfc3339();

    Ok(RepoSyncResult {
        success: true,
        message: "Sync completed successfully".to_string(),
        synced_count,
        updated_count,
        new_count,
        last_sync,
        topics,
        total_count: total_count.into(),
    })
}

/// 为仓库添加标星
///
/// # 参数
/// - `owner`: 仓库所有者
/// - `repo_name`: 仓库名称
///
/// # 返回值
/// 成功返回 ()，失败返回错误信息
pub async fn star_repo(owner: &str, repo_name: &str) -> Result<(), String> {
    let config = load_config().ok();
    let token = config
        .as_ref()
        .and_then(|c| c.auth.as_ref().map(|a| a.access_token.clone()));

    if token.is_none() {
        return Err("No authentication token found".to_string());
    }

    let client = Client::new();
    let url = format!(
        "https://api.github.com/user/starred/{}/{}",
        owner, repo_name
    );

    let response = client
        .put(&url)
        .header("Authorization", format!("Bearer {}", token.unwrap()))
        .header("User-Agent", "GitHub-Star-Manager")
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {:?}", e))?;

    if !response.status().is_success() {
        return Err(format!("GitHub API returned error: {}", response.status()));
    }

    Ok(())
}

/// 取消仓库标星
///
/// # 参数
/// - `owner`: 仓库所有者
/// - `repo_name`: 仓库名称
///
/// # 返回值
/// 成功返回 ()，失败返回错误信息
pub async fn unstar_repo(owner: &str, repo_name: &str) -> Result<(), String> {
    let config = load_config().ok();
    let token = config
        .as_ref()
        .and_then(|c| c.auth.as_ref().map(|a| a.access_token.clone()));

    if token.is_none() {
        return Err("No authentication token found".to_string());
    }

    let client = Client::new();
    let url = format!(
        "https://api.github.com/user/starred/{}/{}",
        owner, repo_name
    );

    let response = client
        .delete(&url)
        .header("Authorization", format!("Bearer {}", token.unwrap()))
        .header("User-Agent", "GitHub-Star-Manager")
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {:?}", e))?;

    if !response.status().is_success() {
        return Err(format!("GitHub API returned error: {}", response.status()));
    }

    Ok(())
}
