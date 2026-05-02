<script setup lang="ts">
import { api } from '@/api'
import { useMessage } from '@/composables/useMessage'
import { useReposStore } from '@/stores/repos'
import { Repository } from '@/types/api/repo'
import { formatNumber } from '@/utils/format'
import { getLanguageBars, getTotalCount, parseLanguageJson } from '@/utils/language'
import { computed, onMounted, ref } from 'vue'
import { useWindowResize } from '@/composables/useWindowResize'
import { useModal } from '@/composables/useModal'

defineOptions({
    name: 'ReadMeTab',
})

const props = defineProps({
    repo: {
        type: Object as () => Repository,
        default: () => {},
    },
})

const repoStore = useReposStore()

const { success, error: messageErr } = useMessage()
const containerRef = ref<HTMLDivElement>()
const { isOpen: showCategoriesDialog, open: openCategoriesDialog, close: closeCategoriesDialog } = useModal()
const relatedRepo = ref<Repository>(props.repo)
const { height: windowHeight } = useWindowResize()
const repoCollections = ref<Array<{ id: number; name: string; color: string }>>([])

const contentHeight = computed(() => {
    if (!containerRef.value) return 0
    return windowHeight.value - containerRef.value.getBoundingClientRect().top - 2 * 32
})

const tagColors = [
    { bg: 'bg-[#f0dbff]', text: 'text-[#6800b4]' },
    { bg: 'bg-[#d0f2fe]', text: 'text-[#0c4a6e]' },
    { bg: 'bg-[#dcfce7]', text: 'text-[#166534]' },
    { bg: 'bg-[#fef3c7]', text: 'text-[#854d0e]' },
    { bg: 'bg-[#fce7f3]', text: 'text-[#9d174d]' },
    { bg: 'bg-[#e0e7ff]', text: 'text-[#4338ca]' },
    { bg: 'bg-[#fed7aa]', text: 'text-[#9a3412]' },
    { bg: 'bg-[#d1fae5]', text: 'text-[#065f46]' },
]

const getTagColor = (tag: string) => {
    let hash = 0
    for (let i = 0; i < tag.length; i++) {
        hash = tag.charCodeAt(i) + ((hash << 5) - hash)
    }
    return tagColors[Math.abs(hash) % tagColors.length]
}

const renderedReadme = computed(() => {
    return repoStore.repoReadme || ''
})

const handleCategoriesConfirm = async (
    categories: Array<{ id: number; name: string; color: string; count: number; checked: boolean }>,
) => {
    try {
        const checkedIds = categories.filter(c => c.checked).map(c => c.id)
        await api.category.updateRepoCategories({
            repo_id: relatedRepo.value.github_id,
            category_ids: checkedIds,
        })

        relatedRepo.value.categories = categories
            .filter(c => c.checked)
            .map(c => ({ id: c.id, name: c.name, color: c.color }))

        success('Category settings updated successfully')
    } catch (error) {
        console.error('Failed to update repo categories:', error)
        messageErr('Failed to update category settings')
    }
}

const fetchRepoCollections = async () => {
    if (!relatedRepo.value?.github_id) return
    try {
        const collections = await api.collection.getCollectionsByRepo({ github_id: relatedRepo.value.github_id })
        repoCollections.value = collections.map(c => ({ id: c.id, name: c.name, color: c.color || '#6b7280' }))
    } catch (error) {
        console.error('Failed to fetch repo collections:', error)
    }
}

onMounted(async () => {
    if (relatedRepo.value?.owner_login && relatedRepo.value?.name) {
        await repoStore.fetchRepoReadme(relatedRepo.value.owner_login, relatedRepo.value.name)
    }

    await fetchRepoCollections()
})

onUnmounted(() => {
    repoStore.clearReadme()
})
</script>

<template>
    <div ref="containerRef" class="flex h-full gap-4" :style="{ height: contentHeight + 'px' }">
        <div class="flex-1 overflow-y-auto bg-card rounded-md border-light">
            <div v-if="repoStore.isLoadingReadme" class="flex justify-center items-center py-20">
                <LoadingSpinner />
            </div>
            <div v-else-if="renderedReadme" class="markdown-body p-4">
                <div v-html="renderedReadme"></div>
            </div>
            <div v-else class="text-center py-20">
                <h3 class="text-xl font-bold text-t-placeholder mb-2">No README Found</h3>
                <p class="text-muted">This repository does not have a README file.</p>
            </div>
        </div>

        <div
            class="flex flex-col items-center gap-8 bg-card rounded-md border-light overflow-y-auto"
            style="width: 360px">
            <div class="p-5 w-full">
                <h3 class="font-bold text-t-primary mb-4 flex items-center gap-2">
                    <span class="i-md-topic text-lg"></span>
                    Repository Topics
                </h3>
                <div class="flex flex-wrap gap-2">
                    <span
                        v-for="topic in relatedRepo?.topics || []"
                        :key="topic"
                        :class="[
                            getTagColor(topic).bg,
                            getTagColor(topic).text,
                            'px-3 py-1 text-sm font-bold rounded-full',
                        ]"
                        >{{ topic }}</span
                    >
                </div>
            </div>

            <div class="p-5 w-full">
                <div class="font-bold text-t-primary mb-4 flex items-center gap-2">
                    <span class="i-md-local-offer text-lg"></span>
                    <span>Repository Tags</span>
                    <button class="primary dashed ml-auto" @click="openCategoriesDialog">
                        <span class="i-md-add"></span>
                        <span>New Tag</span>
                    </button>
                </div>
                <div class="flex flex-wrap gap-2">
                    <span
                        v-for="category in relatedRepo?.categories || []"
                        :key="category.id"
                        class="px-3 py-1 text-sm font-bold rounded-full"
                        :style="{ backgroundColor: category.color }"
                        >{{ category.name }}</span
                    >
                </div>
            </div>

            <div class="p-5 w-full">
                <h3 class="font-bold text-t-primary mb-4 flex items-center gap-2">
                    <span class="i-md-collection text-lg"></span>
                    Repository Collections
                </h3>
                <div class="flex flex-wrap gap-2">
                    <template v-if="repoCollections.length > 0">
                        <span
                            v-for="collection in repoCollections"
                            :key="collection.id"
                            class="px-3 py-1 text-sm font-bold rounded-full"
                            :style="{ backgroundColor: collection.color }"
                            >{{ collection.name }}</span
                        >
                    </template>
                    <span v-else class="text-sm text-t-placeholder">No collections</span>
                </div>
            </div>

            <div class="p-5 w-full">
                <h3 class="font-bold text-t-primary mb-4 flex items-center gap-2">
                    <span class="i-md-code text-lg"></span>
                    Languages
                </h3>
                <div v-if="parseLanguageJson(relatedRepo?.language).length > 0">
                    <div class="flex h-3 rounded-full overflow-hidden mb-3">
                        <template v-for="(lang, _index) in getLanguageBars(relatedRepo?.language)" :key="lang.language">
                            <div
                                class="h-full transition-all duration-300"
                                :style="{
                                    width: `${(lang.count / getTotalCount(relatedRepo?.language)) * 100}%`,
                                    backgroundColor: lang.color,
                                }"
                                :title="`${lang.language}: ${lang.count.toFixed(2)}%`"></div>
                        </template>
                    </div>
                    <div class="flex flex-col gap-2">
                        <span
                            v-for="(lang, _index) in getLanguageBars(relatedRepo?.language)"
                            :key="lang.language"
                            class="flex items-center gap-1.5">
                            <span class="w-2 h-2 rounded-full" :style="{ backgroundColor: lang.color }"></span>
                            <span>{{ lang.language }}: {{ lang.count.toFixed(2) }}%</span>
                        </span>
                    </div>
                </div>
                <div v-else class="text-sm text-t-placeholder">unknown</div>
            </div>

            <div class="p-5 w-full">
                <h3 class="font-bold text-t-primary mb-4 flex items-center gap-2">
                    <span class="i-md-activity text-lg"></span>
                    Repo Health
                </h3>

                <div class="space-y-3 relative z-10">
                    <div class="flex justify-between text-sm">
                        <span>Open Issues</span>
                        <span class="font-mono">{{ formatNumber(relatedRepo?.open_issues_count || 0) }}</span>
                    </div>
                </div>
            </div>
        </div>

        <CategoriesDialog
            v-model="showCategoriesDialog"
            :repo-id="repo?.github_id"
            :related-category-ids="repo?.categories?.map(c => c.id)"
            @close="showCategoriesDialog = false"
            @confirm="handleCategoriesConfirm" />
    </div>
</template>
