<script setup lang="ts">
defineProps<{
    size?: 'sm' | 'md' | 'lg'
    color?: string
}>()
</script>

<template>
    <div
        class="inline-flex items-center justify-center"
        :class="{
            'w-6 h-6': size === 'sm',
            'w-10 h-10': size === 'md' || !size,
            'w-16 h-16': size === 'lg',
        }">
        <div class="relative w-full h-full">
            <div class="absolute inset-0 rounded-full border-2 border-slate-200"></div>
            <div
                class="absolute inset-0 rounded-full border-2 border-transparent border-t-1px border-t-solid border-t-puper animate-spin"></div>
        </div>
    </div>
</template>
