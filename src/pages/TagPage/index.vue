<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { useModal } from '@/composables/useModal'
import { useCategoriesStore } from '@/stores/categories'
import { useShortcutStore } from '@/stores/shortcuts'
import { SortBy, SortOrder } from '@/types/api/category'
import { presetColors } from '@/constants/colors'
import { useReposStore } from '@/stores/repos'
import { useRouter } from 'vue-router'
import { Repository } from '@/types/api/repo'
import { api } from '@/api'
import { useMessage } from '@/composables/useMessage'

const categoriesStore = useCategoriesStore()
const repoStore = useReposStore()
const router = useRouter()
const shortcutStore = useShortcutStore()

// ── Search & Sort ────────────────────────────────────────
const searchQuery = ref('')
const sortBy = ref<SortBy>(SortBy.COUNT)
const sortOrder = ref<SortOrder>(SortOrder.DESC)

// ── Sort Options ──────────────────────────────────────────
const sortOptions = [
    { value: SortBy.COUNT, label: 'Count' },
    { value: SortBy.NAME, label: 'Name' },
    { value: SortBy.UPDATED_AT, label: 'Updated At' },
]

// ── Modal State ──────────────────────────────────────────
const { isOpen: showFormModal, open: openFormModal, close: closeFormModal } = useModal()
const { isOpen: showDeleteModal, open: openDeleteModal, close: closeDeleteModal } = useModal()
const { isOpen: showRepoModal, open: openRepoModal, close: closeRepoModal } = useModal()
const editingTag = ref<{ id: number; name: string; color: string } | null>(null)
const deletingTag = ref<{ id: number; name: string; repoCount: number } | null>(null)

// Current tag info for repo modal
const viewingTag = ref<{ id: number; name: string; color: string } | null>(null)
const releatedRepo = ref<Repository[]>([])

// Form data
const formName = ref('')
const formColor = ref('#9333ea')

const message = useMessage()

// Update on search
function onSearchChange(value?: string | string[]) {
    if (value !== undefined) {
        searchQuery.value = Array.isArray(value) ? value[0] || '' : value
    }
    categoriesStore.setSearchKeyword(searchQuery.value)
    categoriesStore.fetchCategories()
}

// Sort change
function onSortChange() {
    categoriesStore.setSort(sortBy.value, sortOrder.value)
    categoriesStore.fetchCategories()
}

watch(() => sortBy.value, onSortChange)

// Toggle sort direction
function toggleSortOrder() {
    sortOrder.value = sortOrder.value === SortOrder.ASC ? SortOrder.DESC : SortOrder.ASC
    onSortChange()
}

// Open create modal
function openCreate() {
    editingTag.value = null
    formName.value = ''
    formColor.value = presetColors[0]
    openFormModal()
}

// Open edit modal
function openEdit(tag: { id: number; name: string; color: string }) {
    editingTag.value = { ...tag }
    formName.value = tag.name
    formColor.value = tag.color
    openFormModal()
}

// Save tag
async function saveTag() {
    if (!formName.value.trim()) return

    try {
        const oldName = editingTag.value?.name
        if (editingTag.value) {
            await categoriesStore.updateCategoryInfo(editingTag.value.id, formName.value.trim(), formColor.value)
            if (searchQuery.value === oldName) {
                searchQuery.value = formName.value.trim()
                categoriesStore.setSearchKeyword(searchQuery.value)
            }
        } else {
            await categoriesStore.addCategory(formName.value.trim(), formColor.value)
        }
        await categoriesStore.fetchCategories()
        closeFormModal()
    } catch (error) {
        message.error('Failed to save tag')
    }
}

// Open delete confirmation
function openDelete(tag: { id: number; name: string; repoCount: number }) {
    deletingTag.value = { ...tag }
    openDeleteModal()
}

// Confirm delete
async function confirmDelete() {
    if (deletingTag.value) {
        try {
            await categoriesStore.deleteCategory(deletingTag.value.id)
            await categoriesStore.fetchCategories()
            message.success('Tag deleted successfully')
            closeDeleteModal()
            deletingTag.value = null
        } catch (error) {
            message.error('Failed to delete tag')
        }
    }
}

// View repos
async function handleViewRepos(tag: { id: number; name: string; color: string }) {
    try {
        const response = await api.repo.getRepos({
            page: 1,
            page_size: 100,
            sort: 'stargazers_count',
            language: '',
            categories: [tag.name],
        })
        releatedRepo.value = response.repos.map(repoStore.parseRepoTopics)
    } catch (error) {
        message.error('Failed to fetch repos')
        releatedRepo.value = []
    }

    viewingTag.value = { ...tag }
    openRepoModal()
}

// Navigate to repo detail
function goToRepoDetail(repo: { owner_login: string; name: string }) {
    closeRepoModal()
    router.push({
        name: 'RepoDetail',
        params: {
            owner: repo.owner_login,
            name: repo.name,
        },
        state: {
            repo: JSON.stringify(repo),
        },
    })
}

// Shortcut handlers
function onKeydown(e: KeyboardEvent) {
    if (shortcutStore.matchesShortcut(e, 'close-modal')) {
        if (showDeleteModal.value) {
            closeDeleteModal()
            deletingTag.value = null
        } else if (showFormModal.value) {
            closeFormModal()
        }
    }

    if (shortcutStore.matchesShortcut(e, 'create-tag')) {
        e.preventDefault()
        openCreate()
    }
}

// Page change
function handlePageChange(page: number) {
    categoriesStore.setPage(page)
    categoriesStore.fetchCategories()
}

// Format time
function formatTime(dateStr: string): string {
    // 后端存储的是 UTC 时间，需要添加 'Z' 后缀让 JavaScript 正确解析
    const date = new Date(dateStr + 'Z')
    const now = new Date()
    const diff = now.getTime() - date.getTime()

    const minutes = Math.floor(diff / 60000)
    const hours = Math.floor(diff / 3600000)
    const days = Math.floor(diff / 86400000)
    const weeks = Math.floor(diff / 604800000)

    if (minutes < 1) return 'Just now'
    if (minutes < 60) return `${minutes} min ago`
    if (hours < 24) return `${hours} hours ago`
    if (days < 7) return `${days} days ago`
    if (weeks < 4) return `${weeks} weeks ago`
    return date.toLocaleDateString('en-US')
}

onMounted(async () => {
    document.addEventListener('keydown', onKeydown)
    await categoriesStore.fetchCategories()
})

onUnmounted(() => {
    document.removeEventListener('keydown', onKeydown)
})
</script>

<template>
    <div class="overflow-y-auto h-full">
        <!-- Toolbar -->
        <div class="mb-6 p-6 flex items-center justify-between shadow-md">
            <div>
                <h2 class="text-3xl text-t-primary font-semibold tracking-tight">All Tags</h2>
                <p class="mt-4">{{ categoriesStore.total }} repos</p>
            </div>
            <div class="flex items-center gap-2">
                <!-- Search -->
                <MySelect
                    v-model="searchQuery"
                    :options="categoriesStore.categories.map(tag => ({ label: tag.name, value: tag.name }))"
                    class="!w-220px"
                    clearable
                    @change="onSearchChange"></MySelect>

                <!-- Sort -->
                <MySelect v-model="sortBy" :options="sortOptions" class="!w-200px"></MySelect>

                <!-- Sort direction toggle -->
                <div
                    class="flex items-center justify-center p-2 rounded-md bg-card hover:bg-hover transition-colors cursor-pointer text-primary border-light"
                    @click="toggleSortOrder"
                    :title="sortOrder === 'asc' ? 'Switch to descending' : 'Switch to ascending'">
                    <span
                        class="text-[24px]"
                        :class="sortOrder === 'desc' ? 'i-md-descending' : 'i-md-ascending'"></span>
                </div>

                <!-- Create tag button -->
                <button class="primary dashed" @click="openCreate">
                    <span class="text-lg i-md-add"></span>
                    <span>New Tag</span>
                </button>
            </div>
        </div>

        <!-- Tag table -->
        <div class="rounded-md overflow-hidden m-6" style="border: 1px solid var(--sv-border-color)">
            <!-- Header -->
            <div
                class="grid items-center gap-2 px-4 font-medium text-lg h-[48px] bg-card text-t-primary border-b border-b-solid border-b-[var(--sv-border-color)]"
                style="grid-template-columns: 48px 1fr 120px 120px 180px">
                <span class="text-center">Color</span>
                <span class="text-center">Tag Name</span>
                <span class="text-center">Repo Count</span>
                <span class="text-center">Updated At</span>
                <span class="text-center">Actions</span>
            </div>

            <!-- Data rows -->
            <template v-if="categoriesStore.categories.length > 0">
                <div
                    v-for="(tag, index) in categoriesStore.categories"
                    :key="tag.id"
                    class="grid items-center gap-2 px-4 transition-colors h-[48px] border-b border-b-solid border-b-[var(--sv-border-color)]"
                    :style="{
                        gridTemplateColumns: '48px 1fr 120px 120px 180px',
                        background: index % 2 === 1 ? 'var(--sv-popover-color)' : 'var(--sv-card-color)',
                    }">
                    <!-- Color indicator -->
                    <span class="rounded-full size-[14px] text-center" :style="{ background: tag.color }"></span>
                    <!-- Tag name -->
                    <span class="truncate">{{ tag.name }}</span>
                    <!-- Repo count -->
                    <span class="text-center">{{ tag.repoCount }}</span>
                    <!-- Updated at -->
                    <span class="text-center">{{ formatTime(tag.updatedAt) }}</span>
                    <!-- Actions -->
                    <div class="flex items-center justify-center gap-3">
                        <button
                            class="primary plain"
                            title="Edit"
                            @click="openEdit({ id: tag.id, name: tag.name, color: tag.color })">
                            <span class="i-md-edit"></span>
                        </button>
                        <button
                            class="error plain"
                            title="Delete"
                            @click="openDelete({ id: tag.id, name: tag.name, repoCount: tag.repoCount })">
                            <span class="i-md-delete"></span>
                        </button>
                        <button class="info plain" title="View Repos" @click="handleViewRepos(tag)">
                            <span class="i-md-preview"></span>
                        </button>
                    </div>
                </div>
            </template>

            <!-- Empty state -->
            <div v-else class="py-12 text-center" style="color: var(--sv-placeholder-color)">
                <template v-if="categoriesStore.isLoading">Loading...</template>
                <template v-else>No matching tags found</template>
            </div>
        </div>

        <!-- Pagination -->
        <div class="flex items-center justify-between mt-4 px-1 mx-6">
            <span> {{ categoriesStore.total }} tags in total </span>
            <div class="flex items-center gap-1">
                <button
                    :disabled="categoriesStore.currentPage <= 1"
                    @click="handlePageChange(categoriesStore.currentPage - 1)">
                    <span class="i-md-arrow-back"></span>
                </button>
                <button
                    v-for="page in categoriesStore.totalPages"
                    :key="page"
                    :class="{ primary: page === categoriesStore.currentPage }"
                    @click="handlePageChange(page)">
                    {{ page }}
                </button>
                <button
                    :style="{ opacity: categoriesStore.currentPage >= categoriesStore.totalPages ? 0.4 : 1 }"
                    :disabled="categoriesStore.currentPage >= categoriesStore.totalPages"
                    @click="handlePageChange(categoriesStore.currentPage + 1)">
                    <span class="i-md-arrow-forward"></span>
                </button>
            </div>
        </div>

        <!-- Create/Edit tag modal -->
        <TagFormModal
            v-model:showFormModal="showFormModal"
            v-model:formName="formName"
            v-model:formColor="formColor"
            :editing-tag="editingTag"
            @save="saveTag" />

        <!-- Delete confirmation modal -->
        <TagDeleteModal
            v-model:showDeleteModal="showDeleteModal"
            :deleting-tag="deletingTag"
            @confirm="confirmDelete" />

        <!-- Repo list modal -->
        <RepoListModal
            v-model:showRepoModal="showRepoModal"
            :viewing-tag="viewingTag"
            :repos="releatedRepo"
            @goto-detail="goToRepoDetail" />
    </div>
</template>
