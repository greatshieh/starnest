<script setup lang="ts">
import NoteEditor from '@/components/NoteEditor.vue'
import { useMessage } from '@/composables/useMessage'
import { useSettingsStore } from '@/stores/settings'
import { useShortcutStore } from '@/stores/shortcuts'
import { Repository } from '@/types/api/repo'
import { formatDate } from '@/utils/format'
import { invoke } from '@tauri-apps/api/core'
import { computed, onMounted, ref, shallowRef } from 'vue'
import { useWindowResize } from '@/composables/useWindowResize'
import { useModal } from '@/composables/useModal'

defineOptions({
    name: 'NoteTab',
})

const props = defineProps({
    repo: {
        type: Object as () => Repository,
        default: () => {},
    },
})

const settingStore = useSettingsStore()
const shortcutStore = useShortcutStore()

const containerRef = ref<HTMLDivElement>()
const { height: windowHeight } = useWindowResize()

const relatedRepo = ref<Repository>(props.repo)

const noteEditorRef = shallowRef<InstanceType<typeof NoteEditor> | null>(null)
const noteTitle = ref('')
const noteContent = ref(settingStore.defaultNoteTemplate)
const noteId = ref(0)
const isSaving = ref(false)
const isLoadingNotes = ref(false)
const { isOpen: showConfirmDialog, open: openConfirmDialog, close: closeConfirmDialog } = useModal()
const relatedNotes = ref<
    Array<{ id: number; note_name: string; folder: string; created_at: string; updated_at: string }>
>([])

const processedNotes = computed(() => {
    return relatedNotes.value.map(note => ({
        ...note,
        displayName: note.note_name.replace('.md', ''),
        formattedUpdatedAt: formatDate(note.updated_at),
        formattedCreatedAt: formatDate(note.created_at),
    }))
})

const { success, warning } = useMessage()

const doSaveNote = async (noteName: string) => {
    isSaving.value = true
    try {
        const content = getNoteContent()
        await invoke('cmd_save_note', {
            id: noteId.value,
            github_id: relatedRepo.value.github_id,
            owner: relatedRepo.value.owner_login,
            repo_name: relatedRepo.value.name,
            note_name: noteName,
            content,
        })
        cancelNote(noteName)
        await fetchNotes()
        success(`Note "${noteName}" saved successfully`)
    } catch (error) {
        console.error('Failed to save note:', error)
        alert(`Failed to save note: ${error}`)
    } finally {
        isSaving.value = false
    }
}

const showNote = async (name: string, id: number = 0) => {
    try {
        noteContent.value = await invoke('cmd_read_note', {
            owner: relatedRepo.value.owner_login,
            repo_name: relatedRepo.value.name,
            note_name: name,
        })
        noteId.value = id
        noteTitle.value = name.replace('.md', '')
    } catch (error) {
        console.error('Failed to read note:', error)
    }
}

const cancelNote = (name: string) => {
    noteTitle.value = name
}

const getNoteContent = (): string => {
    if (!noteEditorRef.value) return ''
    return noteEditorRef.value.getValue()
}

const saveNote = async () => {
    const trimmedTitle = noteTitle.value.trim()

    if (!trimmedTitle) {
        openConfirmDialog()
        return
    }

    await doSaveNote(trimmedTitle)
}

const cancelSaveNote = () => {
    closeConfirmDialog()
    warning('Save operation cancelled')
}

const confirmSaveWithDefaultName = async () => {
    closeConfirmDialog()
    const defaultName = (await invoke('note_get_default_name', {
        owner: relatedRepo.value.owner_login,
        repo_name: relatedRepo.value.name,
    })) as string
    await doSaveNote(defaultName)
}

const confirmSaveWithHeading = async () => {
    closeConfirmDialog()
    const content = getNoteContent()
    const heading = extractFirstHeading(content)
    if (heading) {
        await doSaveNote(heading)
    }
}

const addNote = (name: string) => {
    noteTitle.value = name
    noteId.value = 0
    if (name === '') {
        noteContent.value = ''
    }
}

const fetchNotes = async () => {
    isLoadingNotes.value = true
    try {
        const notes = (await invoke('cmd_get_notes_by_repo', {
            github_id: relatedRepo.value.github_id,
        })) as Array<{ id: number; note_name: string; folder: string; created_at: string; updated_at: string }>
        relatedNotes.value = notes
    } catch (error) {
        console.error('Failed to fetch notes:', error)
        relatedNotes.value = []
    } finally {
        isLoadingNotes.value = false
    }
}

onMounted(async () => {
    noteContent.value = noteContent.value.replace('{{ repository_name }}', relatedRepo.value.name)
    await fetchNotes()
})

const extractFirstHeading = (content: string): string | null => {
    const lines = content.split('\n')
    for (const line of lines) {
        const trimmed = line.trim()
        if (trimmed.startsWith('# ') && trimmed.length > 2) {
            return trimmed.slice(2).trim()
        }
    }
    return null
}

const handleKeydown = (event: KeyboardEvent) => {
    if (shortcutStore.matchesShortcut(event, 'create-note')) {
        event.preventDefault()
        addNote('')
    }
}

const contentHeight = computed(() => {
    if (!containerRef.value) return 0
    return windowHeight.value - containerRef.value.getBoundingClientRect().top - 2 * 32
})
</script>

<template>
    <div
        class="grid gap-4"
        :style="{ height: contentHeight + 'px' }"
        style="grid-template-columns: 360px 1fr"
        ref="containerRef">
        <!-- 笔记列表 -->
        <div
            class="flex flex-col items-center gap-4 bg-card rounded-md border-1px border-solid border-[var(--sv-border-color)] p-5">
            <div class="font-bold text-t-primary text-lg flex items-center gap-2 w-full">
                <span class="i-md-note text-lg"></span>
                <span>Related Notes</span>
                <button class="ml-auto primary dashed" @click="addNote('')">
                    <span class="i-md-add"></span>
                    <span>New Note</span>
                </button>
                <span class="font-bold tracking-tighter ml-4">{{ relatedNotes.length }} Notes</span>
            </div>

            <div v-if="isLoadingNotes" class="flex justify-center items-center py-8">
                <LoadingSpinner size="sm" />
            </div>
            <div v-else-if="!isLoadingNotes && relatedNotes.length === 0">No notes found.</div>
            <div
                v-else
                v-for="(note, index) in processedNotes"
                :key="note.note_name"
                class="cursor-pointer p-4 w-full border-[var(--sv-border-color)] rounded-md"
                @click="showNote(note.note_name, note.id)">
                <div class="flex justify-between items-start mb-1">
                    <h4 class="font-bold text-primary truncate">
                        {{ note.displayName }}
                    </h4>
                    <span class="flex-shrink-0 ml-2">
                        {{ note.formattedUpdatedAt }}
                    </span>
                </div>
                <p class="text-sm leading-relaxed mt-2">Note created on {{ note.formattedCreatedAt }}</p>
                <div v-if="index < processedNotes.length - 1" class="h-px mt-4"></div>
            </div>

            <button v-if="relatedNotes.length > 0" class="primary">View All Notes</button>
        </div>

        <!-- 笔记编辑区 -->
        <div
            class="bg-card rounded-md border-1px border-solid border-[var(--sv-border-color)] p-8 h-full flex flex-col gap-4">
            <div class="grid items-center gap-4" style="grid-template-columns: 1fr auto">
                <input
                    v-model="noteTitle"
                    type="text"
                    class="text-xl font-bold text-t-primary"
                    placeholder="Add Note - input note title here" />
                <button class="primary" :disabled="isSaving" @click="saveNote">
                    <span v-if="isSaving" class="flex items-center gap-2">
                        <span class="i-md-loader animate-spin"></span>
                        Saving...
                    </span>
                    <span v-else>Save Note</span>
                </button>
            </div>

            <KeepAlive>
                <NoteEditor class="flex-1" :noteContent="noteContent" ref="noteEditorRef" />
            </KeepAlive>
        </div>

        <!-- Confirm Dialog -->
        <Modal v-model="showConfirmDialog" width="auto" height="auto" :show-header="false">
            <div class="p-6 flex items-start gap-4">
                <span class="i-md-warning text-warning w-10 h-10"></span>
                <div class="flex-1">
                    <h3 class="text-lg font-bold mb-4">Note title is empty</h3>
                    <p class="text-md text-t-placeholder mb-6">
                        You have not entered a note title. Please choose one of the following options to continue:
                    </p>
                    <div class="flex flex-col gap-3">
                        <button @click="cancelSaveNote">Cancel save</button>
                        <button @click="confirmSaveWithDefaultName">Save with default name</button>
                        <button v-if="extractFirstHeading(getNoteContent())" @click="confirmSaveWithHeading">
                            Use first heading as note name
                        </button>
                    </div>
                </div>
            </div>
        </Modal>
    </div>
</template>
