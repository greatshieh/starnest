import { createI18n } from 'vue-i18n'
import { zhCN } from './zh-CN'

export const i18n = createI18n({
    legacy: false,
    locale: 'zh-CN',
    fallbackLocale: 'zh-CN',
    messages: {
        'zh-CN': zhCN,
    },
})

export type LocaleType = keyof typeof zhCN

export function useI18n() {
    return i18n.global
}

export function t(key: string, ...args: any[]): string {
    return i18n.global.t(key, ...args)
}

export function setLocale(locale: string) {
    i18n.global.locale.value = locale
}

export function getLocale(): string {
    return i18n.global.locale.value
}
