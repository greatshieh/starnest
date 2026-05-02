/**
 * @module useSearch
 * @description 搜索功能的可复用 composable，封装了仓库搜索、分页加载、键盘导航等核心逻辑。
 *              该 composable 与 UI 解耦，可在任何需要搜索功能的组件中复用。
 */
import { ref, watch, type Ref } from 'vue'
import { useReposStore } from '@/stores/repos'
import { useDebounce } from './useDebounce'

/**
 * @interface UseSearchReturn
 * @description useSearch composable 的返回类型定义
 */
interface UseSearchReturn {
    /** 搜索关键词 */
    searchQuery: Ref<string>
    /** 搜索结果列表 */
    searchResults: Ref<any[]>
    /** 当前选中项的索引（-1 表示未选中） */
    selectedIndex: Ref<number>
    /** 鼠标悬停项的索引（-1 表示无悬停） */
    mouseOverIndex: Ref<number>
    /** 是否正在加载第一页数据 */
    isLoading: Ref<boolean>
    /** 当前页码 */
    currentPage: Ref<number>
    /** 搜索结果总数 */
    totalCount: Ref<number>
    /** 是否正在加载更多数据 */
    isLoadingMore: Ref<boolean>
    /**
     * 执行搜索请求
     * @param query - 搜索关键词
     * @param page - 页码，默认 1
     * @param append - 是否追加到现有结果，默认 false（覆盖）
     */
    searchRepos: (query: string, page?: number, append?: boolean) => Promise<void>
    /** 加载下一页数据（仅追加，不自动选中） */
    loadMore: () => Promise<void>
    /** 加载下一页数据并自动选中新数据的第一项 */
    loadMoreWithScroll: () => Promise<void>
    /**
     * 处理键盘事件（方向键导航）
     * @param e - 键盘事件对象
     * @param scrollToItem - 可选的滚动回调函数，用于跳转到指定索引的项目
     */
    handleKeydown: (e: KeyboardEvent, scrollToItem?: (index: number) => void) => void
    /** 重置搜索状态（清空关键词、结果、选中状态等） */
    resetSearch: () => void
    /** 获取关键词数组（按空格分割） */
    keywords: () => string[]
}

/**
 * @function useSearch
 * @description 提供搜索功能的 composable，包含搜索、分页、键盘导航等完整逻辑。
 *              特点：
 *              1. 支持防抖搜索（300ms）
 *              2. 支持分页加载（滚动/按键触底自动加载）
 *              3. 智能键盘导航（到达边界时可跳转到鼠标悬停位置）
 *              4. 与 UI 解耦，通过回调函数处理滚动等界面操作
 * @returns {UseSearchReturn} 搜索相关的状态和方法
 */
export function useSearch(): UseSearchReturn {
    /** 搜索关键词 */
    const searchQuery = ref('')

    /** 搜索结果列表 */
    const searchResults = ref<any[]>([])

    /** 当前选中项的索引（-1 表示未选中） */
    const selectedIndex = ref(-1)

    /** 鼠标悬停项的索引（-1 表示无悬停），用于智能导航 */
    const mouseOverIndex = ref(-1)

    /** 是否正在加载第一页数据 */
    const isLoading = ref(false)

    /** 当前页码 */
    const currentPage = ref(1)

    /** 搜索结果总数 */
    const totalCount = ref(0)

    /** 是否正在加载更多数据 */
    const isLoadingMore = ref(false)

    /** 仓库数据 store */
    const reposStore = useReposStore()

    /**
     * 执行搜索请求
     * @param query - 搜索关键词
     * @param page - 页码，默认 1
     * @param append - 是否追加到现有结果，默认 false（覆盖）
     */
    const searchRepos = async (query: string, page: number = 1, append: boolean = false): Promise<void> => {
        // 如果关键词为空，清空结果并返回
        if (!query.trim()) {
            searchResults.value = []
            return
        }

        // 根据页码设置不同的加载状态
        if (page === 1) {
            isLoading.value = true
        } else {
            isLoadingMore.value = true
        }

        try {
            // 调用 store 层的搜索方法
            const result = await reposStore.searchRepos(query, page)

            // 根据 append 参数决定是覆盖还是追加结果
            if (append) {
                searchResults.value = [...searchResults.value, ...result.repos]
            } else {
                searchResults.value = result.repos
            }

            // 更新分页信息
            totalCount.value = result.total
            currentPage.value = result.page
        } catch (error) {
            console.error('Search failed:', error)
            // 只有非追加模式下才清空结果
            if (!append) {
                searchResults.value = []
            }
        } finally {
            // 重置加载状态
            isLoading.value = false
            isLoadingMore.value = false
        }
    }

    /**
     * 加载下一页数据（仅追加，不自动选中）
     * 用于滚动触底时的加载场景
     */
    const loadMore = async (): Promise<void> => {
        // 如果正在加载或已加载全部，直接返回
        if (isLoadingMore.value || searchResults.value.length >= totalCount.value) {
            return
        }
        // 加载下一页并追加到现有结果
        await searchRepos(searchQuery.value, currentPage.value + 1, true)
    }

    /**
     * 加载下一页数据并自动选中新数据的第一项
     * 用于键盘导航到达最后一项时的加载场景
     */
    const loadMoreWithScroll = async (): Promise<void> => {
        // 如果正在加载或已加载全部，直接返回
        if (isLoadingMore.value || searchResults.value.length >= totalCount.value) {
            return
        }

        // 记录当前数据长度，用于后续选中新数据的第一项
        const currentLength = searchResults.value.length

        // 加载下一页
        await searchRepos(searchQuery.value, currentPage.value + 1, true)

        // 如果成功加载了新数据，自动选中新数据的第一项
        if (searchResults.value.length > currentLength) {
            selectedIndex.value = currentLength
        }
    }

    /**
     * 处理键盘事件（方向键导航）
     * @param e - 键盘事件对象
     * @param scrollToItem - 可选的滚动回调函数，用于跳转到指定索引的项目
     *
     * 导航逻辑：
     * - 向下键 (ArrowDown): 选中下一项；如果已到最后一项且有更多数据则加载更多；
     *                       如果已到最后一项且无更多数据但鼠标有悬停，则跳转到鼠标位置
     * - 向上键 (ArrowUp): 选中上一项；如果已到第一项且鼠标有悬停，则跳转到鼠标位置；
     *                     如果已到第一项且无鼠标悬停，则取消选中（-1）
     */
    const handleKeydown = (e: KeyboardEvent, scrollToItem?: (index: number) => void): void => {
        const results = searchResults.value

        if (e.key === 'ArrowDown') {
            e.preventDefault()

            // 如果还没到最后一项，选中下一项
            if (selectedIndex.value < results.length - 1) {
                selectedIndex.value++
                scrollToItem?.(selectedIndex.value)
            }
            // 如果已到最后一项且还有更多数据，加载更多
            else if (results.length > 0 && results.length < totalCount.value && !isLoadingMore.value) {
                loadMoreWithScroll().then(() => {
                    scrollToItem?.(selectedIndex.value)
                })
            }
            // 如果已到最后一项且无更多数据，但鼠标有悬停，跳转到鼠标位置
            else if (results.length > 0 && selectedIndex.value === results.length - 1) {
                if (mouseOverIndex.value >= 0 && mouseOverIndex.value < results.length) {
                    selectedIndex.value = mouseOverIndex.value
                    scrollToItem?.(selectedIndex.value)
                }
            }
        } else if (e.key === 'ArrowUp') {
            e.preventDefault()

            // 如果还没到第一项，选中上一项
            if (selectedIndex.value > 0) {
                selectedIndex.value--
                scrollToItem?.(selectedIndex.value)
            }
            // 如果已到第一项且鼠标有悬停，跳转到鼠标位置
            else if (selectedIndex.value === 0) {
                if (mouseOverIndex.value >= 0 && mouseOverIndex.value < results.length) {
                    selectedIndex.value = mouseOverIndex.value
                    scrollToItem?.(selectedIndex.value)
                } else {
                    // 如果没有鼠标悬停，取消选中
                    selectedIndex.value = -1
                }
            }
        }
    }

    /**
     * 重置搜索状态
     * 清空关键词、结果、选中状态、分页信息
     */
    const resetSearch = (): void => {
        searchQuery.value = ''
        searchResults.value = []
        selectedIndex.value = -1
        currentPage.value = 1
        totalCount.value = 0
    }

    /**
     * 监听搜索关键词变化
     * 当关键词变化时自动执行搜索（带 300ms 防抖）
     */
    const debouncedQuery = useDebounce(searchQuery, 300)
    
    watch(debouncedQuery, newVal => {
        // 重置选中状态和分页信息
        selectedIndex.value = -1
        currentPage.value = 1
        totalCount.value = 0

        if (newVal.trim()) {
            searchRepos(newVal, 1)
        } else {
            searchResults.value = []
        }
    })

    /**
     * 获取关键词数组（按空格分割）
     * @returns {string[]} 关键词数组（过滤空字符串）
     */
    const keywords = (): string[] => {
        return searchQuery.value.split(/\s+/).filter(k => k.trim())
    }

    return {
        searchQuery,
        searchResults,
        selectedIndex,
        mouseOverIndex,
        isLoading,
        currentPage,
        totalCount,
        isLoadingMore,
        searchRepos,
        loadMore,
        loadMoreWithScroll,
        handleKeydown,
        resetSearch,
        keywords,
    }
}
