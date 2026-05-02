import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { useTheme } from '@/composables/useTheme'

export const useAppStore = defineStore('app', () => {
    const title = 'StarNest'
    const homepage = 'https://github.com/greatshieh/starnest'

    const viewModes = [
        { name: 'list', icon: 'i-md-list' },
        { name: 'grid', icon: 'i-md-grid' },
    ]

    const sidebarCollapsed = ref(false)

    const savedViewMode = localStorage.getItem('starnest_view_mode') || 'grid'
    const viewMode = ref(savedViewMode)

    const platform = navigator.platform.toLowerCase()
    const isMacOs = computed(() => platform.includes('mac'))

    const { themeMode, themeColor, updateThemeMode, updateThemeColor, initThemeColor } = useTheme()
    initThemeColor()

    function toggleSidebar() {
        sidebarCollapsed.value = !sidebarCollapsed.value
    }

    function setViewMode(mode: string) {
        viewMode.value = mode
        localStorage.setItem('starnest_view_mode', mode)
    }

    return {
        title,
        homepage,
        sidebarCollapsed,
        toggleSidebar,
        viewMode,
        viewModes,
        setViewMode,
        isMacOs,
        themeMode,
        themeColor,
        updateThemeMode,
        updateThemeColor,
    }
})
