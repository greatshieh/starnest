<script setup lang="ts">
import { RouterView } from 'vue-router'
import Message from './components/Message.vue'
import NotificationContainer from './components/NotificationContainer.vue'
import { useMessage } from './composables/useMessage'
import { useNotification } from './composables/useNotification'

const { messages, removeMessage } = useMessage()
const { notifications, removeNotification } = useNotification()
</script>

<template>
    <RouterView />

    <!-- 消息提示容器 - 显示在窗口中间上方 -->
    <div class="fixed top-14 left-1/2 -translate-x-1/2 z-50 flex flex-col gap-2">
        <Message v-for="msg in messages" :key="msg.id" :message="msg" @close="removeMessage" />
    </div>

    <!-- 通知提示容器 - 组件内部处理位置计算 -->
    <NotificationContainer :notifications="notifications" @close="removeNotification" />
</template>
