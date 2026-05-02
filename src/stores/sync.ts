import { defineStore } from 'pinia'
import { ref } from 'vue'
import { listen, emit, UnlistenFn } from '@tauri-apps/api/event'
import { api } from '@/api'
import { useMessage } from '@/composables/useMessage'
import { useAuthStore } from './auth'
import { useReposStore } from './repos'

const STORAGE_KEY = 'starnest_last_sync'

export const useSyncStore = defineStore('sync', () => {
    const message = useMessage()
    const authStore = useAuthStore()
    const reposStore = useReposStore()
    const isSyncing = ref(false)
    const syncProgress = ref(0)
    const syncError = ref<string | null>(null)
    const syncStage = ref<'fetching' | 'saving' | 'completed'>('completed')
    const lastSync = ref<string | null>(null)

    let unlistenFn: UnlistenFn | null = null

    async function sync(): Promise<void> {
        if (isSyncing.value) return

        isSyncing.value = true
        syncProgress.value = 0
        syncError.value = null
        syncStage.value = 'fetching'

        if (!authStore.token) {
            message.error('No token available')
            isSyncing.value = false
            return
        }

        try {
            unlistenFn = await listen('sync_progress', event => {
                const data = event.payload as {
                    stage: 'fetching' | 'saving'
                    current: number
                    total: number
                    percentage: number
                }
                syncStage.value = data.stage
                syncProgress.value = data.percentage
            })

            const syncResult = await api.repo.syncRepos({ token: authStore.token })

            syncProgress.value = 100
            syncStage.value = 'completed'
            lastSync.value = syncResult.last_sync
            localStorage.setItem(STORAGE_KEY, lastSync.value)

            message.success('Sync completed')

            reposStore.resetPagination()
            await reposStore.fetchRepos()
        } catch (error) {
            syncError.value = error instanceof Error ? error.message : 'Sync failed'
            message.error('Sync failed')
            console.error('Sync failed:', error)
        } finally {
            unlistenFn?.()
            unlistenFn = null
            isSyncing.value = false
            syncProgress.value = 0
            syncStage.value = 'completed'
        }
    }

    async function cancelSync(): Promise<void> {
        await emit('cancel_sync')
    }

    function loadLastSync(): void {
        try {
            const stored = localStorage.getItem(STORAGE_KEY)
            if (stored) {
                lastSync.value = stored
            }
        } catch (error) {
            console.error('Failed to load last sync:', error)
        }
    }

    return {
        isSyncing,
        syncProgress,
        syncError,
        syncStage,
        lastSync,
        sync,
        cancelSync,
        loadLastSync,
    }
})
