<script setup lang="ts">
import { type Repository } from '@/stores/repos'
import { convertFileSrc } from '@tauri-apps/api/core'
import { onMounted, ref, computed } from 'vue'

const props = defineProps<{
    repos: Repository[]
    onClick?: (repo: Repository) => void
}>()

const emit = defineEmits<{
    loadMore: []
}>()

const ITEM_HEIGHT = 80
const BUFFER_COUNT = 5

const scrollContainerRef = ref<HTMLDivElement | null>(null)
const scrollTop = ref(0)
const containerHeight = ref(600)

const startIndex = computed(() => {
    const start = Math.floor(scrollTop.value / ITEM_HEIGHT) - BUFFER_COUNT
    return Math.max(0, start)
})

const endIndex = computed(() => {
    const visibleCount = Math.ceil(containerHeight.value / ITEM_HEIGHT)
    const end = startIndex.value + visibleCount + BUFFER_COUNT * 2
    return Math.min(props.repos.length, end)
})

const visibleRepos = computed(() => {
    return props.repos.slice(startIndex.value, endIndex.value)
})

const totalHeight = computed(() => {
    return props.repos.length * ITEM_HEIGHT
})

const offsetY = computed(() => {
    return startIndex.value * ITEM_HEIGHT
})

const getRepoTop = (index: number): number => {
    return offsetY.value + index * ITEM_HEIGHT
}

const handleScroll = (e: Event) => {
    const target = e.target as HTMLDivElement
    scrollTop.value = target.scrollTop

    const { scrollHeight, clientHeight } = target
    if (scrollTop.value + clientHeight >= scrollHeight - 500) {
        emit('loadMore')
    }
}

onMounted(() => {
    if (scrollContainerRef.value) {
        containerHeight.value = scrollContainerRef.value.clientHeight
    }
})

const formatStars = (count: number): string => {
    if (count >= 1000) {
        return (count / 1000).toFixed(1).replace(/\.0$/, '') + 'k'
    }
    return count.toString()
}

const formatForks = (count: number): string => {
    if (count >= 1000) {
        return (count / 1000).toFixed(1).replace(/\.0$/, '') + 'k'
    }
    return count.toString()
}

const languageColors: Record<string, string> = {
    TypeScript: '#3178c6',
    JavaScript: '#f1e05a',
    Python: '#3572A5',
    Rust: '#dea584',
    Go: '#00ADD8',
    Java: '#b07219',
    'C++': '#f34b7d',
    'C#': '#178600',
    Ruby: '#701516',
    Swift: '#F05138',
    Kotlin: '#A97BFF',
    Vue: '#41b883',
    React: '#61dafb',
}

const getLanguageColor = (language: string | null): string => {
    return languageColors[language || ''] || '#8b8b8b'
}
</script>

<template>
    <div
        ref="scrollContainerRef"
        class="relative w-full overflow-y-auto overflow-x-hidden rounded-lg border-light"
        :style="{ height: 'calc(100vh - 140px)' }"
        @scroll="handleScroll">
        <div :style="{ height: `${totalHeight}px`, position: 'relative' }">
            <div
                v-for="(repo, idx) in visibleRepos"
                :key="repo.id"
                class="absolute left-0 right-0 flex items-center gap-4 px-4 cursor-pointer transition-colors hover:bg-hover"
                :style="{
                    height: `${ITEM_HEIGHT}px`,
                    top: `${getRepoTop(idx)}px`,
                    backgroundColor: 'var(--sv-card-color)',
                }"
                @click="onClick?.(repo)">
                <div
                    class="flex-shrink-0 w-10 h-10 rounded-full overflow-hidden bg-[var(--sv-hover-color)] flex items-center justify-center">
                    <img
                        :src="convertFileSrc(repo?.owner_avatar_url?.replace('https://', '') || '')"
                        :alt="repo.owner_login"
                        class="w-full h-full object-cover" />
                </div>

                <div class="flex-1 min-w-0">
                    <span class="font-medium truncate text-t-primary">{{ repo.full_name }}</span>
                    <p class="text-sm truncate mt-1 text-t-regular">{{ repo.description || 'No description' }}</p>
                </div>

                <div class="flex items-center gap-4 flex-shrink-0">
                    <div v-if="repo.language" class="flex items-center gap-1.5 text-t-secondary">
                        <span
                            class="w-3 h-3 rounded-full flex-shrink-0"
                            :style="{ backgroundColor: getLanguageColor(repo.language) }"></span>
                        <span class="text-sm">{{ repo.language }}</span>
                    </div>

                    <div class="flex items-center gap-1 text-t-secondary">
                        <span class="i-md-star text-sm"></span>
                        <span class="text-sm">{{ formatStars(repo.stargazers_count) }}</span>
                    </div>

                    <div class="flex items-center gap-1 text-t-secondary">
                        <span class="i-md-fork text-sm"></span>
                        <span class="text-sm">{{ formatForks(repo.forks_count) }}</span>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
