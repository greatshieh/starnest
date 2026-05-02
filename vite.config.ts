import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import Unocss from 'unocss/vite'
import Components from 'unplugin-vue-components/vite'

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST

// https://vite.dev/config/
export default defineConfig(async () => ({
    plugins: [
        vue(),
        Unocss(),
        Components({
            dts: 'src/types/components.d.ts', // 类型声明文件路径
            dirs: ['src/components', 'src/**/components'],
        }),
    ],
    resolve: {
        alias: {
            '@': '/src',
        },
    },
    clearScreen: false,
    server: {
        port: 1420,
        strictPort: true,
        host: host || false,
        hmr: host
            ? {
                  protocol: 'ws',
                  host,
                  port: 1421,
              }
            : undefined,
        watch: {
            // 3. tell Vite to ignore watching `src-tauri`
            ignored: ['**/src-tauri/**'],
        },
    },
}))
