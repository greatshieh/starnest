/**
 * 应用入口文件
 * 初始化 Vue 应用、路由、状态管理等
 * @module main
 */

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import router from './router'
import App from './App.vue'
import { i18n } from './i18n'

// 样式文件
import './style/index.css'
import 'virtual:uno.css'

// 状态管理
import { useAuthStore } from './stores/auth'
import { useSettingsStore } from './stores/settings'
import { useSyncStore } from './stores/sync'
import { useShortcutStore } from './stores/shortcuts'

// 自定义指令
import setupDirectives from './directives'

/**
 * 设置并启动应用
 */
async function setupApp(): Promise<void> {
    // 创建应用实例
    const app = createApp(App)

    // 创建 Pinia 实例（只创建一次！确保全局共享）
    const pinia = createPinia()
    app.use(pinia)

    // 获取 stores 实例（使用同一个 Pinia）
    const authStore = useAuthStore()
    const settingsStore = useSettingsStore()
    const syncStore = useSyncStore()
    const shortcutStore = useShortcutStore()

    // 加载 localStorage 缓存（同步操作）
    syncStore.loadLastSync()
    shortcutStore.initShortcuts()

    // 并行加载配置（在路由挂载前完成，确保登录状态就绪）
    await Promise.all([authStore.loadConfig(), settingsStore.loadConfig()])

    // 设置自定义指令
    setupDirectives(app)

    // 使用路由（此时配置已加载完成，路由守卫能正确获取登录状态）
    app.use(router)

    // 使用 i18n 国际化
    app.use(i18n)

    // 挂载应用
    app.mount('#app')
}

// 启动应用
setupApp()
