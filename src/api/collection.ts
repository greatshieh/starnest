/**
 * 合集相关 API 封装
 * 统一处理合集相关的后端调用
 * @module api/collection
 */

import { invoke } from '@tauri-apps/api/core'
import type { api } from '@/types'

/**
 * 创建合集
 * @param request - 创建请求参数
 * @returns 创建的合集信息
 */
export async function createCollection(
    request: api.collection.CreateCollectionRequest
): Promise<api.collection.CollectionWithRepoCount> {
    const result = await invoke('cmd_create_collection', { ...request })
    return result as api.collection.CollectionWithRepoCount
}

/**
 * 更新合集
 * @param request - 更新请求参数
 * @returns 更新后的合集信息
 */
export async function updateCollection(
    request: api.collection.UpdateCollectionRequest
): Promise<api.collection.CollectionWithRepoCount> {
    const result = await invoke('cmd_update_collection', { request })
    return result as api.collection.CollectionWithRepoCount
}

/**
 * 删除合集
 * @param request - 删除请求参数
 */
export async function deleteCollection(request: { collection_id: number }): Promise<void> {
    await invoke('cmd_delete_collection', { ...request })
}

/**
 * 根据ID获取合集
 * @param request - 请求参数
 * @returns 合集信息（含仓库数量）
 */
export async function getCollectionById(request: { collection_id: number }): Promise<api.collection.CollectionWithRepoCount> {
    const result = await invoke('cmd_get_collection_by_id', { ...request })
    return result as api.collection.CollectionWithRepoCount
}

/**
 * 获取所有合集
 * @returns 合集列表（含仓库数量）
 */
export async function getAllCollections(): Promise<api.collection.CollectionWithRepoCount[]> {
    const result = await invoke('cmd_get_all_collections')
    return result as api.collection.CollectionWithRepoCount[]
}

/**
 * 分页获取合集列表
 * @param request - 分页请求参数
 * @returns 分页结果
 */
export async function getCollectionsPaged(
    request: api.collection.CollectionPageRequest
): Promise<api.collection.CollectionPageResponse> {
    const result = await invoke('cmd_get_collections_paged', { request })
    return result as api.collection.CollectionPageResponse
}

/**
 * 获取合集关联的所有仓库
 * @param request - 请求参数
 * @returns 仓库列表
 */
export async function getReposByCollection(request: { collection_id: number }): Promise<api.repo.Repository[]> {
    const result = await invoke('cmd_get_repos_by_collection', { ...request })
    return result as api.repo.Repository[]
}

/**
 * 获取仓库关联的所有合集
 * @param request - 请求参数
 * @returns 合集列表
 */
export async function getCollectionsByRepo(request: { github_id: number }): Promise<api.collection.Collection[]> {
    const result = await invoke('cmd_get_collections_by_repo', { ...request })
    return result as api.collection.Collection[]
}

/**
 * 更新合集与仓库的关联关系
 * @param request - 更新请求参数
 */
export async function updateCollectionRepos(
    request: api.collection.UpdateCollectionReposRequest
): Promise<void> {
    await invoke('cmd_update_collection_repos', { request })
}

/**
 * 检查仓库是否已在合集中
 * @param request - 请求参数
 * @returns 是否存在关联
 */
export async function isRepoInCollection(request: { github_id: number; collection_id: number }): Promise<boolean> {
    const result = await invoke('cmd_is_repo_in_collection', { ...request })
    return result as boolean
}