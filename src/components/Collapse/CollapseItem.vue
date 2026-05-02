<script setup lang="ts">
import { ref, inject, watch, onMounted } from 'vue'

const props = defineProps({
    name: {
        type: String,
        required: true,
    },
    title: {
        type: String,
        default: '',
    },
})

const collapse = inject('collapse') as {
    activeName: { value: string }
    onItemClick: (name: string) => void
}

const isExpanded = ref(false)

const updateExpanded = () => {
    isExpanded.value = collapse.activeName.value === props.name
}

const handleClick = () => {
    collapse.onItemClick(props.name)
}

watch(
    () => collapse.activeName.value,
    () => {
        updateExpanded()
    }
)

onMounted(() => {
    updateExpanded()
})
</script>

<template>
    <div class="collapse-item border-t border-zinc-200/40">
        <button
            class="w-full flex items-center justify-between px-3 py-3 text-[10px] uppercase tracking-widest text-slate-500 font-bold hover:text-slate-700 transition-colors"
            @click="handleClick">
            <span>{{ title }}</span>
            <span
                class="i-md-expand-more text-sm transition-transform duration-200"
                :class="{ 'rotate-180': isExpanded }"></span>
        </button>
        <div
            v-show="isExpanded"
            class="space-y-1 overflow-y-auto max-h-64">
            <slot></slot>
        </div>
    </div>
</template>
