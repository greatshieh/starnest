<script setup lang="ts">
import { computed, CSSProperties } from 'vue'

defineOptions({
    name: 'SettingPanel',
})

const props = defineProps({
    inline: {
        type: Boolean,
        default: false,
    },
    title: {
        type: String,
        default: '',
    },
    description: {
        type: String,
        default: '',
    },
    columns: {
        type: String,
        default: '1fr 3fr',
    },
})

const panelCss = computed<CSSProperties>(() => {
    if (props.inline) {
        return {
            display: 'grid',
            alignItems: 'center',
            gridTemplateColumns: props.columns,
        }
    }
    return {}
})
</script>

<template>
    <div class="w-full mt-8 pr-8" :style="panelCss">
        <div class="flex flex-col mr-2">
            <span class="font-semibold text-md">{{ props.title }}</span>
            <span v-if="props.description" class="text-[var(--sv-text-color-3)] mt-2">{{ props.description }}</span>
        </div>
        <div :class="props.inline ? 'flex-1 w-full flex items-center' : 'mt-4'">
            <slot></slot>
        </div>
    </div>
</template>
