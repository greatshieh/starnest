<script setup lang="ts">
// 合集管理页
import { ref, onMounted, onUnmounted } from 'vue'
import { useCollectionsStore, type CollectionWithRepos } from '@/stores/collections'
import { useShortcutStore } from '@/stores/shortcuts'
import CollectionCard from './components/CollectionCard.vue'
import CollectionFormModal from './components/CollectionFormModal.vue'
import CollectionDetailModal from './components/CollectionDetailModal.vue'
import CollectionDeleteModal from './components/CollectionDeleteModal.vue'

const collectionStore = useCollectionsStore()
const shortcutStore = useShortcutStore()

// ── 弹窗状态 ──────────────────────────────────────────
const showFormModal = ref(false)
const showDetailModal = ref(false)
const showDeleteModal = ref(false)
const editingCollection = ref<CollectionWithRepos | null>(null)
const viewingCollection = ref<CollectionWithRepos | null>(null)
const deletingCollection = ref<CollectionWithRepos | null>(null)

// ── 新建 ──────────────────────────────────────────────
function openCreate() {
    editingCollection.value = null
    showFormModal.value = true
}

// ── 编辑 ──────────────────────────────────────────────
function openEdit(collection: CollectionWithRepos) {
    editingCollection.value = collection
    showFormModal.value = true
}

// ── 保存 ──────────────────────────────────────────────
async function handleSaveCollection(data: { name: string; desc: string; color: string; repoIds: number[] }) {
    await collectionStore.saveCollection(data, editingCollection.value?.id)
    showFormModal.value = false
}

// ── 查看详情 ──────────────────────────────────────────
async function openDetail(collection: CollectionWithRepos) {
    await collectionStore.loadCollectionDetail(collection.id)
    const updated = collectionStore.collections.find(c => c.id === collection.id)
    viewingCollection.value = updated || collection
    showDetailModal.value = true
}

// ── 删除 ──────────────────────────────────────────────
function openDelete(collection: CollectionWithRepos) {
    deletingCollection.value = collection
    showDeleteModal.value = true
}

async function confirmDelete() {
    if (!deletingCollection.value) return
    await collectionStore.deleteCollection(deletingCollection.value.id)
    showDeleteModal.value = false
    deletingCollection.value = null
}

// ── 从详情中移除仓库 ──────────────────────────────────
async function removeRepoFromCollection(repoId: number) {
    if (!viewingCollection.value) return
    await collectionStore.removeRepoFromCollection(viewingCollection.value.id, repoId)
    // 更新 viewingCollection
    const updated = collectionStore.collections.find(c => c.id === viewingCollection.value!.id)
    if (updated) {
        viewingCollection.value = updated
    }
}

// ── 从详情页编辑合集 ──────────────────────────────────
function handleEditFromDetail() {
    showDetailModal.value = false
    if (viewingCollection.value) {
        openEdit(viewingCollection.value)
    }
}

// ── 从详情页删除合集 ──────────────────────────────────
function handleDeleteFromDetail() {
    showDetailModal.value = false
    if (viewingCollection.value) {
        openDelete(viewingCollection.value)
    }
}

// ── Escape 关闭弹窗 ──────────────────────────────────
function onKeydown(e: KeyboardEvent) {
    if (shortcutStore.matchesShortcut(e, 'close-modal')) {
        if (showDeleteModal.value) {
            showDeleteModal.value = false
            deletingCollection.value = null
        } else if (showDetailModal.value) {
            showDetailModal.value = false
            viewingCollection.value = null
        } else if (showFormModal.value) {
            showFormModal.value = false
        }
    }

    if (shortcutStore.matchesShortcut(e, 'create-collection')) {
        e.preventDefault()
        openCreate()
    }
}

onMounted(() => {
    document.addEventListener('keydown', onKeydown)
    collectionStore.loadRepos()
    collectionStore.loadCollections()
})

onUnmounted(() => {
    document.removeEventListener('keydown', onKeydown)
})
</script>

<template>
    <div class="overflow-y-auto h-full">
        <div class="p-6 flex items-center justify-between mb-5 shadow-md">
            <div>
                <h1 class="text-lg font-semibold text-fg">
                    Collection
                    <span class="text-xs font-normal ml-2 text-muted">
                        {{ collectionStore.collections.length }} Collection · {{ collectionStore.totalRepos }} Repos
                    </span>
                </h1>
            </div>
            <div class="flex items-center gap-2">
                <!-- 搜索 -->
                <MyInput v-model="collectionStore.searchQuery" placeholder="Search Collection..." class="!w-[180px]" />

                <!-- 新建合集按钮 -->
                <button class="primary dashed" @click="openCreate">
                    <span class="text-lg i-md-add"></span>
                    <span>New Collection</span>
                </button>
            </div>
        </div>

        <CollectionCard :collections="collectionStore.filteredCollections" @click="openDetail" />

        <!-- 新建/编辑合集弹窗 -->
        <CollectionFormModal
            v-model:show="showFormModal"
            :editing-collection="editingCollection"
            :all-repos="collectionStore.allRepos"
            @save="handleSaveCollection" />

        <!-- 合集详情弹窗 -->
        <CollectionDetailModal
            v-model:show="showDetailModal"
            :collection="viewingCollection"
            @edit="handleEditFromDetail"
            @delete="handleDeleteFromDetail"
            @remove-repo="removeRepoFromCollection" />

        <!-- 删除确认弹窗 -->
        <CollectionDeleteModal
            v-model:show="showDeleteModal"
            :collection="deletingCollection"
            @confirm="confirmDelete" />
    </div>
</template>
