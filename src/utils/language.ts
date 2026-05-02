import colorMap from '@/assets/color.json'

export interface LanguageData {
    language: string
    count: number
    color: string
}

export interface LanguageBarData extends LanguageData {
    percentage: number
}

export const parseLanguageJson = (language: string | null): LanguageData[] => {
    if (!language) {
        return []
    }
    try {
        const languages: Record<string, number> = JSON.parse(language)
        return Object.keys(languages).map(lang => ({
            language: lang,
            count: languages[lang],
            color: (colorMap as Record<string, string>)[lang] || '#ccc',
        }))
    } catch {
        return []
    }
}

export const getTopLanguages = (language: string | null, limit: number = 3): LanguageData[] => {
    const languages = parseLanguageJson(language)
    return languages.sort((a, b) => b.count - a.count).slice(0, limit)
}

export const getLanguageBars = (language: string | null): LanguageBarData[] => {
    const languages = parseLanguageJson(language)

    if (languages.length === 1 && Math.abs(languages[0].count - 100) < 0.01) {
        return [
            {
                ...languages[0],
                percentage: 100,
            },
        ]
    }

    const mainLanguages = languages.filter(lang => lang.count >= 1)
    const otherTotal = languages.filter(lang => lang.count < 1).reduce((sum, lang) => sum + lang.count, 0)

    const sorted = mainLanguages.sort((a, b) => b.count - a.count)

    const result: LanguageBarData[] = sorted.map(item => ({
        language: item.language,
        count: item.count,
        color: item.color,
        percentage: item.count,
    }))

    if (otherTotal > 0) {
        result.push({
            language: 'other',
            count: otherTotal,
            color: 'var(--sv-text-muted)',
            percentage: otherTotal,
        })
    }

    return result
}

export const getTotalCount = (language: string | null): number => {
    const languages = parseLanguageJson(language)
    return languages.reduce((sum, item) => sum + item.count, 0)
}

export const isSingleLanguage = (language: string | null): boolean => {
    const languages = parseLanguageJson(language)
    return languages.length === 1 && Math.abs(languages[0].count - 100) < 0.01
}
