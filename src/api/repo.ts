/**
 * 仓库相关 API 封装
 * 统一处理仓库相关的后端调用
 * @module api/repo
 */

import { invoke } from '@tauri-apps/api/core'
import type { api } from '@/types'

/**
 * 获取仓库列表
 * @param request - 请求参数
 * @returns 仓库列表响应
 */
export async function getRepos(
    request: api.repo.GetReposRequest & {
        categories?: string[]
        deleted?: boolean
        sort_order?: 'asc' | 'desc'
    }
): Promise<api.repo.GetReposResponse> {
    const result = await invoke('cmd_get_repos', { request })
    return result as api.repo.GetReposResponse
}

/**
 * 获取仓库过滤器（语言和分类）
 * @param request - 请求参数
 * @returns 过滤器信息
 */
export async function getRepoFilters(
    request: api.repo.GetRepoFiltersRequest
): Promise<api.repo.RepoFilters> {
    const result = await invoke('cmd_get_repo_filters', { request })
    return result as api.repo.RepoFilters
}

/**
 * 搜索仓库
 * @param request - 请求参数
 * @returns 搜索结果
 */
export async function searchRepos(
    request: api.repo.SearchReposRequest & { page_size?: number; deleted?: boolean }
): Promise<api.repo.SearchReposResponse> {
    const result = await invoke('cmd_search_repos', { request })
    return result as api.repo.SearchReposResponse
}

/**
 * 获取仓库 README
 * @param request - 请求参数
 * @returns README 内容（Markdown 格式）
 */
export async function getRepoReadme(request: api.repo.GetReadmeRequest): Promise<string> {
    const result = await invoke('cmd_get_repo_readme', { request })
    return result as string
}

/**
 * 同步仓库
 * @param request - 请求参数
 * @returns 同步结果
 */
export async function syncRepos(request: api.repo.SyncReposRequest): Promise<api.repo.SyncReposResponse> {
    const result = await invoke('cmd_sync_repos', { request })
    return result as api.repo.SyncReposResponse
}

/**
 * 同步 FTS 索引
 * @returns 同步结果信息
 */
export async function syncFts(): Promise<string> {
    const result = await invoke('cmd_sync_fts')
    return result as string
}

/**
 * 获取按日期聚合的事件统计
 * @param request - 请求参数
 * @returns 按日期聚合的事件统计
 */
export async function getRepoEventsByDate(
    request: api.repo.GetRepoEventsByDateRequest
): Promise<api.repo.GetRepoEventsByDateResponse> {
    const result = await invoke('cmd_get_repo_events_by_date', { request })
    return result as api.repo.GetRepoEventsByDateResponse
}

/**
 * 从GitHub获取活动并保存到数据库
 * @param request - 请求参数
 */
export async function fetchRepoActivitiesFromGitHub(
    request: api.repo.FetchRepoActivitiesRequest
): Promise<void> {
    await invoke('cmd_fetch_repo_activities_from_github', { request })
}

/**
 * 为仓库添加标星
 * @param request - 请求参数
 */
export async function starRepo(request: api.repo.StarRepoRequest): Promise<void> {
    await invoke('cmd_star_repo', { request })
}

/**
 * 取消仓库标星
 * @param request - 请求参数
 */
export async function unstarRepo(request: api.repo.StarRepoRequest): Promise<void> {
    await invoke('cmd_unstar_repo', { request })
}

/**
 * 软删除仓库（标记为已取消标星）
 * @param request - 请求参数
 */
export async function softDeleteRepo(request: api.repo.SoftDeleteRepoRequest): Promise<void> {
    await invoke('cmd_soft_delete_repo', { request })
}

/**
 * 恢复已删除的仓库
 * @param request - 请求参数
 */
export async function restoreRepo(request: api.repo.RestoreRepoRequest): Promise<void> {
    await invoke('cmd_restore_repo', { request })
}

/**
 * 获取最近一周更新的仓库
 * @returns 仓库列表
 */
export async function getRecentUpdatedRepos(): Promise<api.repo.Repository[]> {
    const result = await invoke('cmd_get_recent_updated_repos')
    return result as api.repo.Repository[]
}

/**
 * 获取状态分类仓库（归档、低活跃、废弃）
 * @returns 状态分类仓库结果
 */
export async function getRepoStatusCategories(): Promise<api.repo.RepoStatusCategoriesResponse> {
    const result = await invoke('cmd_get_repo_status_categories')
    return result as api.repo.RepoStatusCategoriesResponse
}

/**
 * 获取仓库统计信息
 * @returns 统计信息
 */
export async function getRepoStats(): Promise<api.repo.RepoStatsResponse> {
    const result = await invoke('cmd_get_repo_stats')
    return result as api.repo.RepoStatsResponse
}