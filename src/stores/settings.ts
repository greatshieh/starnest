import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { api } from '@/api'
import type { api as ApiTypes } from '@/types'

const STORAGE_KEY_SETTINGS = 'starnest_settings'

export const useSettingsStore = defineStore('settings', () => {
    const config = ref<ApiTypes.config.AppConfig>({
        version: '1.0.0',
        auto_save: true,
        auto_save_interval: 30,
        enable_markdown_preview: true,
        syntax_highlight: true,
        auto_sync: false,
        auto_sync_interval: 24,
        enable_debug: false,
        enable_analytics: true,
        enable_telemetry: true,
        max_concurrent_requests: 5,
        request_timeout: 30,
        cache_duration: 24,
    })

    const basePath = computed({
        get: () => config.value.base_path || '',
        set: (value: string) => {
            config.value.base_path = value || undefined
        },
    })

    const noteStoragePath = computed({
        get: () => config.value.note_path || '',
        set: (value: string) => {
            config.value.note_path = value || undefined
        },
    })

    const editorMode = computed({
        get: () => config.value.vditorMode || 'ir',
        set: (value: 'wysiwyg' | 'ir' | 'sv') => {
            config.value.vditorMode = value || 'ir'
        },
    })

    const autoSave = computed({
        get: () => config.value.auto_save,
        set: (value: boolean) => {
            config.value.auto_save = value
        },
    })

    const autoSaveInterval = computed({
        get: () => config.value.auto_save_interval || 30,
        set: (value: number) => {
            config.value.auto_save_interval = value
        },
    })

    const defaultNoteTemplate = computed({
        get: () => config.value.default_note_template || '',
        set: (value: string) => {
            config.value.default_note_template = value || undefined
        },
    })

    const enableMarkdownPreview = computed({
        get: () => config.value.enable_markdown_preview,
        set: (value: boolean) => {
            config.value.enable_markdown_preview = value
        },
    })

    const syntaxHighlight = computed({
        get: () => config.value.syntax_highlight,
        set: (value: boolean) => {
            config.value.syntax_highlight = value
        },
    })

    const autoSync = computed({
        get: () => config.value.auto_sync,
        set: (value: boolean) => {
            config.value.auto_sync = value
        },
    })

    const autoSyncInterval = computed({
        get: () => config.value.auto_sync_interval || 24,
        set: (value: number) => {
            config.value.auto_sync_interval = value
        },
    })

    const enableDebug = computed({
        get: () => config.value.enable_debug,
        set: (value: boolean) => {
            config.value.enable_debug = value
        },
    })

    const enableAnalytics = computed({
        get: () => config.value.enable_analytics,
        set: (value: boolean) => {
            config.value.enable_analytics = value
        },
    })

    const enableTelemetry = computed({
        get: () => config.value.enable_telemetry,
        set: (value: boolean) => {
            config.value.enable_telemetry = value
        },
    })

    const maxConcurrentRequests = computed({
        get: () => config.value.max_concurrent_requests || 5,
        set: (value: number) => {
            config.value.max_concurrent_requests = value
        },
    })

    const requestTimeout = computed({
        get: () => config.value.request_timeout || 30,
        set: (value: number) => {
            config.value.request_timeout = value
        },
    })

    const cacheDuration = computed({
        get: () => config.value.cache_duration || 24,
        set: (value: number) => {
            config.value.cache_duration = value
        },
    })

    function saveToLocalStorage(): void {
        try {
            localStorage.setItem(STORAGE_KEY_SETTINGS, JSON.stringify(config.value))
        } catch (error) {
            console.error('Failed to save settings to localStorage:', error)
        }
    }

    function loadFromLocalStorage(): boolean {
        try {
            const stored = localStorage.getItem(STORAGE_KEY_SETTINGS)
            if (stored) {
                const savedConfig = JSON.parse(stored)
                config.value = { ...config.value, ...savedConfig }
                return true
            }
        } catch (error) {
            console.error('Failed to load settings from localStorage:', error)
        }
        return false
    }

    async function loadConfig(): Promise<void> {
        const hasLocalData = loadFromLocalStorage()

        if (!hasLocalData) {
            try {
                const data = await api.config.getConfig()
                if (data && data.app) {
                    config.value = { ...config.value, ...data.app }
                    saveToLocalStorage()
                }
            } catch (error) {
                console.error('Failed to load config:', error)
            }
        }

        if (!config.value.default_note_template) {
            config.value.default_note_template = `# Notes for {{ repository_name }}

## Summary

## Key Features

## Personal Notes`
            saveToLocalStorage()
        }
    }

    async function saveConfig(): Promise<void> {
        try {
            await api.config.setAppConfig({ appConfig: config.value })
            saveToLocalStorage()
        } catch (error) {
            console.error('Failed to save config:', error)
            throw error
        }
    }

    async function updateConfig<K extends keyof ApiTypes.config.AppConfig>(
        key: K,
        value: ApiTypes.config.AppConfig[K],
    ): Promise<void> {
        config.value[key] = value
        await saveConfig()
    }

    async function resetConfig(): Promise<void> {
        try {
            await api.config.resetConfig()

            const data = await api.config.getConfig()
            if (data && data.app) {
                config.value = { ...config.value, ...data.app }
                if (!config.value.default_note_template) {
                    config.value.default_note_template = `# Notes for {{ repository_name }}

## Summary

## Key Features

## Personal Notes`
                }
                saveToLocalStorage()
            }
        } catch (error) {
            console.error('Failed to reset config:', error)
            throw error
        }
    }

    return {
        config,
        basePath,
        noteStoragePath,
        editorMode,
        autoSave,
        autoSaveInterval,
        defaultNoteTemplate,
        enableMarkdownPreview,
        syntaxHighlight,
        autoSync,
        autoSyncInterval,
        enableDebug,
        enableAnalytics,
        enableTelemetry,
        maxConcurrentRequests,
        requestTimeout,
        cacheDuration,
        loadConfig,
        saveConfig,
        updateConfig,
        resetConfig,
    }
})
