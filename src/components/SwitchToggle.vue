<script setup lang="ts">
const props = defineProps<{
    modelValue: boolean
    disabled?: boolean
}>()

const emit = defineEmits<{
    (e: 'update:modelValue', value: boolean): void
}>()

const toggle = () => {
    emit('update:modelValue', !props.modelValue)
}
</script>

<template>
    <button
        class="h-7 w-14 items-center rounded-full transition-colors border-light disabled:cursor-not-allowed"
        :class="[modelValue ? 'bg-popover' : 'bg-card']"
        :disabled="disabled"
        @click="toggle">
        <span
            class="inline-block size-5 transform rounded-full transition"
            :class="[
                modelValue ? 'bg-primary -translate-x-4' : 'bg-[var(--sv-icon-color-disabled)] translate-x-4',
            ]"></span>
    </button>
</template>
