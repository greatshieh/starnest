import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { api } from '@/api'
import type { api as ApiTypes } from '@/types'

export type CollectionWithRepos = ApiTypes.collection.CollectionWithRepoCount & { repos: ApiTypes.repo.Repository[] }

export const useCollectionsStore = defineStore('collections', () => {
    const allRepos = ref<ApiTypes.repo.Repository[]>([])
    const collections = ref<CollectionWithRepos[]>([])
    const searchQuery = ref('')
    const pageSize = ref(100)

    const filteredCollections = computed(() => {
        if (!searchQuery.value.trim()) return collections.value
        const q = searchQuery.value.trim().toLowerCase()
        return collections.value.filter(c => c.name.toLowerCase().includes(q))
    })

    const totalRepos = computed(() => {
        return collections.value.reduce((sum, c) => sum + c.repo_count, 0)
    })

    async function loadRepos() {
        try {
            const result = await api.repo.getRepos({ page: 1, page_size: pageSize.value })
            allRepos.value = result.repos
        } catch (error) {
            console.error('Failed to load repos:', error)
        }
    }

    async function loadCollections() {
        try {
            const result = await api.collection.getAllCollections()
            collections.value = result.map(c => ({ ...c, repos: [] }))
        } catch (error) {
            console.error('Failed to load collections:', error)
        }
    }

    async function loadCollectionDetail(collectionId: number) {
        try {
            const repos = await api.collection.getReposByCollection({ collection_id: collectionId })
            const idx = collections.value.findIndex(c => c.id === collectionId)
            if (idx !== -1) {
                collections.value[idx] = { ...collections.value[idx], repos }
            }
            return repos
        } catch (error) {
            console.error('Failed to load collection repos:', error)
            return []
        }
    }

    async function saveCollection(data: { name: string; desc: string; color: string; repoIds: number[] }, collectionId?: number) {
        if (!data.name.trim()) return

        try {
            if (collectionId) {
                const updated = await api.collection.updateCollection({
                    collection_id: collectionId,
                    name: data.name.trim(),
                    description: data.desc.trim(),
                    color: data.color,
                })
                await api.collection.updateCollectionRepos({
                    collection_id: collectionId,
                    github_ids: data.repoIds,
                })
                const idx = collections.value.findIndex(c => c.id === collectionId)
                if (idx !== -1) {
                    collections.value[idx] = {
                        ...updated,
                        repos: allRepos.value.filter(r => data.repoIds.includes(r.github_id)),
                    }
                }
            } else {
                const created = await api.collection.createCollection({
                    name: data.name.trim(),
                    description: data.desc.trim(),
                    color: data.color,
                })
                await api.collection.updateCollectionRepos({
                    collection_id: created.id,
                    github_ids: data.repoIds,
                })
                collections.value.push({
                    ...created,
                    repo_count: data.repoIds.length,
                    repos: allRepos.value.filter(r => data.repoIds.includes(r.github_id)),
                })
            }
        } catch (error) {
            console.error('Failed to save collection:', error)
        }
    }

    async function deleteCollection(collectionId: number) {
        try {
            await api.collection.deleteCollection({ collection_id: collectionId })
            collections.value = collections.value.filter(c => c.id !== collectionId)
        } catch (error) {
            console.error('Failed to delete collection:', error)
        }
    }

    async function removeRepoFromCollection(collectionId: number, repoId: number) {
        try {
            const collection = collections.value.find(c => c.id === collectionId)
            if (!collection) return

            const remainingRepoIds = collection.repos.filter(r => r.github_id !== repoId).map(r => r.github_id)
            await api.collection.updateCollectionRepos({
                collection_id: collectionId,
                github_ids: remainingRepoIds,
            })

            const idx = collections.value.findIndex(c => c.id === collectionId)
            if (idx !== -1) {
                collections.value[idx] = {
                    ...collections.value[idx],
                    repos: collections.value[idx].repos.filter(r => r.github_id !== repoId),
                    repo_count: collections.value[idx].repo_count - 1,
                }
            }
        } catch (error) {
            console.error('Failed to remove repo from collection:', error)
        }
    }

    return {
        allRepos,
        collections,
        searchQuery,
        filteredCollections,
        totalRepos,
        loadRepos,
        loadCollections,
        loadCollectionDetail,
        saveCollection,
        deleteCollection,
        removeRepoFromCollection,
    }
})
