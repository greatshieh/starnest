/**
 * 分类相关 API 类型定义
 * @module api/category
 */

export enum SortBy {
    COUNT = 'count',
    NAME = 'name',
    UPDATED_AT = 'updated_at',
}

export enum SortOrder {
    ASC = 'asc',
    DESC = 'desc',
}

export interface Category {
    id: number
    name: string
    color: string
    repoCount: number
    updatedAt: string
    selected: boolean
}

export enum OperationType {
    ADD = 'add',
    DELETE = 'delete',
    TOGGLE = 'toggle',
}

export interface Operation {
    type: OperationType
    categoryId?: number
    category?: Omit<Category, 'id'>
}

/**
 * 分类信息接口（含仓库数量）
 */
export interface CategoryWithCount {
    id: number
    name: string
    color: string
    repo_count: number
    updated_at: string
}

/**
 * 分类分页请求参数
 */
export interface GetCategoriesPageRequest {
    page: number
    page_size: number
    search_keyword?: string
    sort_by: string
    sort_order: string
}

/**
 * 分类分页响应
 */
export interface GetCategoriesPageResponse {
    categories: CategoryWithCount[]
    total: number
    page: number
    page_size: number
    total_pages: number
}

/**
 * 更新分类请求参数
 */
export interface UpdateCategoryRequest {
    category_id: number
    name?: string
    color?: string
}

/**
 * 创建分类请求参数
 */
export interface CreateCategoryRequest {
    name: string
    color: string
}

/**
 * 删除分类请求参数
 */
export interface DeleteCategoryRequest {
    category_id: number
}

/**
 * 更新仓库分类关联请求参数
 */
export interface UpdateRepoCategoriesRequest {
    repo_id: number
    category_ids: number[]
}