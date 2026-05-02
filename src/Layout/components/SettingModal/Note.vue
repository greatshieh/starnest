<script setup lang="ts">
import { useSettingsStore } from '@/stores/settings'
import { open } from '@tauri-apps/plugin-dialog'

const settingsStore = useSettingsStore()

const autoSaveIntervals = [
    { label: '5 seconds', value: 5 },
    { label: '10 seconds', value: 10 },
    { label: '30 seconds', value: 30 },
    { label: '1 minute', value: 60 },
    { label: '5 minutes', value: 300 },
]

const editorModes = [
    { label: 'WYSIWYG', value: 'wsysiwyg' },
    { label: 'Instant Render', value: 'ir' },
    { label: 'Split View', value: 'sv' },
]

const browseFolder = async () => {
    try {
        const result = await open({
            title: 'Select Note Storage Folder',
            directory: true,
            multiple: false,
        })
        if (result) {
            settingsStore.noteStoragePath = result as string
        }
    } catch (error) {
        console.error('Failed to select folder:', error)
    }
}
</script>

<template>
    <div class="flex-1 py-6 pl-4">
        <h2 class="text-lg font-bold text-t-primary">编辑器设置</h2>

        <Panel inline title="Storage Path" description="Select the folder where notes are stored." columns="1fr 2fr">
            <MyInput v-model="settingsStore.noteStoragePath" :placeholder="settingsStore.noteStoragePath" type="text" />
            <button class="ml-8px" @click="browseFolder">Browse</button>
        </Panel>

        <Panel inline title="Auto Save" description="Automatically save notes as you type.">
            <SwitchToggle v-model="settingsStore.autoSave" class="ml-auto" />
        </Panel>

        <Panel v-if="settingsStore.autoSave" title="Auto Save Interval">
            <ButtonGroup
                class="ml-auto"
                :options="autoSaveIntervals"
                v-model="settingsStore.autoSaveInterval"
                :columns="5" />
        </Panel>

        <Panel title="Editor Mode">
            <ButtonGroup :options="editorModes" v-model="settingsStore.editorMode" :columns="3" class="ml-auto" />
        </Panel>

        <Panel title="Default Note Template">
            <textarea
                v-model="settingsStore.defaultNoteTemplate"
                class="w-full bg-card border-light rounded-md px-4 py-3 focus:outline-none transition-colors text-t-regular"
                rows="8"
                placeholder="# Notes for {{ repository_name }}&#10;&#10;## Summary&#10;&#10;## Key Features&#10;&#10;## Personal Notes"></textarea>
        </Panel>
    </div>
</template>
