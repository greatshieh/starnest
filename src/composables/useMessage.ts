/**
 * 消息提示组合式函数
 * 提供全局消息提示功能
 * @module composables/useMessage
 */

import { ref } from 'vue'

/**
 * 消息类型
 */
export type MessageType = 'success' | 'error' | 'warning' | 'info'

export interface MessageMethods {
  success: (message: string) => void
  error: (message: string) => void
  warning: (message: string) => void
  info: (message: string) => void
}

/**
 * 消息接口
 */
export interface Message {
  id: number
  type: MessageType
  message: string
}

// 全局消息列表
const messages = ref<Message[]>([])

/**
 * 使用消息提示
 * @returns 消息相关的状态和方法
 */
export function useMessage() {
  function addMessage(message: string, type: MessageType = 'info'): void {
    const id = Date.now()
    messages.value.push({ id, type, message })

    // 3秒后自动移除消息
    setTimeout(() => {
      removeMessage(id)
    }, 3000)
  }

  /**
   * 移除消息
   * @param id - 消息 id
   */
  function removeMessage(id: number): void {
    const index = messages.value.findIndex(m => m.id === id)
    if (index !== -1) {
      messages.value.splice(index, 1)
    }
  }

  /**
   * 成功消息
   * @param message - 消息内容
   */
  function success(message: string): void {
    addMessage(message, 'success')
  }

  /**
   * 错误消息
   * @param message - 消息内容
   */
  function error(message: string): void {
    addMessage(message, 'error')
  }

  /**
   * 警告消息
   * @param message - 消息内容
   */
  function warning(message: string): void {
    addMessage(message, 'warning')
  }

  /**
   * 信息消息
   * @param message - 消息内容
   */
  function info(message: string): void {
    addMessage(message, 'info')
  }

  return {
    messages,
    addMessage,
    removeMessage,
    success,
    error,
    warning,
    info,
  }
}