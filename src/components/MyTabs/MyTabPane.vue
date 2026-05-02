<script setup lang="ts">
import { inject, onMounted, onUnmounted, computed } from 'vue'

interface Props {
    label: string
    name: string
}

const props = defineProps<Props>()

const tabsContext = inject<{
    registerTab: (name: string, label: string) => void
    unregisterTab: (name: string) => void
    activeName: ReturnType<typeof computed>
}>('tabsContext')

const isActive = computed(() => {
    return tabsContext?.activeName.value === props.name
})

onMounted(() => {
    if (tabsContext) {
        tabsContext.registerTab(props.name, props.label)
    }
})

onUnmounted(() => {
    if (tabsContext) {
        tabsContext.unregisterTab(props.name)
    }
})
</script>

<template>
    <div class="my-tab-pane" v-if="isActive">
        <slot></slot>
    </div>
</template>

<style scoped>
.my-tab-pane {
    display: block;
}
</style>
