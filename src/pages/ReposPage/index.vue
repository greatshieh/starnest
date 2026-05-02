<script setup lang="ts">
import { useRouter } from 'vue-router'
import { onMounted, ref, onUnmounted, onBeforeUnmount, watch, computed, nextTick } from 'vue'
import { useReposStore, type Repository } from '@/stores/repos'
import { SelectOption } from '@/components/MySelect'
import { useAuthStore } from '@/stores/auth'
import { useAppStore } from '@/stores/apps'

const router = useRouter()
const reposStore = useReposStore()
const authStore = useAuthStore()
const appStore = useAppStore()

const observerRef = ref<IntersectionObserver | null>(null)
const loadingTriggerRef = ref<HTMLDivElement | null>(null)

const searchQuery = ref('')
const isLoading = ref(false)
const isSearching = ref(false)
const debouncedSearchQuery = ref('')
const searchResults = ref<Repository[]>([])
const searchTotal = ref(0)
const searchCurrentPage = ref(1)
const searchPageSize = ref(30)
const hasMoreSearchResults = ref(false)

let searchTimeout: ReturnType<typeof setTimeout> | null = null

const resetSearch = () => {
    searchResults.value = []
    searchTotal.value = 0
    debouncedSearchQuery.value = ''
    searchCurrentPage.value = 1
    hasMoreSearchResults.value = false
}

const loadMoreSearchResults = async () => {
    if (isSearching.value || !hasMoreSearchResults.value) return

    isSearching.value = true
    searchCurrentPage.value += 1

    try {
        const result = await reposStore.searchRepos(searchQuery.value, searchCurrentPage.value)
        searchResults.value.push(...result.repos)
        hasMoreSearchResults.value = searchCurrentPage.value * searchPageSize.value < searchTotal.value
    } catch (error) {
        console.error('Load more search results error:', error)
    } finally {
        isSearching.value = false
    }
}

type SortOption = 'recent_stars' | 'star_count' | 'fork_count' | 'update_time'

const sortOptions = [
    { value: 'star_count', label: 'Sort by Star Count' },
    { value: 'fork_count', label: 'Sort by Fork Count' },
    { value: 'recent_stars', label: 'Sort by Recent Stars' },
    { value: 'update_time', label: 'Sort by Update Time' },
] as Array<SelectOption>

// 使用 store 中的状态来保持筛选和排序的持久化
const selectedSort = computed({
    get: () => reposStore.currentSort as SortOption,
    set: val => {
        reposStore.currentSort = val
    },
})
const selectedLanguage = computed({
    get: () => reposStore.selectedLanguage,
    set: val => {
        reposStore.selectedLanguage = val
    },
})
const selectedCategory = computed({
    get: () => reposStore.selectedCategory,
    set: val => {
        reposStore.selectedCategory = val
    },
})
const sortOrder = computed({
    get: () => reposStore.sortOrderState as 'asc' | 'desc',
    set: val => {
        reposStore.sortOrderState = val
    },
})

const languageOptions = computed(() => {
    const options: Array<SelectOption> = [{ value: '', label: 'All Languages' }]
    reposStore.filters.languages.forEach(lang => {
        options.push({ value: lang, label: lang })
    })
    return options
})

const categoryOptions = computed(() => {
    const options: Array<SelectOption> = []
    reposStore.filters.categories.forEach(cat => {
        options.push({ value: cat, label: cat })
    })
    return options
})

const handleSortChange = async () => {
    resetSearch()
    isLoading.value = true
    reposStore.resetPagination()
    await reposStore.fetchRepos(1, selectedSort.value, selectedLanguage.value, selectedCategory.value, sortOrder.value)
    isLoading.value = false
    reinitIntersectionObserver()
}

const toggleSortOrder = () => {
    sortOrder.value = sortOrder.value === 'desc' ? 'asc' : 'desc'
    handleSortChange()
}

const handleLanguageChange = async () => {
    resetSearch()
    isLoading.value = true
    reposStore.resetPagination()
    await reposStore.fetchRepos(1, selectedSort.value, selectedLanguage.value, selectedCategory.value, sortOrder.value)
    isLoading.value = false
    reinitIntersectionObserver()
}

const handleCategoryChange = async () => {
    resetSearch()
    isLoading.value = true
    reposStore.resetPagination()
    await reposStore.fetchRepos(1, selectedSort.value, selectedLanguage.value, selectedCategory.value, sortOrder.value)
    isLoading.value = false
    reinitIntersectionObserver()
}

const reinitIntersectionObserver = () => {
    if (observerRef.value) {
        observerRef.value.disconnect()
    }
    nextTick(() => {
        initIntersectionObserver()
    })
}

watch(() => selectedSort.value, handleSortChange)
watch(() => selectedLanguage.value, handleLanguageChange)
watch(() => selectedCategory.value, handleCategoryChange)

const handleRepoClick = (item: Repository) => {
    const [owner, name] = item.full_name.split('/')
    router.push({
        name: 'RepoDetail',
        params: { owner, name },
        state: {
            repo: JSON.stringify(item),
        },
    })
}

const handleLoadMore = async () => {
    if (debouncedSearchQuery.value.trim()) {
        await loadMoreSearchResults()
    } else {
        if (reposStore.isLoadingMore || !reposStore.hasMore) return

        if (observerRef.value && loadingTriggerRef.value) {
            observerRef.value.unobserve(loadingTriggerRef.value)
        }

        await reposStore.loadMoreRepos()

        if (observerRef.value && loadingTriggerRef.value && reposStore.hasMore) {
            observerRef.value.observe(loadingTriggerRef.value)
        }
    }
}

const initIntersectionObserver = () => {
    observerRef.value = new IntersectionObserver(
        entries => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    handleLoadMore()
                }
            })
        },
        {
            rootMargin: '200px',
            threshold: 0.1,
        },
    )

    if (loadingTriggerRef.value) {
        observerRef.value.observe(loadingTriggerRef.value)
    }
}

// 滚动容器引用
const scrollContainerRef = ref<HTMLElement | null>(null)

// 保存的滚动位置（临时存储）
const savedScrollTop = ref(0)

// 生成滚动位置的缓存key（包含筛选条件）
const getScrollCacheKey = () => {
    const categoryStr = selectedCategory.value.join(',')
    return `repoPageScroll_${selectedSort.value}_${selectedLanguage.value}_${categoryStr}_${sortOrder.value}`
}

// 保存滚动位置
const saveScrollPosition = () => {
    const key = getScrollCacheKey()
    if (scrollContainerRef.value) {
        savedScrollTop.value = scrollContainerRef.value.scrollTop
        sessionStorage.setItem(key, scrollContainerRef.value.scrollTop.toString())
    } else {
        // 备选：使用 window.scrollY
        savedScrollTop.value = window.scrollY
        sessionStorage.setItem(key, window.scrollY.toString())
    }
}

// 恢复滚动位置
const restoreScrollPosition = () => {
    const key = getScrollCacheKey()
    const savedScroll = sessionStorage.getItem(key) || savedScrollTop.value.toString()
    if (savedScroll) {
        const scrollValue = parseInt(savedScroll)
        if (scrollContainerRef.value) {
            scrollContainerRef.value.scrollTop = scrollValue
        } else {
            window.scrollTo(0, scrollValue)
        }
    }
}

onMounted(async () => {
    if (authStore.isLoggedIn) {
        await reposStore.fetchFilters()

        // 检查是否是从详情页返回（已有数据）
        const hasExistingData = reposStore.repositories.length > 0

        // 如果没有数据，加载第一页
        if (!hasExistingData) {
            await reposStore.fetchRepos(
                1,
                selectedSort.value,
                selectedLanguage.value,
                selectedCategory.value,
                sortOrder.value,
            )
        }

        // 等待DOM完全渲染后恢复滚动位置
        // 使用 setTimeout 确保所有异步操作完成
        setTimeout(() => {
            restoreScrollPosition()
        }, 100)
    }

    initIntersectionObserver()
})

// 在组件卸载前保存滚动位置（beforeUnmount 比 onUnmounted 更早执行，ref 还没有变为 null）
onBeforeUnmount(() => {
    saveScrollPosition()
})

onUnmounted(() => {
    if (observerRef.value) {
        observerRef.value.disconnect()
    }
    if (searchTimeout) {
        clearTimeout(searchTimeout)
    }
})
</script>

<template>
    <div ref="scrollContainerRef" class="overflow-y-auto h-full">
        <!-- 工具栏 -->
        <div class="p-6 mb-6 flex items-center justify-between shadow-md">
            <div>
                <h2 class="text-3xl text-t-primary font-semibold tracking-tight">All Stars</h2>
                <p class="mt-4">{{ reposStore.totalCount }} repos</p>
            </div>
            <div class="flex items-center gap-2">
                <!-- 分类筛选 -->
                <MySelect
                    v-model="selectedCategory"
                    :options="categoryOptions"
                    class="!w-400px p-2"
                    multiple
                    clearable></MySelect>

                <!-- 语言筛选 -->
                <MySelect v-model="selectedLanguage" :options="languageOptions" class="!w-200px p-2"></MySelect>

                <!-- 排序 -->
                <MySelect v-model="selectedSort" :options="sortOptions" class="!w-200px p-2"></MySelect>

                <!-- 排序方向切换 -->
                <div
                    class="flex items-center justify-center p-2 rounded-md bg-card hover:bg-hover transition-colors cursor-pointer text-primary border-light"
                    @click="toggleSortOrder"
                    :title="sortOrder === 'asc' ? 'Switch to descending' : 'Switch to ascending'">
                    <span
                        class="text-[24px]"
                        :class="sortOrder === 'desc' ? 'i-md-descending' : 'i-md-ascending'"></span>
                </div>

                <!-- 视图切换 -->
                <div class="flex rounded border-light">
                    <div
                        v-for="model in appStore.viewModes"
                        :key="model.name"
                        class="p-2 transition-colors bg-[--sv-card-color] cursor-pointer"
                        :style="{
                            backgroundColor:
                                appStore.viewMode === model.name ? 'var(--sv-primary-solid)' : '--sv-card-color',
                            color: appStore.viewMode === model.name ? '#ffffff' : 'var(--sv-text-color-2)',
                        }"
                        @click="appStore.setViewMode(model.name)">
                        <span :class="model.icon" class="text-[24px]"></span>
                    </div>
                </div>
            </div>
        </div>

        <div v-if="isLoading || isSearching" class="flex justify-center items-center py-16 px-6">
            <div class="text-center">
                <LoadingSpinner />
                <p class="mt-4 text-light-text-muted">Searching repositories...</p>
            </div>
        </div>
        <div
            v-else-if="reposStore.repositories.length === 0 && appStore.viewMode !== 'list'"
            class="flex justify-center items-center py-16 px-6 text-t-placeholder">
            <div class="text-center">
                <div class="inline-flex items-center justify-center w-16 h-16 rounded-full bg-light-bg-mute mb-4">
                    <span class="i-md-star text-2xl"></span>
                </div>
                <p class="font-bold">No repositories found</p>
                <p class="mt-2">You don't have any starred repositories yet.</p>
                <p class="mt-2">Star some repositories on GitHub to see them here.</p>
            </div>
        </div>
        <template v-else>
            <RepoList
                v-if="appStore.viewMode === 'list'"
                class="p-6"
                :repos="reposStore.repositories"
                @click="handleRepoClick"
                @loadMore="handleLoadMore" />
            <RepoCard v-else :repos="reposStore.repositories" @click="handleRepoClick" />

            <!-- 滚动加载触发元素 -->
            <div ref="loadingTriggerRef" class="h-10 w-full"></div>
        </template>
    </div>
</template>
