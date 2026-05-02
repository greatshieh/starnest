<script setup lang="ts">
interface Option {
    label: string
    value: number | string
}

defineProps<{
    options: Option[]
    modelValue: number | string
    columns?: number
}>()

const emit = defineEmits<{
    (e: 'update:modelValue', value: number | string): void
}>()

const selectOption = (value: number | string) => {
    emit('update:modelValue', value)
}
</script>

<template>
    <div
        class="gap-2"
        :class="[
            columns === 3
                ? 'grid grid-cols-3'
                : columns === 4
                  ? 'grid grid-cols-4'
                  : columns === 5
                    ? 'grid grid-cols-5'
                    : columns === 6
                      ? 'grid grid-cols-6'
                      : 'grid grid-cols-3',
        ]">
        <button
            v-for="option in options"
            :key="option.value"
            class="font-semibold transition-all"
            :class="{ primary: modelValue === option.value }"
            @click="selectOption(option.value)">
            {{ option.label }}
        </button>
    </div>
</template>
