<script setup lang="ts">
import { convertFileSrc } from '@tauri-apps/api/core'
import type { Repository } from '@/stores/repos'
import { formatNumber } from '@/utils/format'
import { getTopLanguages, parseLanguageJson } from '@/utils/language'
import { presetColors } from '@/constants/colors'
import { onMounted, onUnmounted, ref } from 'vue'

const props = defineProps<{
    repos: Repository[]
    onClick?: (repo: Repository) => void
}>()

const emit = defineEmits<{
    click: [repo: Repository]
}>()

const CARD_W = 380
const CARD_H = 204
const GAP = 16
const containerRef = ref<HTMLElement>()
const cols = ref(3)

const handleClick = (repo: Repository) => {
    if (repo) {
        if (props.onClick) {
            props.onClick(repo)
        }
        emit('click', repo)
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
            v-for="(repo, index) in repos"
            :key="repo.id"
            class="relative bg-card hover:shadow-xl p-5 rounded-xl border border-light transition-all duration-300 cursor-pointer flex flex-col gap-4 overflow-hidden"
            :style="{ height: CARD_H + 'px' }"
            @click="handleClick(repo)">
            <div
                class="absolute top-0 left-0 w-full h-1"
                :style="{ backgroundColor: `${presetColors[index % presetColors.length]}` }"></div>
            <div class="flex items-start justify-between">
                <div class="flex items-center gap-3">
                    <img
                        :src="convertFileSrc(repo?.owner_avatar_url?.replace('https://', '') || '')"
                        :alt="repo?.owner_login || ''"
                        class="w-9 h-9 rounded-full" />
                    <div>
                        <h3 class="font-bold text-t-primary transition-colors leading-relaxed">
                            {{ repo.name }}
                        </h3>
                        <p class="text-[var(--sv-text-color-3)] transition-colors tracking-wider font-semibold">
                            {{ repo.owner_login }}
                        </p>
                    </div>
                </div>
                <div class="gap-1 px-2 py-1 rounded-md flex items-center justify-center text-t-regular">
                    <span class="i-md-star"></span>
                    <span class="font-bold">{{ formatNumber(repo.stargazers_count) }}</span>
                </div>
            </div>
            <p class="text-sm text-t-regular leading-relaxed line-clamp-3">
                {{ repo.description || 'No description available' }}
            </p>

            <div class="mt-auto">
                <div v-if="parseLanguageJson(repo.language).length > 0" class="flex flex-wrap gap-3">
                    <span
                        v-for="(lang, _index) in getTopLanguages(repo.language, 3)"
                        :key="lang.language"
                        class="flex items-center gap-1.5 text-sm">
                        <span class="w-2 h-2 rounded-full" :style="{ backgroundColor: lang.color }"></span>
                        <span class="text-fg">{{ lang.language }}</span>
                    </span>
                    <span
                        v-if="parseLanguageJson(repo.language).length > 3"
                        class="flex items-center gap-1.5 text-sm text-t-secondary">
                        <span class="w-2 h-2 rounded-full bg-border-subtle"></span>
                        <span>+{{ parseLanguageJson(repo.language).length - 3 }}</span>
                    </span>
                </div>
                <div v-else class="h-3 flex items-center">
                    <span class="text-sm text-t-placeholder font-medium">unknown</span>
                </div>
            </div>
        </div>
    </div>
</template>
