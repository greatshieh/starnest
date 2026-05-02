<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useMessage } from '@/composables/useMessage'
import { useShortcutStore } from '@/stores/shortcuts'

const shortcutStore = useShortcutStore()
const { success } = useMessage()

const editingId = ref<string | null>(null)
const recordingKeys = ref<string[]>([])
const isRecording = ref(false)

const startEdit = (id: string) => {
    editingId.value = id
    isRecording.value = true
    recordingKeys.value = []
}

const cancelEdit = () => {
    editingId.value = null
    isRecording.value = false
    recordingKeys.value = []
}

const handleKeyRecord = (e: KeyboardEvent) => {
    if (!isRecording.value) return

    e.preventDefault()

    const key =
        e.key === 'Control'
            ? 'Ctrl'
            : e.key === 'Meta'
              ? 'Cmd'
              : e.key === ' '
                ? 'Space'
                : e.key === 'ArrowUp'
                  ? 'Up'
                  : e.key === 'ArrowDown'
                    ? 'Down'
                    : e.key === 'ArrowLeft'
                      ? 'Left'
                      : e.key === 'ArrowRight'
                        ? 'Right'
                        : e.key

    if (!recordingKeys.value.includes(key)) {
        recordingKeys.value.push(key)
    }

    if (recordingKeys.value.length >= 3) {
        finishEdit()
    }
}

const handleKeyUp = (e: KeyboardEvent) => {
    if (!isRecording.value) return

    const key = e.key === 'Control' ? 'Ctrl' : e.key === 'Meta' ? 'Cmd' : e.key

    if (recordingKeys.value.includes(key)) {
        setTimeout(() => {
            if (isRecording.value && recordingKeys.value.length > 0) {
                finishEdit()
            }
        }, 100)
    }
}

const finishEdit = () => {
    if (editingId.value && recordingKeys.value.length > 0) {
        shortcutStore.updateShortcut(editingId.value, recordingKeys.value)
        success('Shortcut updated successfully')
    }
    editingId.value = null
    isRecording.value = false
    recordingKeys.value = []
}

const resetToDefault = () => {
    shortcutStore.resetToDefault()
    success('Shortcuts reset to default')
}

const saveShortcuts = () => {
    shortcutStore.saveShortcuts()
    success('Shortcuts saved successfully')
}

onMounted(() => {
    window.addEventListener('keydown', handleKeyRecord)
    window.addEventListener('keyup', handleKeyUp)
})

onUnmounted(() => {
    window.removeEventListener('keydown', handleKeyRecord)
    window.removeEventListener('keyup', handleKeyUp)
})
</script>

<template>
    <div class="flex-1 py-6 pl-4 overflow-y-auto">
        <h2 class="text-lg font-bold text-t-primary">Keyboard Shortcuts</h2>

        <Panel title="Global Shortcuts" description="Keyboard shortcuts for common actions">
            <div class="space-y-4">
                <div
                    v-for="shortcut in shortcutStore.shortcuts"
                    :key="shortcut.id"
                    class="flex items-center justify-between py-3 px-4 rounded-lg bg-popover border border-border">
                    <div class="flex-1">
                        <div class="font-medium text-t-primary">{{ shortcut.name }}</div>
                        <div class="text-sm text-t-placeholder">{{ shortcut.description }}</div>
                    </div>

                    <div class="flex items-center gap-3">
                        <div v-if="editingId === shortcut.id" class="flex items-center gap-2">
                            <span class="text-sm text-t-secondary">Press keys...</span>
                            <div class="flex gap-1">
                                <kbd
                                    v-for="(key, idx) in recordingKeys"
                                    :key="idx"
                                    class="px-2 py-1 text-sm bg-primary text-white rounded">
                                    {{ key }}
                                </kbd>
                            </div>
                            <button class="text-t-secondary hover:text-t-primary" @click="cancelEdit">
                                <span class="i-md-close"></span>
                            </button>
                        </div>

                        <div v-else class="flex items-center gap-2">
                            <div class="flex gap-1">
                                <kbd
                                    v-for="(key, idx) in shortcut.keys"
                                    :key="idx"
                                    class="px-2 py-1 text-sm bg-[var(--sv-btn-secondary-color)] text-t-primary rounded">
                                    {{ key }}
                                </kbd>
                            </div>
                            <button
                                v-if="shortcut.editable"
                                class="text-t-secondary hover:text-t-primary"
                                @click="startEdit(shortcut.id)">
                                <span class="i-md-edit"></span>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </Panel>

        <Panel title="Modifier Keys" description="Special keys used in shortcuts">
            <div class="grid grid-cols-2 gap-4 mt-4">
                <div class="flex items-center gap-3">
                    <kbd class="px-3 py-1 bg-[var(--sv-btn-secondary-color)] text-t-primary rounded">Ctrl</kbd>
                    <span class="text-t-secondary">Control key</span>
                </div>
                <div class="flex items-center gap-3">
                    <kbd class="px-3 py-1 bg-[var(--sv-btn-secondary-color)] text-t-primary rounded">Cmd</kbd>
                    <span class="text-t-secondary">Command key (macOS)</span>
                </div>
                <div class="flex items-center gap-3">
                    <kbd class="px-3 py-1 bg-[var(--sv-btn-secondary-color)] text-t-primary rounded">Shift</kbd>
                    <span class="text-t-secondary">Shift key</span>
                </div>
                <div class="flex items-center gap-3">
                    <kbd class="px-3 py-1 bg-[var(--sv-btn-secondary-color)] text-t-primary rounded">Alt</kbd>
                    <span class="text-t-secondary">Alt key</span>
                </div>
            </div>
        </Panel>

        <div class="flex justify-end mt-12 gap-8 pr-8">
            <button @click="resetToDefault">Reset to Default</button>
            <button class="primary" @click="saveShortcuts">Save Changes</button>
        </div>
    </div>
</template>
