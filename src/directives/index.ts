import { App } from 'vue'
import clickOutside from './click-outside'

// 扩展HTMLElement类型，添加自定义属性的类型定义
declare global {
    interface HTMLElement {
        _clickOutsideHandler?: (event: MouseEvent | TouchEvent) => void
    }
}

export default function setupDirectives(app: App) {
    app.directive('click-outside', clickOutside)
}
