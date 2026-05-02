<script setup lang="ts">
import type { api as ApiTypes } from '@/types'
import { presetColors } from '@/constants/colors'
import Modal from '@/components/Modal.vue'

defineProps<{
    show: boolean
    collection: (ApiTypes.collection.CollectionWithRepoCount & { repos: ApiTypes.repo.Repository[] }) | null
}>()

const emit = defineEmits<{
    'update:show': [value: boolean]
    edit: []
    delete: []
    'remove-repo': [githubId: number]
}>()

// 格式化数字
const formatNumber = (num: number): string => {
    if (num >= 10000) {
        return (num / 10000).toFixed(1) + 'k'
    }
    return num.toString()
}
</script>

<template>
    <Modal
        :modelValue="show"
        @update:modelValue="emit('update:show', $event)"
        width="auto"
        height="auto"
        :showHeader="false">
        <div class="relative p-5 overflow-hidden" style="width: 520px; max-height: 80vh">
            <!-- 头部 -->
            <div class="flex items-center justify-between mb-6 text-lg">
                <div class="flex items-center gap-2">
                    <span
                        class="rounded-full size-3"
                        :style="{
                            background: presetColors[(collection?.id || 0) % presetColors.length],
                        }"></span>
                    <div class="font-semibold" style="color: var(--sv-text-color-1)">
                        {{ collection?.name || '-' }}
                    </div>
                </div>
                <div class="flex items-center gap-2">
                    <button @click="emit('edit')">
                        <span class="i-md-edit text-lg"></span>
                    </button>
                    <button @click="emit('delete')">
                        <span class="i-md-delete text-lg text-[var(--sv-error-solid)]"></span>
                    </button>
                </div>
            </div>

            <!-- 描述 -->
            <p class="text-sm mb-4">
                {{ collection?.description || '-' }}
            </p>

            <!-- 仓库列表 -->
            <div
                class="overflow-y-auto rounded-md mb-4"
                style="max-height: 300px; border: 1px solid var(--sv-border-color)">
                <div
                    v-for="(repo, index) in collection?.repos || []"
                    :key="repo.github_id"
                    class="flex items-center gap-3 px-3 py-2 transition-colors"
                    :style="{
                        background: index % 2 === 0 ? 'var(--sv-card-color)' : 'var(--sv-popover-color)',
                        borderBottom: '1px solid var(--sv-border-color)',
                    }">
                    <span class="truncate flex-1 font-medium">{{ repo.full_name }}</span>
                    <span class="i-md-star"></span>
                    <span>
                        {{ formatNumber(repo.stargazers_count || 0) }}
                    </span>
                    <button class="shrink-0 error" title="Remove" @click="emit('remove-repo', repo.github_id)">
                        <span class="i-md-delete"></span>
                    </button>
                </div>
                <!-- 空状态 -->
                <div
                    v-if="collection?.repos.length === 0"
                    class="py-8 text-center text-lg"
                    style="color: var(--sv-text-muted)">
                    No repos
                </div>
            </div>

            <!-- 关闭按钮 -->
            <button @click="emit('update:show', false)">Close</button>
        </div>
    </Modal>
</template>
