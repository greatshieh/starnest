<script setup lang="ts">
import type { api as ApiTypes } from '@/types'
import Modal from '@/components/Modal.vue'

defineProps<{
    show: boolean
    collection: (ApiTypes.collection.CollectionWithRepoCount & { repos: ApiTypes.repo.Repository[] }) | null
}>()

const emit = defineEmits<{
    'update:show': [value: boolean]
    confirm: []
}>()
</script>

<template>
    <Modal
        :modelValue="show"
        @update:modelValue="emit('update:show', $event)"
        width="auto"
        height="auto"
        :showHeader="false">
        <div class="relative p-5" style="width: 360px">
            <h3 class="font-semibold mb-6 text-lg">Confirm Delete</h3>
            <p class="mb-5">
                Are you sure you want to delete the collection "<span style="color: var(--sv-text-color-1); font-weight: 500">{{ collection?.name }}</span>"? Repos will not be deleted.
            </p>
            <div class="flex items-center justify-end gap-2">
                <button @click="emit('update:show', false)">Cancel</button>
                <button class="error" @click="emit('confirm')">Delete</button>
            </div>
        </div>
    </Modal>
</template>
