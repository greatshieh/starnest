<script setup lang="ts">
import { computed } from 'vue'
import Notification from './Notification.vue'
import type { Notification as NotificationType, NotificationPosition } from '@/composables/useNotification'

const props = defineProps<{
    notifications: NotificationType[]
}>()

const emit = defineEmits<{
    close: [id: number]
}>()

// 根据位置分组通知
const notificationsByPosition = computed(() => {
    const groups: Record<NotificationPosition, NotificationType[]> = {
        'top-right': [],
        'top-left': [],
        'bottom-right': [],
        'bottom-left': [],
    }
    
    props.notifications.forEach(notification => {
        const position = notification.position || 'top-right'
        if (position in groups) {
            groups[position].push(notification)
        }
    })
    
    return groups
})

// 获取位置样式
function getPositionStyle(position: NotificationPosition) {
    const titlebarHeight = 32 // titlebar 高度
    const padding = 16 // 边距
    
    const styles: Record<string, string> = {}
    
    switch (position) {
        case 'top-right':
            styles.top = `${titlebarHeight + 4}px`
            styles.right = `${padding}px`
            styles.flexDirection = 'column'
            break
        case 'top-left':
            styles.top = `${titlebarHeight + 4}px`
            styles.left = `${padding}px`
            styles.flexDirection = 'column'
            break
        case 'bottom-right':
            styles.bottom = `${padding + 32}px` // 底部留出状态栏空间
            styles.right = `${padding}px`
            styles.flexDirection = 'column-reverse'
            break
        case 'bottom-left':
            styles.bottom = `${padding + 32}px` // 底部留出状态栏空间
            styles.left = `${padding}px`
            styles.flexDirection = 'column-reverse'
            break
        default:
            styles.top = `${titlebarHeight + 4}px`
            styles.right = `${padding}px`
            styles.flexDirection = 'column'
    }
    
    return styles
}

// 获取动画名称
function getAnimationName(position: NotificationPosition) {
    return position.includes('left') ? 'notification-left' : 'notification-right'
}
</script>

<template>
    <Teleport to="body">
        <div
            v-for="(items, position) in notificationsByPosition"
            :key="position"
            class="fixed z-50 flex gap-3"
            :style="getPositionStyle(position as NotificationPosition)"
        >
            <TransitionGroup :name="getAnimationName(position as NotificationPosition)">
                <Notification
                    v-for="notification in items"
                    :key="notification.id"
                    :notification="notification"
                    @close="emit('close', $event)"
                />
            </TransitionGroup>
        </div>
    </Teleport>
</template>

<style scoped>
/* 右侧通知动画 */
.notification-right-enter-active,
.notification-right-leave-active {
    transition: all 0.3s ease;
}

.notification-right-enter-from {
    opacity: 0;
    transform: translateX(100%);
}

.notification-right-leave-to {
    opacity: 0;
    transform: translateX(100%);
}

.notification-right-move {
    transition: transform 0.3s ease;
}

/* 左侧通知动画 */
.notification-left-enter-active,
.notification-left-leave-active {
    transition: all 0.3s ease;
}

.notification-left-enter-from {
    opacity: 0;
    transform: translateX(-100%);
}

.notification-left-leave-to {
    opacity: 0;
    transform: translateX(-100%);
}

.notification-left-move {
    transition: transform 0.3s ease;
}
</style>
