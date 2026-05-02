<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useSyncStore } from '@/stores/sync'
import { useReposStore } from '@/stores/repos'
import { useRoute } from 'vue-router'
import { useAppStore } from '@/stores/apps'
import { useCategoriesStore } from '@/stores/categories'
import { useCollectionsStore } from '@/stores/collections'

defineOptions({
    name: 'StatusBar',
})

const syncStore = useSyncStore()
const repoStore = useReposStore()
const categoriesStore = useCategoriesStore()
const collectionStore = useCollectionsStore()
const route = useRoute()
const appStore = useAppStore()

const lastSyncTimeAgo = ref('从未同步')
let timerId: number | null = null

const statusBarContent = computed(() => {
    const name = route.name as string
    switch (name) {
        case 'Dashboard':
            return {
                left: `${syncStore.isSyncing ? '同步中...' : `已同步 · ${lastSyncTimeAgo.value}`}`,
                center: `${repoStore.stats.total_repos} 个仓库 · ${repoStore.stats.category_count} 个标签`,
                right: '⌘K 搜索 · ⌘B 侧边栏',
            }
        case 'Repo':
            return {
                left: '已选 0 项',
                center: `${repoStore.currentPage} / ${repoStore.totalPages} (${repoStore.totalCount} repos) · ${appStore.viewMode} view`,
                right: '⌘A 全选 · ⌘K 搜索',
            }
        case 'RepoDetail':
            return {
                left: `${route.params.owner}/${route.params.repo}`,
                center: '',
                right: '⌘N 新建笔记',
            }
        case 'Tag':
            return {
                left: `${categoriesStore.total} tags`,
                center: `${categoriesStore.currentPage} / ${categoriesStore.totalPages} (${categoriesStore.total} tags)`,
                right: '⌘N 新建标签 · ⌘K 搜索',
            }
        case 'Collection':
            return {
                left: `${collectionStore.collections.length} collections`,
                center: `${collectionStore.totalRepos} repos / ${collectionStore.collections.length} collections`,
                right: '⌘N 新建合集 · ⌘K 搜索',
            }
        default:
            return {
                left: `${syncStore.isSyncing ? '同步中...' : `已同步 · ${lastSyncTimeAgo.value}`}`,
                center: `${repoStore.stats.total_repos} 个仓库 · ${repoStore.stats.category_count} 个标签`,
                right: '⌘K 搜索 · ⌘B 侧边栏',
            }
    }
})

function updateLastSyncTime(): void {
    if (!syncStore.lastSync) return

    const lastSyncDate = new Date(syncStore.lastSync)
    const now = new Date()
    const diffMs = now.getTime() - lastSyncDate.getTime()

    const minutes = Math.floor(diffMs / (1000 * 60))
    const hours = Math.floor(minutes / 60)
    const days = Math.floor(hours / 24)

    if (days > 0) {
        lastSyncTimeAgo.value = `${days} 天前`
    } else if (hours > 0) {
        lastSyncTimeAgo.value = `${hours} 小时前`
    } else if (minutes > 0) {
        lastSyncTimeAgo.value = `${minutes} 分钟前`
    } else {
        lastSyncTimeAgo.value = '刚刚'
    }
}

onMounted(() => {
    updateLastSyncTime()
    timerId = window.setInterval(updateLastSyncTime, 60000)
})

onUnmounted(() => {
    if (timerId !== null) {
        clearInterval(timerId)
        timerId = null
    }
})
</script>

<template>
    <footer class="h-[32px] bg-card flex items-center justify-between px-4 text-sm select-none">
        <div class="flex items-center gap-4">
            <span>{{ statusBarContent.left }}</span>
        </div>
        <div class="flex items-center gap-4">
            <span>{{ statusBarContent.center }}</span>
        </div>
        <div class="flex items-center gap-4">
            <span>{{ statusBarContent.right }}</span>
        </div>
    </footer>
</template>
