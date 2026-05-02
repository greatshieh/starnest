/**
 * 仓库相关 API 类型定义
 * @module api/repo
 */

/**
 * 仓库信息
 */
export interface Repository {
    id: number
    github_id: number
    full_name: string
    name: string
    description: string | null
    stargazers_count: number
    forks_count: number
    language: string | null
    pushed_at: string
    created_at: string
    html_url: string
    clone_url: string
    homepage: string | null
    open_issues_count: number
    license: string | null
    starred_at: string
    owner_login: string
    owner_avatar_url: string
    learning_status: string
    is_favorite: boolean
    open_pr: number
    total_pr: number
    topics: string[]
    categories: Array<{ id: number; name: string; color: string }>
    status: string
    archived: boolean
}

/**
 * 过滤器项
 */
export interface FilterItem {
    name: string
    count: number
}

/**
 * 仓库过滤器
 */
export interface RepoFilters {
    languages: string[]
    categories: string[]
}

/**
 * 获取仓库列表请求参数
 */
export interface GetReposRequest {
    page: number
    page_size: number
    sort?: string
    language?: string
    categories?: string[]
}

/**
 * 获取仓库列表响应
 */
export interface GetReposResponse {
    repos: Repository[]
    total: number
    page: number
    page_size: number
}

/**
 * 搜索仓库请求参数
 */
export interface SearchReposRequest {
    query: string
    page: number
}

/**
 * 搜索仓库响应
 */
export interface SearchReposResponse {
    repos: Repository[]
    total: number
    page: number
}

/**
 * 获取 README 请求参数
 */
export interface GetReadmeRequest {
    owner: string
    repo_name: string
}

/**
 * 同步仓库请求参数
 */
export interface SyncReposRequest {
    token: string
}

/**
 * 获取仓库事件请求参数
 */
export interface GetRepoEventsByDateRequest {
    repo_id: number
}

/**
 * 获取仓库活动请求参数
 */
export interface FetchRepoActivitiesRequest {
    owner: string
    repo_name: string
}

/**
 * 标星/取消标星请求参数
 */
export interface StarRepoRequest {
    owner: string
    repo_name: string
}

/**
 * 软删除仓库请求参数
 */
export interface SoftDeleteRepoRequest {
    owner: string
    repo_name: string
}

/**
 * 恢复仓库请求参数
 */
export interface RestoreRepoRequest {
    owner: string
    repo_name: string
}

/**
 * 获取仓库过滤器请求参数
 */
export interface GetRepoFiltersRequest {
    deleted?: boolean
    categories?: string[]
    language?: string
}

/**
 * 同步仓库响应
 */
export interface SyncReposResponse {
    synced_count: number
    updated_count: number
    new_count: number
    last_sync: string
    topics: string[]
    total_count: number
}

/**
 * 事件类型统计
 */
export interface EventTypeCount {
    type: string
    count: number
}

/**
 * 每日事件统计
 */
export interface DailyEventStats {
    date: string
    events: EventTypeCount[]
}

/**
 * 获取按日期聚合的事件统计响应
 */
export interface GetRepoEventsByDateResponse {
    date: string
    events: EventTypeCount[]
}

/**
 * 获取状态分类仓库响应
 */
export type RepoStatusCategoriesResponse = Repository[]

/**
 * 获取仓库统计信息响应
 */
export interface RepoStatsResponse {
    total_repos: number
    category_count: number
    uncategorized_count: number
    recent_starred_count: number
}
