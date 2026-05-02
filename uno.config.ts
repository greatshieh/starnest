import { defineConfig, presetMini, presetIcons, presetAttributify } from 'unocss'
import { FileSystemIconLoader } from '@iconify/utils/lib/loader/node-loaders'

export default defineConfig({
    presets: [
        presetMini({
            dark: {
                dark: "html[data-theme='dark']",
            },
        }),
        presetAttributify(),
        presetIcons({
            collections: {
                md: FileSystemIconLoader('./src/assets/icons', (svg: string) => {
                    return svg.replace(/^<svg\s/, '<svg width="1em" height="1em" ')
                }),
            },
            extraProperties: {
                display: 'inline-block',
                'vertical-align': 'middle',
            },
            warn: true,
        }),
    ],
    safelist: [
        'i-md-dashboard',
        'i-md-setting',
        'i-md-note',
        'i-md-star-outline',
        'i-md-star',
        'i-md-collection',
        'i-md-auto-awesome',
        'i-md-search',
        'i-md-tag',
        'i-md-check-circle',
        'i-md-error',
        'i-md-warning',
        'i-md-info',
        'i-md-grid',
        'i-md-list',
    ],
    theme: {
        breakpoints: {
            xs: '480px',
            sm: '640px',
            md: '768px',
            lg: '1024px',
            xl: '1280px',
            '2xl': '1536px',
        },
        colors: {
            primary: 'var(--sv-primary-solid)',
            success: 'var(--sv-success-solid)',
            warning: 'var(--sv-warning-solid)',
            error: 'var(--sv-error-solid)',
            info: 'var(--sv-info-solid)',
            page: 'var(--sv-base-color)',
            body: 'var(--sv-body-color)',
            card: 'var(--sv-card-color)',
            modal: 'var(--sv-modal-color)',
            hover: 'var(--sv-hover-color)',
            popover: 'var(--sv-popover-color)',
            inverted: 'var(--sv-inverted-color)',
            't-primary': 'var(--sv-text-color-1)',
            't-regular': 'var(--sv-text-color-2)',
            't-secondary': 'var(--sv-text-color-3)',
            't-placeholder': 'var(--sv-placeholder-color)',
            't-placeholder-disabled': 'var(--sv-placeholder-color-disabled)',
            't-disabled': 'var(--sv-text-color-disabled)',
            'b-color': 'var(--sv-border-color)',
        },
        shadow: {
            xs: 'var(--sv-shadow-xs)',
            sm: 'var(--sv-shadow-sm)',
            md: 'var(--sv-shadow-md)',
            lg: 'var(--sv-shadow-lg)',
            none: 'var(--sv-shadow-none)',
        },
        fontSize: {
            xs: 'var(--sv-font-size-xs)',
            sm: 'var(--sv-font-size-sm)',
            base: 'var(--sv-font-size-base)',
            md: 'var(--sv-font-size-md)',
            lg: 'var(--sv-font-size-lg)',
            xl: 'var(--sv-font-size-xl)',
            '2xl': 'var(--sv-font-size-2xl)',
            '3xl': 'var(--sv-font-size-3xl)',
            lineHeight: 'var(--sv-line-height)',
            fontWeight: 'var(--sv-font-weight-normal)',
            fontWeightMedium: 'var(--sv-font-weight-medium)',
            fontWeightSemibold: 'var(--sv-font-weight-semibold)',
            fontWeightBold: 'var(--sv-font-weight-bold)',
        },
    },
    rules: [
        [
            /^line-clamp-(\d+)$/,
            ([, rows]) => ({
                overflow: 'hidden',
                display: '-webkit-box',
                '-webkit-box-orient': 'vertical',
                '-webkit-line-clamp': rows,
                'line-clamp': rows,
                'word-break': 'break-all',
            }),
        ],
    ],
    shortcuts: {
        'border-light': 'border border-solid border-[var(--sv-border-color)]',
    },
})
