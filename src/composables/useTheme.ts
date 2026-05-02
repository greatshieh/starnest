import { ref } from 'vue'
import { colorStates } from '@/utils/color'
import { getThemeColorByName } from '@/utils/theme'

const themeMode = ref(localStorage.getItem('starnest_theme_mode') || 'dark')
const themeColor = ref(localStorage.getItem('starnest_accent_name') || '极客蓝')

export function useTheme() {
    function initThemeColor() {
        document.documentElement.setAttribute('data-theme', themeMode.value)
        updateThemeColor(themeColor.value)
    }

    function updateThemeColor(themeName: string) {
        themeColor.value = themeName
        const baseColor = getThemeColorByName(themeName)
        const state = colorStates(baseColor)
        Object.entries(state).forEach(([key, value]) => {
            document.documentElement.style.setProperty(key, value)
        })
        localStorage.setItem('starnest_accent_name', themeName)
    }

    function updateThemeMode(mode: string) {
        themeMode.value = mode
        document.documentElement.setAttribute('data-theme', mode)
        localStorage.setItem('starnest_theme_mode', mode)
    }

    return {
        themeMode,
        themeColor,
        initThemeColor,
        updateThemeColor,
        updateThemeMode,
    }
}
