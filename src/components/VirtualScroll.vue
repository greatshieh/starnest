<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'

interface Props {
  htmlContent: string
  containerHeight?: number
}

const props = withDefaults(defineProps<Props>(), {
  containerHeight: 500
})

const containerRef = ref<HTMLElement | null>(null)
const contentRef = ref<HTMLElement | null>(null)
const scrollTop = ref(0)

const CHUNK_SIZE = 300

const contentChunks = computed(() => {
  if (!props.htmlContent) return []
  
  const html = props.htmlContent
  const chunks: string[] = []
  let currentIndex = 0
  
  while (currentIndex < html.length) {
    let chunk = html.substring(currentIndex, currentIndex + CHUNK_SIZE)
    
    const lastTagStart = chunk.lastIndexOf('<')
    const lastTagEnd = chunk.lastIndexOf('>')
    
    if (lastTagStart > lastTagEnd && currentIndex + CHUNK_SIZE < html.length) {
      const nextTagEnd = html.indexOf('>', currentIndex + CHUNK_SIZE)
      if (nextTagEnd !== -1) {
        chunk = html.substring(currentIndex, nextTagEnd + 1)
      }
    }
    
    chunks.push(chunk)
    currentIndex += chunk.length
  }
  
  return chunks
})

const totalHeight = computed(() => {
  return contentChunks.value.length * CHUNK_SIZE * 1.5
})

const visibleStartIndex = computed(() => {
  return Math.max(0, Math.floor(scrollTop.value / (CHUNK_SIZE * 1.2)) - 1)
})

const visibleEndIndex = computed(() => {
  const visibleCount = Math.ceil(props.containerHeight / (CHUNK_SIZE * 1.2)) + 2
  return Math.min(contentChunks.value.length, visibleStartIndex.value + visibleCount)
})

const visibleChunks = computed(() => {
  return contentChunks.value.slice(visibleStartIndex.value, visibleEndIndex.value)
})

const topPadding = computed(() => {
  return visibleStartIndex.value * CHUNK_SIZE * 1.2
})

const bottomPadding = computed(() => {
  return Math.max(0, totalHeight.value - topPadding.value - (visibleChunks.value.length * CHUNK_SIZE * 1.2))
})

const handleScroll = () => {
  if (containerRef.value) {
    scrollTop.value = containerRef.value.scrollTop
  }
}

onMounted(() => {
  if (containerRef.value) {
    containerRef.value.addEventListener('scroll', handleScroll)
  }
})

onUnmounted(() => {
  if (containerRef.value) {
    containerRef.value.removeEventListener('scroll', handleScroll)
  }
})

watch(() => props.htmlContent, () => {
  scrollTop.value = 0
})
</script>

<template>
  <div 
    ref="containerRef"
    class="virtual-scroll-container"
    :style="{ height: containerHeight + 'px' }"
  >
    <div 
      ref="contentRef"
      class="virtual-scroll-content"
      :style="{ height: totalHeight + 'px' }"
    >
      <div 
        v-if="topPadding > 0" 
        class="virtual-scroll-padding"
        :style="{ height: topPadding + 'px' }"
      ></div>
      <div 
        v-for="(chunk, index) in visibleChunks" 
        :key="visibleStartIndex + index"
        class="virtual-scroll-chunk"
        v-html="chunk"
      ></div>
      <div 
        v-if="bottomPadding > 0" 
        class="virtual-scroll-padding"
        :style="{ height: bottomPadding + 'px' }"
      ></div>
    </div>
  </div>
</template>

<style scoped>
.virtual-scroll-container {
  overflow-y: auto;
  overflow-x: hidden;
  position: relative;
  width: 100%;
}

.virtual-scroll-content {
  position: relative;
  width: 100%;
}

.virtual-scroll-chunk {
  display: block;
}

.virtual-scroll-padding {
  display: block;
}
</style>
