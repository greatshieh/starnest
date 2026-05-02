<script setup lang="ts">
import { ref, watch } from 'vue'
import { useNotesStore } from '@/stores/notes'
import { useMessage } from '@/composables/useMessage'
import { useSettingsStore } from '@/stores/settings'

const visible = defineModel({
    type: Boolean,
    default: false,
})

const notesStore = useNotesStore()
const settingsStore = useSettingsStore()
const { success, error } = useMessage()

const noteTitle = ref('')
const noteContent = ref('')

watch(visible, (newVal) => {
    if (newVal) {
        noteTitle.value = ''
        noteContent.value = settingsStore.defaultNoteTemplate || ''
    }
})

const handleSave = async () => {
    if (!noteTitle.value.trim()) {
        error('Please enter a note title')
        return
    }

    notesStore.addNote({
        title: noteTitle.value.trim(),
        content: noteContent.value,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
    })

    success('Note created successfully')
    visible.value = false
}

const handleCancel = () => {
    visible.value = false
}
</script>

<template>
    <div v-if="visible" class="fixed inset-0 z-50 flex items-center justify-center">
        <div class="absolute inset-0 bg-black/50" @click="handleCancel"></div>
        <div class="relative bg-card rounded-xl shadow-2xl w-full max-w-2xl max-h-[80vh] flex flex-col">
            <div class="flex items-center justify-between px-6 py-4 border-b border-border">
                <h2 class="text-lg font-bold text-t-primary">Create New Note</h2>
                <button class="text-t-secondary hover:text-t-primary" @click="handleCancel">
                    <span class="i-md-close"></span>
                </button>
            </div>

            <div class="flex-1 overflow-y-auto p-6">
                <div class="mb-4">
                    <label class="block text-sm font-medium text-t-secondary mb-2">Title</label>
                    <input
                        v-model="noteTitle"
                        type="text"
                        class="w-full px-4 py-2 bg-popover border border-border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary/50"
                        placeholder="Enter note title"
                        autofocus
                    />
                </div>

                <div>
                    <label class="block text-sm font-medium text-t-secondary mb-2">Content</label>
                    <textarea
                        v-model="noteContent"
                        rows="10"
                        class="w-full px-4 py-2 bg-popover border border-border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary/50 resize-none"
                        placeholder="Start writing your note..."
                    ></textarea>
                </div>
            </div>

            <div class="flex justify-end gap-3 px-6 py-4 border-t border-border">
                <button class="secondary" @click="handleCancel">Cancel</button>
                <button class="primary" @click="handleSave">Create Note</button>
            </div>
        </div>
    </div>
</template>