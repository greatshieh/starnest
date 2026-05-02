<script setup lang="ts">
import { presetColors } from '@/constants/colors'

defineProps<{
    showFormModal: boolean
    formName: string
    formColor: string
    editingTag: { id: number; name: string; color: string } | null
}>()

const emit = defineEmits<{
    'update:showFormModal': [value: boolean]
    'update:formName': [value: string]
    'update:formColor': [value: string]
    save: []
}>()

function handleClose() {
    emit('update:showFormModal', false)
}

function handleSave() {
    emit('save')
}
</script>

<template>
    <Modal
        :model-value="showFormModal"
        @update:model-value="v => emit('update:showFormModal', v)"
        width="auto"
        height="auto"
        :showHeader="false">
        <div class="relative p-5 w-[400px]">
            <h3 class="text-xl font-semibold mb-4 text-t-primary">
                {{ editingTag ? 'Edit Tag' : 'Create Tag' }}
            </h3>

            <div class="mb-4">
                <label class="mb-4 block text-lg">Tag Name</label>
                <MyInput
                    :model-value="formName"
                    @update:model-value="v => emit('update:formName', v)"
                    placeholder="Enter tag name" />
            </div>

            <div class="mb-4">
                <label class="mb-4 block text-lg">Tag Color</label>
                <div class="flex flex-wrap gap-2">
                    <div
                        v-for="color in presetColors"
                        :key="color"
                        class="size-7 rounded-full cursor-pointer transition-transform hover:scale-110 flex items-center justify-center"
                        :style="{
                            background: color,
                            boxShadow:
                                formColor === color ? '0 0 0 2px var(--sv-card-color), 0 0 0 4px ' + color : 'none',
                        }"
                        @click="emit('update:formColor', color)">
                        <span v-if="formColor === color" class="i-md-check"></span>
                    </div>
                </div>
            </div>

            <div class="flex items-center justify-end gap-4">
                <button class="plain" @click="handleClose">Cancel</button>
                <button class="primary dashed" @click="handleSave">Save</button>
            </div>
        </div>
    </Modal>
</template>
