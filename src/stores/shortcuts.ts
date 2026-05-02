import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface ShortcutConfig {
    id: string
    name: string
    description: string
    keys: string[]
    editable: boolean
}

const STORAGE_KEY = 'starnest_shortcuts'

const defaultShortcuts: ShortcutConfig[] = [
    { id: 'search', name: 'Search', description: 'Open search modal', keys: ['Ctrl', 'K'], editable: true },
    {
        id: 'create-collection',
        name: 'Create Collection',
        description: 'Create a new collection',
        keys: ['Ctrl', 'N'],
        editable: true,
    },
    { id: 'create-tag', name: 'Create Tag', description: 'Create a new tag', keys: ['Ctrl', 'N'], editable: true },
    {
        id: 'create-note',
        name: 'Create Note',
        description: 'Create a new note',
        keys: ['Ctrl', 'Shift', 'N'],
        editable: true,
    },
    { id: 'close-modal', name: 'Close Modal', description: 'Close current modal', keys: ['Escape'], editable: false },
]

export const useShortcutStore = defineStore('shortcuts', () => {
    const shortcuts = ref<ShortcutConfig[]>([])

    function initShortcuts() {
        try {
            const saved = localStorage.getItem(STORAGE_KEY)
            if (saved) {
                shortcuts.value = JSON.parse(saved)
                const validIds = defaultShortcuts.map(s => s.id)
                const filtered = shortcuts.value.filter(s => validIds.includes(s.id))
                if (filtered.length !== shortcuts.value.length) {
                    shortcuts.value = filtered
                    saveShortcuts()
                }
            } else {
                shortcuts.value = JSON.parse(JSON.stringify(defaultShortcuts))
            }
        } catch {
            shortcuts.value = JSON.parse(JSON.stringify(defaultShortcuts))
        }
    }

    function updateShortcut(id: string, keys: string[]) {
        const index = shortcuts.value.findIndex(s => s.id === id)
        if (index !== -1) {
            const newShortcuts = [...shortcuts.value]
            newShortcuts[index] = { ...newShortcuts[index], keys: [...keys] }
            shortcuts.value = newShortcuts
            saveShortcuts()
        }
    }

    function resetToDefault() {
        shortcuts.value = JSON.parse(JSON.stringify(defaultShortcuts))
        saveShortcuts()
    }

    function saveShortcuts() {
        localStorage.setItem(STORAGE_KEY, JSON.stringify(shortcuts.value))
    }

    function getShortcutKeys(id: string): string[] {
        return shortcuts.value.find(s => s.id === id)?.keys || []
    }

    function matchesShortcut(event: KeyboardEvent, shortcutId: string): boolean {
        const shortcut = shortcuts.value.find(s => s.id === shortcutId)
        if (!shortcut) return false

        const keys = shortcut.keys

        if (keys.length === 1 && keys[0] === 'Escape') {
            return event.key === 'Escape'
        }

        const ctrlKey = keys.includes('Ctrl') || keys.includes('Cmd')
        const shiftKey = keys.includes('Shift')
        const altKey = keys.includes('Alt')
        const mainKey = keys.find(k => !['Ctrl', 'Cmd', 'Shift', 'Alt'].includes(k))

        if (!mainKey) return false

        const isCtrlPressed = ctrlKey && (event.ctrlKey || event.metaKey)
        const isShiftPressed = shiftKey && event.shiftKey
        const isAltPressed = altKey && event.altKey
        const keyMatches = event.key.toLowerCase() === mainKey.toLowerCase()

        return isCtrlPressed === ctrlKey && isShiftPressed === shiftKey && isAltPressed === altKey && keyMatches
    }

    return {
        shortcuts,
        initShortcuts,
        updateShortcut,
        resetToDefault,
        saveShortcuts,
        getShortcutKeys,
        matchesShortcut,
    }
})
