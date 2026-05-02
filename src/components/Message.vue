<script setup lang="ts">
import { ref, onMounted } from 'vue'

export interface MessageOptions {
  id: number
  type: 'success' | 'error' | 'warning' | 'info'
  message: string
  duration?: number
}

const props = defineProps<{
  message: MessageOptions
}>()

const emit = defineEmits<{
  close: [id: number]
}>()

const isVisible = ref(false)
const isLeaving = ref(false)

const typeStyles = {
  success: {
    bg: 'bg-green-500',
    icon: 'i-md-check-circle',
  },
  error: {
    bg: 'bg-red-500',
    icon: 'i-md-error',
  },
  warning: {
    bg: 'bg-yellow-500',
    icon: 'i-md-warning',
  },
  info: {
    bg: 'bg-blue-500',
    icon: 'i-md-info',
  },
}

onMounted(() => {
  setTimeout(() => {
    isVisible.value = true
  }, 50)
  
  const duration = 3000
  setTimeout(() => {
    close()
  }, duration)
})

function close() {
  isLeaving.value = true
  setTimeout(() => {
    emit('close', props.message.id)
  }, 300)
}
</script>

<template>
  <div
    class="flex items-center justify-center gap-2 px-4 py-2 rounded-lg text-white text-sm font-medium shadow-lg"
    :class="[
      typeStyles[message.type].bg,
      isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 -translate-y-4',
      isLeaving ? 'opacity-0 -translate-y-4' : '',
    ]"
    style="transition: all 0.3s ease-out; max-height: 48px;"
  >
    <span class="text-base">
      <span :class="typeStyles[message.type].icon"></span>
    </span>
    <span>{{ message.message }}</span>
  </div>
</template>
