<script setup lang="ts">
import { ref, provide } from 'vue'

const props = defineProps({
    modelValue: {
        type: String,
        default: '',
    },
    accordion: {
        type: Boolean,
        default: true,
    },
})

const emit = defineEmits<{
    (e: 'update:modelValue', value: string): void
    (e: 'change', value: string): void
}>()

const activeName = ref(props.modelValue)

const handleItemClick = (name: string) => {
    if (props.accordion) {
        if (activeName.value === name) {
            activeName.value = ''
        } else {
            activeName.value = name
        }
    } else {
        activeName.value = name
    }
    emit('update:modelValue', activeName.value)
    emit('change', activeName.value)
}

provide('collapse', {
    activeName,
    onItemClick: handleItemClick,
})
</script>

<template>
    <div class="collapse space-y-0">
        <slot></slot>
    </div>
</template>
