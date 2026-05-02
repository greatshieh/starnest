<script setup lang="ts">
import { computed, ref } from 'vue'
import { useSyncStore } from '@/stores/sync'
import { useAppStore } from '@/stores/apps'

defineOptions({
    name: 'SideBar',
})

const syncStore = useSyncStore()
const appStore = useAppStore()
const showSyncModal = ref(false)
const isCollapsed = computed(() => appStore.sidebarCollapsed)

const handleSync = async () => {
    showSyncModal.value = true
    await syncStore.sync()
}
</script>

<template>
    <aside class="flex flex-col shrink-0 bg-popover px-2" :class="isCollapsed ? '!w-16' : '!w-64'">
        <MenuNav :isCollapsed="isCollapsed" />

        <!-- 底部同步按钮 -->
        <button
            :class="[
                isCollapsed ? 'rounded-full w-8 p-0' : 'w-full',
                'sticky',
                'bottom-2',
                'primary',
                'mx-auto',
                'text-lg',
            ]"
            title="Sync"
            @click="handleSync">
            <span :class="[{ 'text-2xl': isCollapsed }, 'i-md-sync']"></span>
            <span :class="isCollapsed ? 'hidden' : 'w-auto'">Sync</span>
        </button>
    </aside>
</template>
