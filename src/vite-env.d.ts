/// <reference types="vite/client" />

declare module 'vue3-virtual-scroll-list' {
  import type { DefineComponent } from 'vue'
  const VirtualList: DefineComponent<{
    dataKey?: string
    dataSources?: any[]
    dataComponent?: any
    extraProps?: Record<string, any>
    itemSize?: number
    estimateSize?: number
  }>
  export default VirtualList
}
