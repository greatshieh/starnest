<script setup lang="ts">
defineProps<{
    showDeleteModal: boolean
    deletingTag: { id: number; name: string; repoCount: number } | null
}>()

const emit = defineEmits<{
    'update:showDeleteModal': [value: boolean]
    confirm: []
}>()

function handleClose() {
    emit('update:showDeleteModal', false)
}

function handleConfirm() {
    emit('confirm')
}
</script>

<template>
    <Modal
        :model-value="showDeleteModal"
        @update:model-value="v => emit('update:showDeleteModal', v)"
        width="auto"
        height="auto"
        :showHeader="false">
        <div class="relative p-5 w-[360px]">
            <h3 class="text-xl font-semibold mb-4" style="color: var(--sv-text-color-1)">Confirm Delete</h3>
            <p class="mb-5 text-lg">
                Are you sure you want to delete the tag 「<span
                    style="color: var(--sv-text-color-1); font-weight: 500"
                    >{{ deletingTag?.name }}</span
                >」? This will remove the association with {{ deletingTag?.repoCount }} repos, but will not delete the
                repos.
            </p>
            <div class="flex items-center justify-end gap-2">
                <button @click="handleClose">Cancel</button>
                <button class="error" @click="handleConfirm">Delete</button>
            </div>
        </div>
    </Modal>
</template>
