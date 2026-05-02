<script setup lang="ts">
import { ref, watch, provide, computed } from 'vue'

interface TabInfo {
    name: string
    label: string
}

interface Props {
    modelValue?: string
}

const props = withDefaults(defineProps<Props>(), {
    modelValue: '',
})

const emit = defineEmits<{
    'update:modelValue': [value: string]
    'tab-click': [name: string]
}>()

const tabs = ref<TabInfo[]>([])
const internalActiveName = ref('')

const activeName = computed({
    get: () => internalActiveName.value || props.modelValue || (tabs.value.length > 0 ? tabs.value[0].name : ''),
    set: (value: string) => {
        internalActiveName.value = value
        emit('update:modelValue', value)
        emit('tab-click', value)
    },
})

const registerTab = (name: string, label: string) => {
    const exists = tabs.value.find(tab => tab.name === name)
    if (!exists) {
        tabs.value.push({ name, label })
        if (!internalActiveName.value && !props.modelValue) {
            internalActiveName.value = tabs.value[0].name
        }
    }
}

const unregisterTab = (name: string) => {
    const index = tabs.value.findIndex(tab => tab.name === name)
    if (index > -1) {
        tabs.value.splice(index, 1)
    }
}

provide('tabsContext', {
    registerTab,
    unregisterTab,
    activeName: computed(() => activeName.value),
})

watch(
    () => props.modelValue,
    newVal => {
        if (newVal) {
            const exists = tabs.value.find(tab => tab.name === newVal)
            if (exists) {
                internalActiveName.value = newVal
            }
        }
    },
)
</script>

<template>
    <div class="my-tabs">
        <div class="my-tabs-header">
            <div
                v-for="tab in tabs"
                :key="tab.name"
                :class="['my-tab', { 'is-active': activeName === tab.name }]"
                @click="activeName = tab.name">
                <span class="my-tab-label">{{ tab.label }}</span>
            </div>
        </div>
        <div class="my-tabs-content">
            <slot></slot>
        </div>
    </div>
</template>

<style scoped>
.my-tabs {
    width: 100%;
}

.my-tabs-header {
    display: flex;
    border-bottom: 1px solid var(--sv-border-color);
    background-color: var(--sv-card-color);
}

.my-tab {
    position: relative;
    padding: 12px 20px;
    font-size: 14px;
    font-weight: 500;
    color: var(--sv-text-color-2);
    cursor: pointer;
    transition: color 0.2s ease;
    border-bottom: 2px solid transparent;
}

.my-tab:hover {
    color: var(--sv-primary-hover-solid);
}

.my-tab.is-active {
    color: var(--sv-primary-solid);
    border-bottom-color: var(--sv-primary-solid);
}

.my-tab-label {
    display: inline-block;
}

.my-tabs-content {
    padding: 20px 0;
}
</style>
