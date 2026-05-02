<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import type { api as ApiTypes } from '@/types'
import { presetColors } from '@/constants/colors'

const props = defineProps<{
    collections: Array<ApiTypes.collection.CollectionWithRepoCount & { repos: ApiTypes.repo.Repository[] }>
    onClick?: (collection: ApiTypes.collection.CollectionWithRepoCount & { repos: ApiTypes.repo.Repository[] }) => void
}>()

const emit = defineEmits<{
    click: [collection: ApiTypes.collection.CollectionWithRepoCount & { repos: ApiTypes.repo.Repository[] }]
}>()

const CARD_W = 380
const CARD_H = 204
const GAP = 16
const containerRef = ref<HTMLElement>()
const cols = ref(3)

const handleClick = (
    collection: ApiTypes.collection.CollectionWithRepoCount & { repos: ApiTypes.repo.Repository[] },
) => {
    if (collection) {
        if (props.onClick) {
            props.onClick(collection)
        }
        emit('click', collection)
    }
}

const updateCols = () => {
    if (!containerRef.value) return
    const w = containerRef.value.clientWidth
    cols.value = Math.max(1, Math.floor((w + GAP) / (CARD_W + GAP)))
}

onMounted(() => {
    updateCols()
    window.addEventListener('resize', updateCols)
})
onUnmounted(() => {
    window.removeEventListener('resize', updateCols)
})
</script>

<template>
    <div
        ref="containerRef"
        class="grid gap-4 justify-center p-6"
        :style="{ gridTemplateColumns: `repeat(${cols}, ${CARD_W}px)` }">
        <div
            v-for="(collection, index) in collections"
            :key="collection.id"
            class="relative bg-card hover:shadow-xl p-5 rounded-xl border border-light transition-all duration-300 cursor-pointer flex flex-col gap-3 overflow-hidden"
            :style="{ height: CARD_H + 'px' }"
            @click="handleClick(collection)">
            <div
                class="absolute top-0 left-0 w-full h-1"
                :style="{ backgroundColor: collection.color || presetColors[index % presetColors.length] }"></div>
            <div class="flex items-start justify-between">
                <div class="flex items-center gap-3">
                    <div
                        class="w-9 h-9 rounded-full flex items-center justify-center"
                        :style="{ backgroundColor: collection.color || presetColors[index % presetColors.length] }">
                        <span class="i-md-collection text-white text-lg"></span>
                    </div>
                    <div>
                        <h3 class="font-bold text-t-primary transition-colors leading-relaxed">
                            {{ collection.name }}
                        </h3>
                        <p class="text-[var(--sv-text-color-3)] transition-colors tracking-wider font-semibold">
                            {{ collection.repo_count }} repos
                        </p>
                    </div>
                </div>
            </div>
            <p class="text-sm text-t-regular leading-relaxed line-clamp-2">
                {{ collection.description || 'No description available' }}
            </p>

            <div class="mt-auto">
                <div class="flex flex-wrap gap-2">
                    <span
                        v-for="repo in collection.repos.slice(0, 3)"
                        :key="repo.github_id"
                        class="text-xs px-2 py-1 rounded-md"
                        style="background: var(--sv-surface); color: var(--sv-text-color-2)">
                        {{ repo.full_name.split('/')[1] }}
                    </span>
                    <span
                        v-if="collection.repo_count > 3"
                        class="text-xs px-2 py-1 rounded-md"
                        style="background: var(--sv-surface); color: var(--sv-text-muted)">
                        +{{ collection.repo_count - 3 }}
                    </span>
                </div>
            </div>
        </div>
    </div>
</template>
