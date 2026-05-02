<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useAppStore } from '@/stores/apps'
import { useShortcutStore } from '@/stores/shortcuts'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { openUrl } from '@tauri-apps/plugin-opener'
import { convertFileSrc } from '@tauri-apps/api/core'
import { useAuthStore } from '@/stores/auth'
import { autoPlacement, autoUpdate, flip, offset, shift, useFloating } from '@floating-ui/vue'

defineOptions({
    name: 'TitleBar',
})

const appStore = useAppStore()
const authStore = useAuthStore()
const shortcutStore = useShortcutStore()

const sidebarToggleTooltip = ref('Toggle Sidebar')
const searchTooltip = ref('Search')
const isPinned = ref(false)
const showModal = ref(false)
const showSearchModal = ref(false)
const userContainer = ref<HTMLDivElement>()
const optionContainer = ref<HTMLDivElement>()
const showOption = ref(false)

function initTooltip() {
    const sidebarShortcut = appStore.isMacOs ? 'Cmd+B' : 'Ctrl+B'
    const searchShortcut = appStore.isMacOs ? 'Cmd+K' : 'Ctrl+K'

    sidebarToggleTooltip.value = `Toggle Sidebar (${sidebarShortcut})`
    searchTooltip.value = `Search (${searchShortcut})`
}

function minimizeWindow() {
    getCurrentWindow().minimize()
}

function fullscreenWindow() {
    getCurrentWindow().toggleMaximize()
}

function closeWindow() {
    getCurrentWindow().close()
}

async function togglePin() {
    const window = getCurrentWindow()
    if (isPinned.value) {
        await window.setAlwaysOnTop(false)
        isPinned.value = false
    } else {
        await window.setAlwaysOnTop(true)
        isPinned.value = true
    }
}

function showSearch() {
    showSearchModal.value = true
}

async function handleLogout() {
    showOption.value = false
    await authStore.logout()
}

function handleKeydown(event: KeyboardEvent): void {
    if (shortcutStore.matchesShortcut(event, 'search')) {
        event.preventDefault()
        showSearch()
        return
    }

    if (event.ctrlKey && event.key === 'b') {
        event.preventDefault()
        appStore.toggleSidebar()
    }
}

/**
 * Floating UI配置
 */
const { floatingStyles } = useFloating(userContainer, optionContainer, {
    whileElementsMounted: autoUpdate,
    middleware: [autoPlacement({ allowedPlacements: ['bottom'] }), offset(15)],
})

onMounted(() => {
    initTooltip()
    window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
    window.removeEventListener('keydown', handleKeydown)
})
</script>

<template>
    <div class="flex items-center h-38px px-3 shrink-0 bg-popover text-md" data-tauri-drag-region>
        <!-- 左侧：窗口控制按钮（红绿灯） -->
        <!-- <div class="flex items-center gap-1.5 mr-4">
            <span class="inline-block w-3 h-3 rounded-full" style="background: #ff5f57"></span>
            <span class="inline-block w-3 h-3 rounded-full" style="background: #febc2e"></span>
            <span class="inline-block w-3 h-3 rounded-full" style="background: #28c840"></span>
        </div> -->

        <div class="flex items-center gap-4">
            <!-- <nav class="flex items-center gap-4">
                <span class="cursor-pointer transition-colors">File</span>
                <span class="cursor-pointer transition-colors">Edit</span>
                <span class="cursor-pointer transition-colors">View</span>
                <span class="cursor-pointer transition-colors">Help</span>
            </nav> -->
            <div class="flex items-center gap-2 cursor-pointer" @click="() => openUrl(appStore.homepage)">
                <img src="@/assets/app-icon.png" alt="logo" class="w-[30px] rounded-lg" />
                <span class="text-t-primary">{{ appStore.title }}</span>
            </div>
            <!-- 搜索按钮 -->
            <div
                class="flex items-center gap-2 text-[var(--sv-btn-text-color)] hover:text-[var(--sv-btn-text-color-hover)] transition-colors"
                :title="searchTooltip"
                @click="showSearch">
                <span class="i-md-search"></span>
                <span>Search</span>
            </div>
        </div>

        <div class="ml-auto flex items-center gap-4">
            <!-- 侧边栏切换按钮 -->
            <div
                v-if="!appStore.sidebarCollapsed"
                class="i-md-sidebar-collapse size-4.5 cursor-pointer"
                @click="appStore.toggleSidebar"
                :title="sidebarToggleTooltip"></div>
            <div
                v-else
                class="i-md-sidebar-expand size-4.5 cursor-pointer"
                @click="appStore.toggleSidebar"
                :title="sidebarToggleTooltip"></div>

            <div class="i-md-setting size-4.5 cursor-pointer" @click="showModal = true"></div>

            <div ref="userContainer" class="flex items-center gap-2 cursor-pointer" @click="showOption = !showOption">
                <img
                    :src="convertFileSrc(authStore.user?.avatar?.replace('https://', '') || '')"
                    alt="avatar"
                    class="size-6 rounded-full" />
                <span>{{ authStore.user?.name }}</span>
                <ul
                    v-click-outside="() => (showOption = false)"
                    ref="optionContainer"
                    v-show="showOption"
                    class="bg-card z-999 rounded-md w-auto border-light"
                    :style="floatingStyles">
                    <li @click="showModal = true">账号设置</li>
                    <li class="border border-b-solid border-t-solid">切换账号</li>
                    <li @click="handleLogout">退出登录</li>
                </ul>
            </div>

            <div
                class="i-md-pin size-4.5 cursor-pointer"
                :class="isPinned ? 'rotate-0' : 'rotate-45'"
                @click="togglePin"></div>

            <div class="w-1px h-1em bg-[var(--sv-divider-color)] inline-block"></div>

            <div
                class="hover:bg-card hover:scale-[1.02] transition-all duration-200 p-1 rounded-md cursor-pointer"
                @click="minimizeWindow">
                <span class="i-md-minimize"></span>
            </div>
            <div
                class="hover:bg-card hover:scale-[1.02] transition-all duration-200 p-1 rounded-md cursor-pointer"
                @click="fullscreenWindow">
                <span class="i-md-fullscreen"></span>
            </div>
            <div
                class="hover:bg-card hover:scale-[1.02] transition-all duration-200 p-1 rounded-md cursor-pointer hover:text-error"
                @click="closeWindow">
                <span class="i-md-close"></span>
            </div>
        </div>

        <SettingModal v-model="showModal" />
        <SearchModal v-model="showSearchModal" />
    </div>
</template>

<style lang="css" scoped>
ul li {
    margin: 8px 12px;
    padding: 8px 0;
    cursor: pointer;
    /* margin-bottom: 4px; */
}
</style>
