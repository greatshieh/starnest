<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted, shallowRef, nextTick } from 'vue'
import { useSettingsStore } from '@/stores/settings'
import Vditor from 'vditor'
import 'vditor/dist/index.css'
import { useTheme } from '@/composables/useTheme'

const props = defineProps<{
    noteContent: string
}>()

const emit = defineEmits<{
    (e: 'update:noteContent', value: string): void
}>()

const { themeMode } = useTheme()
const settingsStore = useSettingsStore()
const vditor = shallowRef<Vditor>()
const isMounted = ref(false)

const initEditor = () => {
    if (vditor.value) return

    nextTick(() => {
        vditor.value = new Vditor('note-editor-container', {
            height: 400,
            cache: { enable: false },
            theme: themeMode.value === 'dark' ? 'dark' : 'classic',
            preview: { theme: { current: themeMode.value === 'dark' ? 'dark' : 'ant-design' } },
            toolbarConfig: {
                pin: true,
            },
            mode: settingsStore.editorMode,
            placeholder: '开始编辑你的学习笔记',
            after: () => {
                if (vditor.value && props.noteContent) {
                    console.log(props.noteContent)
                    vditor.value.setValue(props.noteContent)
                }
            },
            input: () => {
                if (vditor.value) {
                    emit('update:noteContent', vditor.value.getValue())
                }
            },
        })
    })
}

watch(
    () => props.noteContent,
    newValue => {
        if (vditor.value && isMounted.value) {
            const currentValue = vditor.value.getValue()

            if (currentValue !== newValue) {
                vditor.value.setValue(newValue)
            }
        }
    },
    { immediate: true, deep: true },
)

watch(
    () => settingsStore.editorMode,
    _newMode => {
        if (vditor.value) {
            vditor.value.destroy()
            vditor.value = undefined
            initEditor()
        }
    },
)

watch(
    () => themeMode.value,
    newMode => {
        if (vditor.value) {
            vditor.value.setTheme(newMode === 'dark' ? 'dark' : 'classic', 'ant-design')
        }
    },
)

onMounted(() => {
    isMounted.value = true
    initEditor()
})

onUnmounted(() => {
    if (vditor.value) {
        vditor.value.destroy()
        vditor.value = undefined
    }
})

const getValue = (): string => {
    if (!vditor.value) return ''
    return vditor.value.getValue()
}

defineExpose({ getValue })
</script>

<template>
    <div id="note-editor-container"></div>
</template>
