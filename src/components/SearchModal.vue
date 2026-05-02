<script setup lang="ts">
import { ref, nextTick, watch, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { useSearch } from '@/composables/useSearch'
import { formatNumber } from '@/utils/format'
import MyInput from './MyInput.vue'

const visible = defineModel({
    type: Boolean,
    default: false,
})

const router = useRouter()

const search = useSearch()

const inputRef = ref<InstanceType<typeof MyInput> | null>(null)
const resultsContainerRef = ref<HTMLDivElement | null>(null)
const itemRefs = ref<HTMLElement[]>([])

const setItemRef = (el: HTMLElement | null, index: number) => {
    if (el) {
        itemRefs.value[index] = el
    }
}

const scrollToItem = (index: number) => {
    const item = itemRefs.value[index]
    if (item && resultsContainerRef.value) {
        item.scrollIntoView({
            behavior: 'smooth',
            block: 'nearest',
        })
    }
}

const handleKeydownWrapper = (e: KeyboardEvent) => {
    search.handleKeydown(e, scrollToItem)

    if (e.key === 'Enter') {
        e.preventDefault()
        inputRef.value?.blur()

        if (search.selectedIndex.value >= 0 && search.searchResults.value.length > 0) {
            selectRepo(search.searchResults.value[search.selectedIndex.value])
        } else if (search.searchResults.value.length > 0) {
            search.selectedIndex.value = 0
            scrollToItem(0)
        }
    }
}

const handleEsc = (e: KeyboardEvent) => {
    if (e.key === 'Escape') {
        visible.value = false
    }
}

onMounted(() => {
    window.addEventListener('keydown', handleEsc)
})

onUnmounted(() => {
    window.removeEventListener('keydown', handleEsc)
})

const selectRepo = (repo: any) => {
    visible.value = false
    router.push({
        name: 'RepoDetail',
        params: {
            owner: repo.owner_login,
            name: repo.name,
        },
        state: {
            repo: JSON.stringify(repo),
        },
    })
}

const handleScroll = (e: Event) => {
    const target = e.target as HTMLDivElement
    if (target.scrollTop + target.clientHeight >= target.scrollHeight - 20 && !search.isLoadingMore.value) {
        search.loadMore()
    }
}

watch(visible, async newVal => {
    if (newVal) {
        search.resetSearch()
        await nextTick()
        inputRef.value?.focus()
    }
})
</script>

<template>
    <Modal v-model="visible" :width="'600px'" :height="'500px'" :show-header="false" click-on-overlay>
        <div class="p-4 flex flex-col h-full">
            <div class="flex items-center gap-2 mb-4">
                <span class="i-md-search text-muted"></span>
                <MyInput
                    ref="inputRef"
                    v-model="search.searchQuery.value"
                    placeholder="搜索仓库，支持空格分隔多关键字..."
                    @keydown="handleKeydownWrapper" />
            </div>

            <div ref="resultsContainerRef" class="flex-1 overflow-y-auto" @scroll="handleScroll">
                <div v-if="search.isLoading.value" class="flex items-center justify-center py-8">
                    <span class="i-md-loading animate-spin"></span>
                </div>

                <div
                    v-else-if="search.searchQuery.value.trim() && search.searchResults.value.length === 0"
                    class="text-center py-8 text-t-placeholder">
                    未找到匹配的仓库
                </div>

                <div v-else-if="!search.searchQuery.value.trim()" class="text-center py-8 text-t-placeholder">
                    请输入搜索关键字
                </div>

                <div v-else class="space-y-1">
                    <div
                        v-for="(repo, index) in search.searchResults.value"
                        :key="`${repo.owner_login}-${repo.name}`"
                        :ref="el => setItemRef(el as HTMLElement, index)"
                        class="flex items-center gap-3 px-3 py-2 rounded-md cursor-pointer transition-colors"
                        :class="{
                            'bg-primary text-[var(--sv-btn-primary-text-color)]': index === search.selectedIndex.value,
                            'bg-hover': index !== search.selectedIndex.value && index === search.mouseOverIndex.value,
                        }"
                        @click="selectRepo(repo)"
                        @mouseenter="
                            () => {
                                search.mouseOverIndex.value = index
                                inputRef?.blur()
                            }
                        "
                        @mouseleave="search.mouseOverIndex.value = -1">
                        <span class="i-md-repository"></span>
                        <div class="flex-1 min-w-0">
                            <div class="font-medium truncate">{{ repo.owner_login }} / {{ repo.name }}</div>
                            <div class="truncate">{{ repo.description }}</div>
                        </div>
                        <div class="flex items-center gap-3">
                            <span v-if="repo.stargazers_count" class="flex items-center gap-1">
                                <span class="i-md-star"></span>
                                {{ formatNumber(repo.stargazers_count) }}
                            </span>
                            <span v-if="repo.forks_count" class="flex items-center gap-1">
                                <span class="i-md-fork"></span>
                                {{ formatNumber(repo.forks_count) }}
                            </span>
                        </div>
                    </div>
                    <div v-if="search.isLoadingMore.value" class="flex items-center justify-center py-4">
                        <span class="i-md-loading animate-spin"></span>
                        <span class="ml-2 text-t-placeholder">加载更多...</span>
                    </div>
                    <div
                        v-if="
                            !search.isLoadingMore.value &&
                            search.searchResults.value.length > 0 &&
                            search.searchResults.value.length >= search.totalCount.value
                        "
                        class="text-center py-4 text-t-placeholder">
                        已加载全部结果
                    </div>
                </div>
            </div>

            <div
                class="pt-3 border border-t border-[var(--sv-border-color)] flex items-center justify-between text-t-placeholder">
                <span>Enter 确认选择 | Arrow Up/Down 切换 | Esc 关闭</span>
            </div>
        </div>
    </Modal>
</template>
