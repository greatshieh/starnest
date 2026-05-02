import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import MarkdownIt from 'markdown-it'
import { api } from '@/api'
import type { api as ApiTypes } from '@/types'
import { parseRepoTopics } from '@/utils/format'
import { useMessage } from '@/composables/useMessage'

export type Repository = ApiTypes.repo.Repository

interface PageCacheItem {
    repos: ApiTypes.repo.Repository[]
    timestamp: number
}

export const useReposStore = defineStore('repos', () => {
    const mdrender = new MarkdownIt({
        html: true,
    })
    const message = useMessage()

    const repositories = ref<ApiTypes.repo.Repository[]>([])
    const currentPage = ref(1)
    const pageSize = ref(30)
    const totalCount = ref(0)

    const stats = ref<ApiTypes.repo.RepoStatsResponse>({
        total_repos: 0,
        category_count: 0,
        uncategorized_count: 0,
        recent_starred_count: 0,
    })

    const isLoadingMore = ref(false)
    const hasMore = ref(true)
    const currentSort = ref<'recent_stars' | 'star_count' | 'fork_count' | 'update_time'>('star_count')
    const sortOrderState = ref<'asc' | 'desc'>('desc')
    const selectedLanguage = ref<string>('')
    const selectedCategory = ref<string[]>([])
    const filters = ref<ApiTypes.repo.RepoFilters>({
        languages: [],
        categories: [],
    })

    const selectedRepo = ref<ApiTypes.repo.Repository | null>(null)
    const repoReadme = ref<string | null>(null)
    const isLoadingReadme = ref(false)

    const pageCache = ref<Map<string, PageCacheItem>>(new Map())
    const cacheSize = 5

    const totalPages = computed(() => Math.ceil(totalCount.value / pageSize.value))

    function cleanupCache() {
        if (pageCache.value.size > cacheSize) {
            const sortedPages = Array.from(pageCache.value.entries()).sort((a, b) => a[1].timestamp - b[1].timestamp)
            const pagesToRemove = sortedPages.slice(0, sortedPages.length - cacheSize)
            pagesToRemove.forEach(([page]) => pageCache.value.delete(page))
        }
    }

    async function fetchRepos(
        page: number = 1,
        sortBy?: string,
        language?: string,
        categories?: string[],
        order?: 'asc' | 'desc',
    ): Promise<void> {
        try {
            currentSort.value = sortBy as typeof currentSort.value
            if (language !== undefined) selectedLanguage.value = language
            if (categories !== undefined) selectedCategory.value = categories
            if (order !== undefined) sortOrderState.value = order

            const categoryStr = selectedCategory.value.join(',')
            const cacheKey = `${page}-${currentSort.value}-${selectedLanguage.value}-${categoryStr}-${sortOrderState.value}`
            if (pageCache.value.has(cacheKey)) {
                const cachedItem = pageCache.value.get(cacheKey)!
                repositories.value = cachedItem.repos
                currentPage.value = page
                cachedItem.timestamp = Date.now()
                return
            }

            const request: any = {
                page,
                page_size: pageSize.value,
                sort: currentSort.value,
                sort_order: sortOrderState.value,
            }
            if (selectedLanguage.value) {
                request.language = selectedLanguage.value
            }
            if (selectedCategory.value.length > 0) {
                request.categories = selectedCategory.value
            }
            const response = await api.repo.getRepos(request)
            const reposWithParsedTopics = response.repos.map(parseRepoTopics)

            pageCache.value.set(cacheKey, {
                repos: reposWithParsedTopics,
                timestamp: Date.now(),
            })
            cleanupCache()

            if (page === 1) {
                repositories.value = reposWithParsedTopics
            } else {
                repositories.value.push(...reposWithParsedTopics)
            }

            totalCount.value = response.total
            currentPage.value = response.page
            hasMore.value = response.repos.length === pageSize.value && currentPage.value < totalPages.value
        } catch (error) {
            message?.error('Failed to fetch repos')
            console.error('Failed to fetch repos:', error)
        }
    }

    async function loadMoreRepos(): Promise<void> {
        if (isLoadingMore.value || !hasMore.value) return

        isLoadingMore.value = true
        const nextPage = currentPage.value + 1

        try {
            const categoryStr = selectedCategory.value.join(',')
            const cacheKey = `${nextPage}-${currentSort.value}-${selectedLanguage.value}-${categoryStr}-${sortOrderState.value}`
            if (pageCache.value.has(cacheKey)) {
                const cachedItem = pageCache.value.get(cacheKey)!
                repositories.value.push(...cachedItem.repos)
                currentPage.value = nextPage
                cachedItem.timestamp = Date.now()
                hasMore.value = cachedItem.repos.length === pageSize.value && nextPage < totalPages.value
                return
            }

            const request: any = {
                page: nextPage,
                page_size: pageSize.value,
                sort: currentSort.value,
                sort_order: sortOrderState.value,
            }
            if (selectedLanguage.value) {
                request.language = selectedLanguage.value
            }
            if (selectedCategory.value.length > 0) {
                request.categories = selectedCategory.value
            }
            const response = await api.repo.getRepos(request)

            const reposWithParsedTopics = response.repos.map(parseRepoTopics)

            pageCache.value.set(cacheKey, {
                repos: reposWithParsedTopics,
                timestamp: Date.now(),
            })
            cleanupCache()

            repositories.value.push(...reposWithParsedTopics)

            totalCount.value = response.total
            currentPage.value = response.page
            hasMore.value = response.repos.length === pageSize.value && response.page < totalPages.value
        } catch (error) {
            message?.error('Failed to load more repos')
            console.error('Failed to load more repos:', error)
        } finally {
            isLoadingMore.value = false
        }
    }

    function resetPagination(): void {
        currentPage.value = 1
        totalCount.value = 0
        hasMore.value = true
        repositories.value = []
        pageCache.value.clear()
    }

    function selectRepo(repo: ApiTypes.repo.Repository): void {
        selectedRepo.value = repo
    }

    async function fetchRepoReadme(owner: string, repoName: string): Promise<void> {
        isLoadingReadme.value = true
        repoReadme.value = null

        try {
            const result = await api.repo.getRepoReadme({ owner, repo_name: repoName })
            repoReadme.value = mdrender.render(result)
        } catch (error) {
            console.error('Failed to fetch README:', error)
            repoReadme.value = null
        } finally {
            isLoadingReadme.value = false
        }
    }

    function clearReadme(): void {
        repoReadme.value = null
    }

    async function fetchFilters(categories?: string[], language?: string): Promise<void> {
        try {
            const request: any = {
                deleted: false,
            }
            if (language) {
                request.language = language
            }
            if (categories && categories.length > 0) {
                request.categories = categories
            }
            const response = await api.repo.getRepoFilters(request)
            filters.value = response
        } catch (error) {
            console.error('Failed to fetch filters:', error)
        }
    }

    async function fetchRepoStats(): Promise<void> {
        try {
            const result = await api.repo.getRepoStats()
            stats.value = result
            totalCount.value = result.total_repos
        } catch (error) {
            console.error('Failed to fetch repo stats:', error)
        }
    }

    async function searchRepos(
        query: string,
        page: number = 1,
    ): Promise<{
        repos: ApiTypes.repo.Repository[]
        total: number
        page: number
    }> {
        if (!query.trim()) {
            return {
                repos: repositories.value,
                total: repositories.value.length,
                page: 1,
            }
        }

        try {
            const response = await api.repo.searchRepos({ query, page })
            const reposWithParsedTopics = response.repos.map(parseRepoTopics)

            return {
                ...response,
                repos: reposWithParsedTopics,
            }
        } catch (error) {
            message?.error('Failed to search repos')
            console.error('Failed to search repos:', error)
            return {
                repos: [],
                total: 0,
                page,
            }
        }
    }

    function removeRepo(owner: string, repoName: string): void {
        const index = repositories.value.findIndex(repo => repo.owner_login === owner && repo.name === repoName)
        if (index !== -1) {
            repositories.value.splice(index, 1)
            totalCount.value = Math.max(0, totalCount.value - 1)
        }
    }

    function cleanupData(): void {
        repositories.value = []
        currentPage.value = 1
        totalCount.value = 0
        hasMore.value = true
        pageCache.value.clear()
        selectedRepo.value = null
        repoReadme.value = null
    }

    return {
        repositories,
        currentPage,
        pageSize,
        totalCount,
        totalPages,
        isLoadingMore,
        hasMore,
        currentSort,
        selectedLanguage,
        selectedCategory,
        sortOrderState,
        filters,
        selectedRepo,
        repoReadme,
        isLoadingReadme,
        fetchRepos,
        loadMoreRepos,
        resetPagination,
        selectRepo,
        fetchRepoReadme,
        clearReadme,
        fetchFilters,
        fetchRepoStats,
        stats,
        searchRepos,
        cleanupData,
        parseRepoTopics,
        removeRepo,
    }
})
