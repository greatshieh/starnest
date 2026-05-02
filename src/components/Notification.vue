<script setup lang="ts">
import { computed, onMounted } from 'vue'
import type { Notification } from '@/composables/useNotification'

const props = defineProps<{
    notification: Notification
}>()

const emit = defineEmits<{
    close: [id: number]
}>()

const typeStyles = computed(() => ({
    success: {
        borderColor: 'var(--sv-success-solid)',
        iconColor: 'var(--sv-success-solid)',
        bgColor: 'rgba(var(--sv-success), 0.08)',
        icon: 'i-md-check-circle',
    },
    error: {
        borderColor: 'var(--sv-error-solid)',
        iconColor: 'var(--sv-error-solid)',
        bgColor: 'rgba(var(--sv-error), 0.08)',
        icon: 'i-md-error',
    },
    warning: {
        borderColor: 'var(--sv-warning-solid)',
        iconColor: 'var(--sv-warning-solid)',
        bgColor: 'rgba(var(--sv-warning), 0.08)',
        icon: 'i-md-warning',
    },
    info: {
        borderColor: 'var(--sv-info-solid)',
        iconColor: 'var(--sv-info-solid)',
        bgColor: 'rgba(var(--sv-info), 0.08)',
        icon: 'i-md-info',
    },
}))

onMounted(() => {
    // 默认 500ms 自动关闭，duration=0 表示不自动关闭
    const duration = props.notification.duration ?? 500
    if (duration > 0) {
        setTimeout(() => {
            close()
        }, duration)
    }
})

function close() {
    emit('close', props.notification.id)
}
</script>

<template>
    <div
        class="relative rounded-lg border-light p-4 shadow-lg min-w-[320px] max-w-[400px]"
        :style="{
            borderLeftColor: typeStyles[notification.type].borderColor,
            background: typeStyles[notification.type].bgColor,
        }">
        <!-- 关闭按钮 -->
        <button
            class="absolute top-2 right-2 p-1 rounded hover:bg-black/5 transition-colors"
            @click="close"
            style="color: var(--sv-text-muted)">
            <span class="i-md-close text-2xl"></span>
        </button>

        <!-- 内容区域 -->
        <div class="flex items-center justify-start gap-3">
            <!-- 图标 -->
            <div class="shrink-0 mt-0.5" :style="{ color: typeStyles[notification.type].iconColor }">
                <span :class="`${typeStyles[notification.type].icon} text-2xl`"></span>
            </div>

            <!-- 文字内容 -->
            <div class="flex-1 min-w-0 text-t-primary">
                <div v-if="notification.title" class="font-semibold mb-2">
                    {{ notification.title }}
                </div>
                <p>
                    {{ notification.message }}
                </p>
            </div>
        </div>
    </div>
</template>
