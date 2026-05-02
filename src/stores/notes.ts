import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { api as ApiTypes } from '@/types'

export const useNotesStore = defineStore('notes', () => {
    const notes = ref<ApiTypes.note.Note[]>([])

    function addNote(note: Omit<ApiTypes.note.Note, 'id'>): void {
        notes.value.push({
            ...note,
            id: Date.now(),
        })
    }

    function updateNote(id: number, updates: Partial<Omit<ApiTypes.note.Note, 'id'>>): void {
        const note = notes.value.find(n => n.id === id)
        if (note) {
            Object.assign(note, updates)
        }
    }

    function deleteNote(id: number): void {
        notes.value = notes.value.filter(n => n.id !== id)
    }

    function findNoteById(id: number): ApiTypes.note.Note | undefined {
        return notes.value.find(n => n.id === id)
    }

    return {
        notes,
        addNote,
        updateNote,
        deleteNote,
        findNoteById,
    }
})