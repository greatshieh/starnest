<script setup lang="ts">
import { useSyncStore } from '@/stores/sync'
import { useRoute } from 'vue-router'

const route = useRoute()
const syncStore = useSyncStore()
</script>

<template>
    <div class="flex flex-col h-screen overflow-hidden bg-page">
        <!-- 顶部标题栏 -->
        <TiteBar />

        <!-- 主体区域：侧边栏 + 内容 -->
        <div class="flex flex-1 overflow-hidden">
            <!-- 左侧导航栏 -->
            <SideBar />

            <!-- 主内容区 -->
            <main ref="scrollContainerRef" class="flex-1 overflow-y-auto relative">
                <RouterView :key="route.fullPath">
                    <template #default="{ Component, route }">
                        <Transition name="fade-transform" mode="out-in" appear>
                            <KeepAlive>
                                <component :is="Component" :key="route.path" />
                            </KeepAlive>
                        </Transition>
                    </template>
                </RouterView>
            </main>
        </div>

        <!-- 底部状态栏 -->
        <StatusBar />

        <!-- Sync Progress Drawer -->
        <Transition
            enter-active-class="transition-all duration-300 ease-out"
            enter-from-class="translate-y-full opacity-0"
            enter-to-class="translate-y-0 opacity-100"
            leave-active-class="transition-all duration-200 ease-in"
            leave-from-class="translate-y-0 opacity-100"
            leave-to-class="translate-y-full opacity-0">
            <div
                append-to-body
                v-if="syncStore.isSyncing"
                class="fixed bottom-38px left-0 right-0 z-40 bg-popover shadow-lg p-4 px-8">
                <div class="mx-auto flex flex-col md:flex-row items-center justify-between gap-4">
                    <div class="flex items-center gap-4 text-t-primary">
                        <span class="i-md-sync w-10 h-10 rounded-full animate-spin"></span>
                        <div>
                            <p class="font-bold text-md tracking-wide">Syncing your GitHub repositories...</p>
                            <p class="text-base mt-4">Checking for new stars and metadata updates</p>
                        </div>
                    </div>
                    <div class="w-full md:w-96 flex items-center gap-4">
                        <div class="flex-1 h-3 bg-[var(--sv-base-color)] rounded-full overflow-hidden relative">
                            <div
                                class="absolute inset-y-0 left-0 bg-[var(--sv-success-solid)] rounded-full transition-all duration-500"
                                :style="{ width: `${syncStore.syncProgress}%` }"></div>
                            <div class="absolute inset-0 bg-[var(--sv-warning-solid)] rounded-full animate-pulse"></div>
                        </div>
                        <span class="text-t-primary font-black text-lg shrink-0">{{
                            syncStore.syncStage === 'fetching' ? syncStore.syncProgress + '%' : syncStore.syncStage
                        }}</span>
                    </div>
                </div>
            </div>
        </Transition>
    </div>
</template>

<style scoped>
@keyframes shimmer {
    0% {
        background-position: -200% 0;
    }
    100% {
        background-position: 200% 0;
    }
}
</style>
