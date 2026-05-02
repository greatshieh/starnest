<script setup lang="ts">
import { convertFileSrc } from '@tauri-apps/api/core'
import type { Repository } from '@/types/api/repo'

defineProps<{
    showRepoModal: boolean
    viewingTag: { id: number; name: string; color: string } | null
    repos: Repository[]
}>()

const emit = defineEmits<{
    'update:showRepoModal': [value: boolean]
    'goto-detail': [repo: Repository]
}>()

function handleClose() {
    emit('update:showRepoModal', false)
}

function handleGoToDetail(repo: Repository) {
    emit('goto-detail', repo)
}
</script>

<template>
    <Modal
        :model-value="showRepoModal"
        @update:model-value="v => emit('update:showRepoModal', v)"
        width="auto"
        height="auto"
        :showHeader="false">
        <div class="relative p-5 w-[500px]" style="max-height: 60vh">
            <h3 class="text-xl font-semibold mb-4" style="color: var(--sv-text-color-1)">
                Repos associated with tag 「<span :style="{ color: viewingTag?.color }">{{ viewingTag?.name }}</span
                >」
                <span class="text-base font-normal ml-2" style="color: var(--sv-text-color-2)">
                    ({{ repos.length }} repos)
                </span>
            </h3>

            <div class="overflow-y-auto" style="max-height: 400px">
                <template v-if="repos.length > 0">
                    <div
                        v-for="repo in repos"
                        :key="repo.id"
                        class="flex items-center gap-3 p-3 rounded-md cursor-pointer transition-colors hover:bg-hover mb-2"
                        @click="handleGoToDetail(repo)">
                        <img
                            :src="convertFileSrc(repo?.owner_avatar_url?.replace('https://', '') || '')"
                            :alt="repo?.owner_login || ''"
                            class="w-9 h-9 rounded-full" />
                        <div class="flex-1 min-w-0">
                            <div class="flex items-center gap-2">
                                <span class="font-medium truncate" style="color: var(--sv-text-color-1)">
                                    {{ repo.full_name }}
                                </span>
                            </div>
                            <p class="text-sm truncate mt-0.5" style="color: var(--sv-text-color-2)">
                                {{ repo.description || 'No description' }}
                            </p>
                            <div class="flex items-center gap-3 mt-1.5">
                                <span class="text-sm flex items-center gap-1">
                                    <span class="i-md-star"></span>
                                    {{ repo.stargazers_count }}
                                </span>
                                <span class="text-sm flex items-center gap-1">
                                    <span class="i-md-fork"></span>
                                    {{ repo.forks_count }}
                                </span>
                            </div>
                        </div>
                        <span class="text-[var(--sv-text-color-3)] flex-shrink-0">
                            <span class="i-md-arrow-forward"></span>
                        </span>
                    </div>
                </template>
                <div v-else class="py-12 text-center text-lg" style="color: var(--sv-placeholder-color)">
                    No repos associated with this tag
                </div>
            </div>

            <div
                class="flex items-center justify-end gap-2 mt-4 pt-4 border-t"
                style="border-color: var(--sv-border-color)">
                <button @click="handleClose">Close</button>
            </div>
        </div>
    </Modal>
</template>
