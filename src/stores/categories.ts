import { defineStore } from 'pinia'
import { ref } from 'vue'
import * as categoryApi from '@/api/category'
import type { Category, Operation, SortBy, SortOrder } from '@/types/api/category'
import { SortBy as SortByEnum, SortOrder as SortOrderEnum, OperationType } from '@/types/api/category'

const categoryColors = [
    '#9333ea', '#60a5fa', '#facc15', '#ef4444', '#22c55e',
    '#f97316', '#ec4899', '#06b6d4', '#8b5cf6', '#10b981',
]

export const useCategoriesStore = defineStore('categories', () => {
    const categories = ref<Category[]>([])
    const operations = ref<Operation[]>([])
    const isLoading = ref(false)
    const isLoaded = ref(false)
    const total = ref(0)
    const currentPage = ref(1)
    const pageSize = ref(20)
    const totalPages = ref(0)
    const searchKeyword = ref('')
    const sortBy = ref<SortBy>(SortByEnum.COUNT)
    const sortOrder = ref<SortOrder>(SortOrderEnum.DESC)

    function getRandomColor(): string {
        const index = Math.floor(Math.random() * categoryColors.length)
        return categoryColors[index]
    }

    async function fetchCategories(
        page?: number,
        keyword?: string,
        sortField?: SortBy,
        sortDir?: SortOrder,
    ): Promise<void> {
        isLoading.value = true
        try {
            const reqPage = page ?? currentPage.value
            const reqKeyword = keyword ?? searchKeyword.value
            const reqSortBy = sortField ?? sortBy.value
            const reqSortOrder = sortDir ?? sortOrder.value

            const result = await categoryApi.getCategoriesPaged({
                page: reqPage,
                page_size: pageSize.value,
                search_keyword: reqKeyword || undefined,
                sort_by: reqSortBy,
                sort_order: reqSortOrder,
            })

            categories.value = result.categories.map(c => ({
                id: c.id,
                name: c.name,
                color: c.color,
                repoCount: c.repo_count,
                updatedAt: c.updated_at,
                selected: false,
            }))

            total.value = result.total
            currentPage.value = result.page
            totalPages.value = result.total_pages
            searchKeyword.value = reqKeyword
            sortBy.value = reqSortBy
            sortOrder.value = reqSortOrder
            isLoaded.value = true
        } catch (error) {
            console.error('Failed to fetch categories:', error)
        } finally {
            isLoading.value = false
        }
    }

    async function fetchAllCategories(): Promise<void> {
        if (isLoaded.value && categories.value.length > 0) return

        isLoading.value = true
        try {
            const result = await categoryApi.getCategories()
            categories.value = result.map(c => ({
                id: c.id,
                name: c.name,
                color: c.color,
                repoCount: c.repo_count,
                updatedAt: c.updated_at,
                selected: false,
            }))
            isLoaded.value = true
        } catch (error) {
            console.error('Failed to fetch categories:', error)
        } finally {
            isLoading.value = false
        }
    }

    function setSelectedCategories(categoryIds: number[]): void {
        categories.value.forEach(category => {
            category.selected = categoryIds.includes(category.id)
        })
    }

    function toggleCategory(categoryId: number): void {
        const category = categories.value.find(c => c.id === categoryId)
        if (category) {
            category.selected = !category.selected

            operations.value.push({
                type: OperationType.TOGGLE,
                categoryId,
            })
        }
    }

    async function addCategory(name: string, color?: string): Promise<void> {
        try {
            const newCat = await categoryApi.createCategory({ name, color: color || getRandomColor() })
            const category: Category = {
                id: newCat.id,
                name: newCat.name,
                color: newCat.color,
                repoCount: 0,
                updatedAt: newCat.updated_at,
                selected: true,
            }
            categories.value.unshift(category)

            operations.value.push({
                type: OperationType.ADD,
                category: {
                    name: category.name,
                    color: category.color,
                    repoCount: category.repoCount,
                    updatedAt: category.updatedAt,
                    selected: category.selected,
                },
            })

            total.value += 1
        } catch (error) {
            console.error('Failed to add category:', error)
            throw error
        }
    }

    async function deleteCategory(categoryId: number): Promise<void> {
        try {
            await categoryApi.deleteCategory({ category_id: categoryId })
            const index = categories.value.findIndex(c => c.id === categoryId)
            if (index !== -1) {
                categories.value.splice(index, 1)
            }

            operations.value.push({
                type: OperationType.DELETE,
                categoryId,
            })

            total.value -= 1
        } catch (error) {
            console.error('Failed to delete category:', error)
            throw error
        }
    }

    async function updateCategoryInfo(categoryId: number, name?: string, color?: string): Promise<void> {
        try {
            const result = await categoryApi.updateCategory({
                category_id: categoryId,
                name,
                color,
            })

            const category = categories.value.find(c => c.id === categoryId)
            if (category) {
                if (name) category.name = name
                if (color) category.color = color
                category.updatedAt = result.updated_at
            }
        } catch (error) {
            console.error('Failed to update category:', error)
            throw error
        }
    }

    function updateCategory(categoryId: number, updates: Partial<Category>): void {
        const category = categories.value.find(c => c.id === categoryId)
        if (category) {
            Object.assign(category, updates)
        }
    }

    async function saveChanges(repoId: number): Promise<void> {
        if (operations.value.length === 0) return

        const selectedCategoryIds = categories.value.filter(c => c.selected).map(c => c.id)

        try {
            await categoryApi.updateRepoCategories({
                repo_id: repoId,
                category_ids: selectedCategoryIds,
            })
            operations.value = []
        } catch (error) {
            console.error('Failed to save category changes:', error)
            throw error
        }
    }

    function clearOperations(): void {
        operations.value = []
    }

    function reset(): void {
        categories.value = []
        operations.value = []
        isLoaded.value = false
        total.value = 0
        currentPage.value = 1
        totalPages.value = 0
        searchKeyword.value = ''
        sortBy.value = SortByEnum.COUNT
        sortOrder.value = SortOrderEnum.DESC
    }

    function setPage(page: number): void {
        if (page >= 1 && page <= totalPages.value) {
            currentPage.value = page
        }
    }

    function setSearchKeyword(keyword: string): void {
        searchKeyword.value = keyword
        currentPage.value = 1
    }

    function setSort(field: SortBy, order: SortOrder): void {
        sortBy.value = field
        sortOrder.value = order
        currentPage.value = 1
    }

    return {
        categories,
        operations,
        isLoading,
        isLoaded,
        total,
        currentPage,
        pageSize,
        totalPages,
        searchKeyword,
        sortBy,
        sortOrder,
        fetchCategories,
        fetchAllCategories,
        setSelectedCategories,
        toggleCategory,
        addCategory,
        deleteCategory,
        updateCategoryInfo,
        updateCategory,
        saveChanges,
        clearOperations,
        reset,
        getRandomColor,
        setPage,
        setSearchKeyword,
        setSort,
    }
})