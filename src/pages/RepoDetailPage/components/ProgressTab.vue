<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getRepoEventsByDate, fetchRepoActivitiesFromGitHub } from '@/api/repo'
import { useNotification } from '@/composables/useNotification'
import { useMessage } from '@/composables/useMessage'
import type { DailyEventStats, Repository } from '@/types/api/repo'

const props = defineProps<{
    repo: Repository
}>()

const notification = useNotification()
const message = useMessage()
const events = ref<DailyEventStats[] | null>(null)
const isLoading = ref(false)
let loadingNotificationId: number | null = null

async function loadEvents() {
    try {
        const result = await getRepoEventsByDate({ repo_id: props.repo.github_id })
        events.value = Array.isArray(result) ? result : null
    } catch (error) {
        console.error('Failed to load events:', error)
        events.value = null
    }
}

async function fetchEventsFromGitHub() {
    loadingNotificationId = notification.info('Fetching events from GitHub...', { duration: 0 })
    isLoading.value = true

    try {
        await fetchRepoActivitiesFromGitHub({
            owner: props.repo.owner_login,
            repo_name: props.repo.name,
        })

        if (loadingNotificationId !== null) {
            notification.close(loadingNotificationId)
            loadingNotificationId = null
        }

        message.success('Events synced successfully')
        await loadEvents()
    } catch (error) {
        if (loadingNotificationId !== null) {
            notification.close(loadingNotificationId)
            loadingNotificationId = null
        }

        message.error('Failed to fetch repo events')
        console.error('Failed to fetch events from GitHub:', error)
    } finally {
        isLoading.value = false
    }
}

/**
 * 格式化日期为 "月/日 星期几" 格式
 *
 * @param dateStr - ISO 格式的日期字符串
 * @returns 格式化后的日期字符串，例如 "5/1 Fri"
 *
 * @example
 * formatDate('2024-05-01') // 返回 "5/1 Wed"
 */
function formatDate(dateStr: string): string {
    const date = new Date(dateStr)
    const weekdays = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat']
    const month = date.getMonth() + 1
    const day = date.getDate()
    const weekday = weekdays[date.getDay()]
    return `${month}/${day} ${weekday}`
}

function getEventTypeName(eventType: string): string {
    const names: Record<string, string> = {
        PushEvent: 'Push',
        ReleaseEvent: 'Release',
        CreateEvent: 'Create',
        DeleteEvent: 'Delete',
    }
    return names[eventType] || eventType
}

function getEventDisplayText(eventType: string, count: number): string {
    const name = getEventTypeName(eventType)
    if (count === 1) {
        return name
    }
    return `${count} ${name}s`
}

function getEventIcon(eventType: string): string {
    const icons: Record<string, string> = {
        PushEvent: 'i-md-git-push',
        ReleaseEvent: 'i-md-git-release',
        CreateEvent: 'i-md-git-create',
        DeleteEvent: 'i-md-git-delete',
    }
    return icons[eventType] || 'i-md-circle'
}

function getEventColor(eventType: string): string {
    const colors: Record<string, string> = {
        PushEvent: 'bg-[#18a058] text-white',
        ReleaseEvent: 'bg-[#7b9cf5] text-white',
        CreateEvent: 'bg-[#18a058] text-white',
        DeleteEvent: 'bg-[#d03050] text-white',
    }
    return colors[eventType] || 'bg-primary text-white'
}

onMounted(() => {
    loadEvents()
})
</script>

<template>
    <div class="p-6">
        <div class="flex justify-between items-center mb-8">
            <div class="flex items-center gap-3">
                <span class="i-md-activity text-2xl text-primary"></span>
                <h3 class="text-xl font-semibold text-[--sv-text-color-1]">Activity Timeline</h3>
            </div>
            <button
                class="flex items-center gap-2 px-4 py-2 rounded-lg transition-all duration-200"
                :class="
                    isLoading
                        ? 'bg-[--sv-placeholder-color-disabled] text-[--sv-text-color-disabled] cursor-not-allowed'
                        : 'bg-primary text-white hover:bg-[--sv-primary-hover-solid] active:bg-[--sv-primary-pressed-solid]'
                "
                :disabled="isLoading"
                @click="fetchEventsFromGitHub">
                <span v-if="isLoading" class="i-md-sync animate-spin"></span>
                <span>{{ isLoading ? 'Syncing...' : 'Sync from GitHub' }}</span>
            </button>
        </div>

        <div v-if="events && events.length > 0" class="relative">
            <div
                class="absolute left-8 top-0 bottom-0 w-0.5 bg-gradient-to-b from-primary via-[--sv-primary-hover-solid] to-[--sv-border-color]"></div>

            <div class="space-y-6">
                <div v-for="(day, index) in events" :key="day.date" class="relative flex gap-6">
                    <div class="relative z-10 flex flex-col items-center shrink-0">
                        <div
                            class="w-16 h-16 rounded-2xl flex flex-col items-center justify-center shadow-lg transition-transform duration-200 hover:scale-105"
                            :class="index === 0 ? 'bg-primary text-[#000] dark:text-white' : 'bg-card text-primary'">
                            <span class="text-lg font-medium">{{
                                new Date(day.date).toLocaleDateString('en-US', { weekday: 'short' })
                            }}</span>
                            <span class="text-2xl font-bold mt-1">{{ new Date(day.date).getDate() }}</span>
                        </div>
                    </div>

                    <div class="flex-1 pt-2">
                        <div class="bg-card rounded-xl p-4 mb-3 shadow-sm">
                            <div class="flex items-center gap-2 text-lg">
                                <span class="i-md-calendar text-t-secondary"></span>
                                <span class="font-medium">{{ formatDate(day.date) }}</span>
                                <span class="text-t-secondary"
                                    >({{ day.events.length }} event{{ day.events.length > 1 ? 's' : '' }})</span
                                >
                            </div>
                        </div>

                        <div class="grid gap-3">
                            <div
                                v-for="event in day.events"
                                :key="event.type"
                                class="flex items-center gap-4 p-4 rounded-xl bg-body hover:bg-hover transition-colors duration-200 border-light">
                                <div
                                    class="w-12 h-12 rounded-xl flex items-center justify-center shrink-0 shadow-md"
                                    :class="getEventColor(event.type)">
                                    <span :class="getEventIcon(event.type)" class="text-xl"></span>
                                </div>

                                <div class="flex-1 min-w-0">
                                    <div class="flex items-center gap-2 mb-1">
                                        <span class="font-semibold text-t-primary">{{
                                            getEventTypeName(event.type)
                                        }}</span>
                                        <span
                                            v-if="event.count > 1"
                                            class="px-2 py-1 font-medium rounded-md bg-primary text-[#000] dark:text-white">
                                            x{{ event.count }}
                                        </span>
                                    </div>
                                    <p class="text-t-secondary truncate">
                                        {{ getEventDisplayText(event.type, event.count) }}
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <div v-else-if="!isLoading" class="flex flex-col items-center justify-center py-16">
            <div class="w-24 h-24 rounded-full bg-[--sv-action-color] flex items-center justify-center mb-6">
                <span class="i-md-calendar-outline text-4xl text-[--sv-text-color-3]"></span>
            </div>
            <h4 class="text-lg font-semibold text-[--sv-text-color-1] mb-2">No Activity Data</h4>
            <p class="text-sm text-[--sv-text-color-3] text-center max-w-md mb-6">
                Start tracking this repository's activity by syncing with GitHub.
            </p>
            <button
                class="flex items-center gap-2 px-6 py-3 rounded-xl bg-primary text-white hover:bg-[--sv-primary-hover-solid] transition-colors duration-200 shadow-lg"
                @click="fetchEventsFromGitHub">
                <span class="i-md-sync"></span>
                <span>Sync from GitHub</span>
            </button>
        </div>

        <div v-else class="flex flex-col items-center justify-center py-16">
            <div class="w-16 h-16 rounded-full bg-[--sv-action-color] flex items-center justify-center mb-4">
                <span class="i-md-loader animate-spin text-2xl text-primary"></span>
            </div>
            <p class="text-sm text-[--sv-text-color-3]">Loading activity data...</p>
        </div>
    </div>
</template>
