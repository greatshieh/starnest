<script setup lang="ts">
import { getRepoStatusCategories } from '@/api/repo'
import { useReposStore } from '@/stores/repos'
import { api } from '@/types'
import { formatTimeAgo } from '@/utils/format'
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'

const coloarMap: Record<api.repo.Repository['status'], string> = {
    archived: 'info',
    inactive: 'warning',
    deprecated: 'error',
}

const router = useRouter()

const repoStatusCategories = ref<api.repo.Repository[]>([])
const isLoading = ref(false)

const { parseRepoTopics } = useReposStore()

const fetchRepoStatusCategories = async () => {
    isLoading.value = true
    try {
        const response = await getRepoStatusCategories()
        repoStatusCategories.value = response.map(parseRepoTopics)
    } catch (error) {
        console.error('Failed to fetch repo status categories:', error)
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
    await fetchRepoStatusCategories()
})
</script>

<template>
    <div class="rounded-md bg-card border-light">
        <div
            class="px-4 py-3 flex items-center justify-between border-b-1px border-b-solid border-b-[var(--sv-border-color)] sticky top-0 bg-popover z-100">
            <div class="flex flex-col gap-2">
                <div class="text-lg font-black text-t-primary]">Deprecation Warning</div>
                <div class="text-md">{{ repoStatusCategories.length }} repos</div>
            </div>
            <button class="text-2xl !border-none !bg-transparent" @click="fetchRepoStatusCategories">
                <span class="i-md-sync"></span>
            </button>
        </div>
        <div v-if="repoStatusCategories.length === 0" class="px-4 py-8 text-center text-t-placeholder">
            No repo status categories found.
        </div>
        <div v-else class="mb-4 h-[300px] overflow-x-auto relative">
            <div
                v-for="repo in repoStatusCategories"
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
                    <span
                        class="py-1 px-2 rounded-md text-[#fff] dark:text-[#000]"
                        :style="{
                            backgroundColor: `var(--sv-${coloarMap[repo.status]}-solid)`,
                        }"
                        >{{ repo.status }}</span
                    >
                    <span>{{ formatTimeAgo(repo.pushed_at) }}</span>
                </div>
            </div>
        </div>
    </div>
</template>
