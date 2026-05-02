/**
 * 路由守卫模块
 * 包含认证守卫和窗口尺寸守卫
 * @module router/guards
 */

import type { NavigationGuardNext, RouteLocation } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { PhysicalSize } from '@tauri-apps/api/dpi'

/**
 * 认证守卫
 * 验证用户登录状态，控制页面访问权限
 * @param to - 目标路由
 * @param next - 导航函数
 */
export async function authGuard(to: RouteLocation, next: NavigationGuardNext): Promise<void> {
    // 获取认证状态
    const authStore = useAuthStore()
    const requiresAuth = to.meta.requiresAuth
    const isLoginRoute = to.name === 'Login'
    // 检查是否需要认证
    if (requiresAuth && !authStore.isLoggedIn) {
        // 需要认证但未登录，重定向到登录页
        next('/login')
        return
    }

    // 已登录状态下访问登录页，重定向到首页
    if (!requiresAuth && authStore.isLoggedIn && isLoginRoute) {
        next('/dashboard')
        return
    }

    // 正常放行
    next()
}

/**
 * 窗口尺寸守卫
 * 根据路由设置合适的窗口尺寸
 * @param to - 目标路由
 * @param _next - 导航函数（此处未使用）
 */
export async function windowSizeGuard(to: RouteLocation, _next: NavigationGuardNext): Promise<void> {
    try {
        const window = getCurrentWindow()
        const isLoginRoute = to.name === 'Login'

        if (isLoginRoute) {
            // 登录页面使用小窗口模式
            await window.setSize(new PhysicalSize(800, 600))
            await window.setMinSize(new PhysicalSize(800, 600))
            await window.setMaxSize(new PhysicalSize(800, 600))
        } else if (to.name !== 'Settings') {
            const isMaximized = await window.isMaximized()
            if (!isMaximized) {
                await window.setMinSize(new PhysicalSize(1280, 800))
                await window.setMaxSize(null)
                await window.maximize()
            }
        }

        // 窗口居中
        await window.center()
    } catch (error) {
        console.error('Failed to set window size:', error)
    }
}
