<script setup lang="ts">
import { getRecentUpdatedRepos } from '@/api/repo'
import { useReposStore } from '@/stores/repos'
import { api } from '@/types'
import { formatNumber, formatTimeAgo } from '@/utils/format'
import { getTopLanguages, LanguageData } from '@/utils/language'
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

const recentRepos = ref<api.repo.Repository[]>([])
const isLoading = ref(false)

const { parseRepoTopics } = useReposStore()

const getTopLanguage = (language: string | null): LanguageData => {
    const topLanguages = getTopLanguages(language)
    if (topLanguages.length === 0) return { language: 'Unknown', count: 0, color: 'var(--sv-muted)' }
    return topLanguages[0]
}

const fetchRecentUpdatedRepos = async () => {
    isLoading.value = true
    try {
        const response = await getRecentUpdatedRepos()
        recentRepos.value = response.map(parseRepoTopics)
    } catch (error) {
        console.error('Failed to fetch recent updated repos:', error)
    } finally {
        isLoading.value = false
    }
}

const handleRepoClick = (repo: api.repo.Repository) => {
    router.push({
        name: 'RepoDetail',
        params: { owner: repo.owner_login, name: repo.name },
        state: {
            repo: JSON.stringify(repo),
        },
    })
}

onMounted(async () => {
    await fetchRecentUpdatedRepos()
})
</script>

<template>
    <div class="rounded-md bg-card border-light">
        <div
            class="px-4 py-3 flex items-center justify-between border-b-1px border-b-solid border-b-[var(--sv-border-color)] sticky top-0 bg-popover z-100">
            <div class="flex flex-col gap-2">
                <div class="text-lg font-black text-t-primary">Recently Updated</div>
                <div class="text-md">{{ recentRepos.length }} repos</div>
            </div>
            <button class="text-2xl !border-none !bg-transparent" @click="fetchRecentUpdatedRepos">
                <span class="i-md-sync"></span>
            </button>
        </div>
        <div v-if="recentRepos.length === 0" class="px-4 py-8 text-center text-t-placeholder">
            No recently updated repos found.
        </div>
        <div v-else class="mb-4 h-[300px] overflow-x-auto relative">
            <div
                v-for="repo in recentRepos"
                :key="repo.github_id"
                class="px-4 py-3 flex items-center gap-3 cursor-pointer transition-colors hover:bg-hover"
                @click="handleRepoClick(repo)">
                <div class="flex-1 min-w-0">
                    <div class="text-md font-medium truncate">
                        {{ repo.full_name }}
                    </div>
                    <div class="truncate mt-2 text-t-secondary">
                        {{ repo.description || 'No description' }}
                    </div>
                </div>
                <div class="flex items-center gap-3 shrink-0">
                    <span class="flex items-center gap-1">
                        <span
                            class="inline-block size-2 rounded-full"
                            :style="{ background: getTopLanguage(repo.language).color }"></span>
                        {{ getTopLanguage(repo.language).language }}
                    </span>
                    <div>
                        <span class="i-md-star"></span>
                        <span>{{ formatNumber(repo.stargazers_count) }}</span>
                    </div>
                    <div>
                        <span class="i-md-fork"></span>
                        <span>{{ formatNumber(repo.forks_count) }}</span>
                    </div>
                    <div>
                        <span class="i-md-history"></span>
                        <span>{{ formatTimeAgo(repo.pushed_at) }}</span>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
