/**
 * 选择项接口定义
 * @property {string} label - 选项显示文本
 * @property {string} value - 选项值
 */
export interface SelectOption {
    label: string
    value: string
}

export { default as MySelect } from './index.vue'
