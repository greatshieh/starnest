import type { Directive } from 'vue'

declare module 'vue' {
    export interface ComponentCustomProperties {
        vClickOutside: Directive<Element, () => void>
        vDropdown: Directive<Element, () => void>
    }
}

export {}
