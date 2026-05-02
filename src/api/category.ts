/**
 * 分类相关 API 封装模块
 *
 * 统一处理分类相关的后端调用，提供分类的增删改查以及仓库分类关联管理功能。
 * 通过 Tauri IPC 与 Rust 后端进行通信。
 *
 * @module api/category
 */

import { invoke } from '@tauri-apps/api/core'
import type { api } from '@/types'

/**
 * 获取所有分类及其关联的仓库数量
 *
 * @returns {Promise<api.category.CategoryWithCount[]>} 分类列表
 */
export async function getCategories(): Promise<api.category.CategoryWithCount[]> {
    const result = await invoke('cmd_get_categories')
    return result as api.category.CategoryWithCount[]
}

/**
 * 分页获取分类列表
 *
 * @param {api.category.GetCategoriesPageRequest} request - 分页请求参数
 * @returns {Promise<api.category.GetCategoriesPageResponse>} 分页响应
 */
export async function getCategoriesPaged(
    request: api.category.GetCategoriesPageRequest
): Promise<api.category.GetCategoriesPageResponse> {
    const result = await invoke('cmd_get_categories_paged', { request })
    return result as api.category.GetCategoriesPageResponse
}

/**
 * 更新分类信息
 *
 * @param {api.category.UpdateCategoryRequest} request - 更新请求参数
 * @returns {Promise<api.category.CategoryWithCount>} 更新后的分类信息
 */
export async function updateCategory(
    request: api.category.UpdateCategoryRequest
): Promise<api.category.CategoryWithCount> {
    const result = await invoke('cmd_update_category', { request })
    return result as api.category.CategoryWithCount
}

/**
 * 创建新分类
 *
 * @param {api.category.CreateCategoryRequest} request - 创建请求参数
 * @returns {Promise<api.category.CategoryWithCount>} 创建成功的分类信息
 */
export async function createCategory(
    request: api.category.CreateCategoryRequest
): Promise<api.category.CategoryWithCount> {
    const result = await invoke('cmd_create_category', { request })
    return result as api.category.CategoryWithCount
}

/**
 * 删除分类
 *
 * @param {api.category.DeleteCategoryRequest} request - 删除请求参数
 * @returns {Promise<void>}
 */
export async function deleteCategory(request: api.category.DeleteCategoryRequest): Promise<void> {
    await invoke('cmd_delete_category', { category_id: request.category_id })
}

/**
 * 为仓库更新分类关联
 *
 * 根据仓库ID和分类ID列表，更新仓库与分类的关联关系。
 * 会删除不再关联的分类，并添加新的分类关联。
 *
 * @param {api.category.UpdateRepoCategoriesRequest} request - 更新请求参数
 * @returns {Promise<void>}
 */
export async function updateRepoCategories(
    request: api.category.UpdateRepoCategoriesRequest
): Promise<void> {
    await invoke('cmd_update_repo_categories', { request })
}