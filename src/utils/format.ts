export function formatNumber(num: number): string {
    if (num >= 1000000) {
        return (num / 1000000).toFixed(1) + 'M'
    }
    if (num >= 1000) {
        return (num / 1000).toFixed(1) + 'k'
    }
    return num.toString()
}

export function formatDate(dateString: string): string {
    try {
        const date = new Date(dateString)
        const now = new Date()
        const diffMs = now.getTime() - date.getTime()
        const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24))

        if (diffDays === 0) return 'Today'
        if (diffDays === 1) return 'Yesterday'
        if (diffDays < 7) return `${diffDays} days ago`
        if (diffDays < 30) return `${Math.floor(diffDays / 7)} weeks ago`
        if (diffDays < 365) return `${Math.floor(diffDays / 30)} months ago`
        return `${Math.floor(diffDays / 365)} years ago`
    } catch {
        return dateString.substring(0, 10)
    }
}

export function formatTimeAgo(dateStr: string): string {
    const date = new Date(dateStr)
    const now = new Date()
    const diff = now.getTime() - date.getTime()
    const minutes = Math.floor(diff / 60000)
    const hours = Math.floor(diff / 3600000)
    const days = Math.floor(diff / 86400000)

    if (minutes < 60) return `${minutes}m ago`
    if (hours < 24) return `${hours}h ago`
    return `${days}d ago`
}

const colorSchemes = [
    { bg: 'bg-[#f6e6ff]', text: 'text-[#9333ea]', border: 'border-[#9333ea]/20' },
    { bg: 'bg-[#e2e7ff]', text: 'text-[#3b82f6]', border: 'border-[#3b82f6]/20' },
    { bg: 'bg-[#d1fae5]', text: 'text-[#10b981]', border: 'border-[#10b981]/20' },
    { bg: 'bg-[#fce7f3]', text: 'text-[#ec4899]', border: 'border-[#ec4899]/20' },
    { bg: 'bg-[#ffedd5]', text: 'text-[#f97316]', border: 'border-[#f97316]/20' },
    { bg: 'bg-[#cffafe]', text: 'text-[#06b6d4]', border: 'border-[#06b6d4]/20' },
    { bg: 'bg-[#fef3c7]', text: 'text-[#f59e0b]', border: 'border-[#f59e0b]/20' },
    { bg: 'bg-[#fee2e2]', text: 'text-[#ef4444]', border: 'border-[#ef4444]/20' },
    { bg: 'bg-[#e0e7ff]', text: 'text-[#6366f1]', border: 'border-[#6366f1]/20' },
    { bg: 'bg-[#d1fae5]', text: 'text-[#14b8a6]', border: 'border-[#14b8a6]/20' },
]

export function getCategoryColor(name: string): (typeof colorSchemes)[0] {
    let hash = 5381

    for (let i = 0; i < name.length; i++) {
        const char = name.charCodeAt(i)
        hash = ((hash << 5) + hash) ^ char
    }

    hash = hash * 1234567

    const index = Math.abs(hash) % colorSchemes.length

    return colorSchemes[index]
}

export function parseRepoTopics<T extends { topics?: string | string[] | null }>(repo: T): T & { topics: string[] } {
    return {
        ...repo,
        topics: repo.topics ? (typeof repo.topics === 'string' ? JSON.parse(repo.topics) : repo.topics) : [],
    }
}
