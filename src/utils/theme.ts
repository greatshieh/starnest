import { themeColors } from '@/constants/colors'

export function getThemeByName(name: string) {
    return themeColors.find(t => t.name === name)
}

export function getThemeColorByName(name: string): string {
    const theme = getThemeByName(name)
    return theme?.color || '#2f54eb'
}