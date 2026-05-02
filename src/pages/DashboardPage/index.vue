<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useReposStore } from '@/stores/repos'

const reposStore = useReposStore()

onMounted(() => {
    reposStore.fetchRepoStats()
})

// Stats card data
const stats = computed(() => [
    {
        label: 'Total Stars',
        value: reposStore.stats.total_repos,
        color: '#3178c6',
        icon: 'i-md-star',
    },
    {
        label: 'Active Tags',
        value: reposStore.stats.category_count,
        color: '#dea584',
        icon: 'i-md-local-offer',
    },
    {
        label: 'Uncategorized',
        value: reposStore.stats.uncategorized_count,
        color: '#89e051',
        icon: 'i-md-inbox',
        total: reposStore.stats.total_repos,
    },
    {
        label: 'Recent Stars',
        value: reposStore.stats.recent_starred_count,
        color: '#e34c26',
        icon: 'i-md-trending-up',
    },
])
</script>

<template>
    <div class="overflow-y-auto h-full">
        <!-- 页面标题 -->
        <div class="p-6 mb-6 shadow-md">
            <h2 class="text-3xl text-t-primary font-semibold tracking-tight">Overview</h2>
            <p class="mt-4">Manage and curate your collection of {{ reposStore.totalCount }} GitHub stars.</p>
        </div>

        <!-- 统计卡片区域 -->
        <div class="grid grid-cols-4 gap-4 my-6 mx-6">
            <div
                v-for="stat in stats"
                :key="stat.label"
                class="rounded-md p-6 bg-card border-light flex items-center gap-4">
                <div>
                    <!-- 图标 + 数值区域（前置） -->
                    <div class="flex items-center gap-4 mb-3">
                        <span :class="stat.icon" class="text-3xl" :style="{ color: stat.color }"></span>
                        <div class="text-3xl font-extrabold" :style="{ color: stat.color }">
                            {{ stat.value }}
                        </div>
                    </div>

                    <!-- 标签 -->
                    <div class="text-lg text-t-secondary">{{ stat.label }}</div>
                </div>
                <!-- 进度指示（仅针对未分类） -->
                <div v-if="stat.total && stat.total > 0" class="flex-1">
                    <div class="h-2 bg-popover rounded-full overflow-hidden">
                        <div
                            class="h-full transition-all duration-300"
                            :style="{
                                width: `${(stat.value / stat.total) * 100}%`,
                                backgroundColor: stat.color,
                            }"></div>
                    </div>
                    <div class="text-xs text-t-tertiary mt-1 text-right">
                        {{ Math.round((stat.value / stat.total) * 100) }}% of total
                    </div>
                </div>
            </div>
        </div>

        <!-- 双列布局：热门仓库 + 最近活动 -->
        <div class="grid grid-cols-2 gap-4 mx-6">
            <!-- 左列：最近更新仓库 -->
            <RecentlyUpdate />

            <!-- 右列：最近活动 -->
            <DeprecationWarning />
        </div>
    </div>
</template>

<style scoped>
.repo-card-gradient {
    border-left: 4px solid transparent;
    border-image: linear-gradient(to bottom, #7800ce, #9333ea) 1;
}
</style>
