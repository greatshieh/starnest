/**
 * 合集相关 API 类型定义
 * @module api/collection
 */

/**
 * 合集信息
 */
export interface Collection {
    id: number
    name: string
    description: string
    color: string
    created_at: string
    updated_at: string
}

/**
 * 带仓库数量的合集信息
 */
export interface CollectionWithRepoCount extends Collection {
    repo_count: number
}

/**
 * 创建合集请求参数
 */
export interface CreateCollectionRequest {
    name: string
    description: string
    color: string
}

/**
 * 更新合集请求参数
 */
export interface UpdateCollectionRequest {
    collection_id: number
    name?: string
    description?: string
    color?: string
}

/**
 * 更新合集仓库关联请求参数
 */
export interface UpdateCollectionReposRequest {
    collection_id: number
    github_ids: number[]
}

/**
 * 分页查询请求参数
 */
export interface CollectionPageRequest {
    keyword?: string
    page: number
    page_size: number
    sort_by?: string
    sort_order?: string
}

/**
 * 分页查询响应
 */
export interface CollectionPageResponse {
    collections: CollectionWithRepoCount[]
    total: number
    page: number
    page_size: number
    total_pages: number
}

/**
 * 合集详情（含仓库列表）
 */
export interface CollectionDetail extends Collection {
    repos: Array<{
        id: number
        github_id: number
        full_name: string
        name: string
        language: string | null
        stargazers_count: number
    }>
}
