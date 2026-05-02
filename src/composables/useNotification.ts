/**
 * 通知提示组合式函数
 * 提供全局通知提示功能，类似 Element Plus 的 ElNotification
 * @module composables/useNotification
 */

import { ref } from 'vue'

/**
 * 通知类型
 */
export type NotificationType = 'success' | 'error' | 'warning' | 'info'

/**
 * 通知位置
 */
export type NotificationPosition = 'top-right' | 'top-left' | 'bottom-right' | 'bottom-left'

/**
 * 通知接口
 */
export interface Notification {
    id: number
    type: NotificationType
    title?: string
    message: string
    duration?: number
    position: NotificationPosition
}

/**
 * 通知选项接口
 */
export interface NotificationOptions {
    title?: string
    message: string
    /**
     * 自动关闭时间（毫秒）
     * - 不传或 undefined：使用默认值（500ms）
     * - 0：不自动关闭，需手动点击关闭按钮
     * - > 0：指定毫秒数后自动关闭
     */
    duration?: number
    position?: NotificationPosition
}

// 默认自动关闭时间（毫秒）
const DEFAULT_DURATION = 500

// 全局通知列表
const notifications = ref<Notification[]>([])

/**
 * 使用通知提示
 * @returns 通知相关的状态和方法
 */
export function useNotification() {
    /**
     * 添加通知
     * @param message - 通知内容
     * @param type - 通知类型（默认 info）
     * @param options - 额外选项
     */
    function addNotification(
        message: string,
        type: NotificationType = 'info',
        options?: Omit<NotificationOptions, 'message'>,
    ): number {
        const id = Date.now() + Math.random()
        const notification: Notification = {
            id,
            type,
            message,
            title: options?.title,
            duration: options?.duration ?? DEFAULT_DURATION,
            position: options?.position || 'top-right',
        }

        notifications.value.push(notification)

        return id
    }

    /**
     * 移除通知
     * @param id - 通知 id
     */
    function removeNotification(id: number): void {
        const index = notifications.value.findIndex(n => n.id === id)
        if (index !== -1) {
            notifications.value.splice(index, 1)
        }
    }

    /**
     * 成功通知
     * @param message - 通知内容
     * @param options - 额外选项
     */
    function success(message: string, options?: Omit<NotificationOptions, 'message'>): number {
        return addNotification(message, 'success', options)
    }

    /**
     * 错误通知
     * @param message - 通知内容
     * @param options - 额外选项
     */
    function error(message: string, options?: Omit<NotificationOptions, 'message'>): number {
        return addNotification(message, 'error', options)
    }

    /**
     * 警告通知
     * @param message - 通知内容
     * @param options - 额外选项
     */
    function warning(message: string, options?: Omit<NotificationOptions, 'message'>): number {
        return addNotification(message, 'warning', options)
    }

    /**
     * 信息通知
     * @param message - 通知内容
     * @param options - 额外选项
     */
    function info(message: string, options?: Omit<NotificationOptions, 'message'>): number {
        return addNotification(message, 'info', options)
    }

    /**
     * 打开通知（通用方法，可指定类型）
     * @param type - 通知类型
     * @param options - 通知选项
     */
    function open(type: NotificationType, options: NotificationOptions): number {
        return addNotification(options.message, type, {
            title: options.title,
            duration: options.duration,
            position: options.position,
        })
    }

    /**
     * 关闭指定通知
     * @param id - 通知 id
     */
    function close(id: number): void {
        removeNotification(id)
    }

    /**
     * 关闭所有通知
     */
    function closeAll(): void {
        notifications.value = []
    }

    return {
        notifications,
        addNotification,
        removeNotification,
        success,
        error,
        warning,
        info,
        open,
        close,
        closeAll,
    }
}
