<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { openUrl } from '@tauri-apps/plugin-opener'

const router = useRouter()
const authStore = useAuthStore()
const loading = ref(false)
const error = ref('')

const features = [
    { icon: 'tag', label: 'Smart Tagging' },
    { icon: 'search', label: 'Global Search' },
    { icon: 'auto-awesome', label: 'AI Curation' },
]

function closeWindow() {
    getCurrentWindow().close()
}

const handleLogin = async () => {
    if (!authStore.token.trim()) {
        error.value = 'Please enter your Personal Access Token'
        return
    }

    loading.value = true
    error.value = ''

    try {
        await authStore.login(authStore.token)
        router.push('/dashboard')
    } catch (err) {
        console.error('Login error:', err)
        error.value = 'Login failed. Please check your token and try again.'
    } finally {
        loading.value = false
    }
}
</script>

<template>
    <div class="h-screen overflow-hidden bg-page rounded-md">
        <!-- Window Header -->
        <div class="h-[32px] bg-card" data-tauri-drag-region>
            <div
                class="hover:bg-popover hover:scale-[1.02] transition-all duration-200 p-1 rounded-md cursor-pointer hover:text-error absolute top-1 right-2"
                @click="closeWindow">
                <span class="i-md-close"></span>
            </div>
        </div>

        <main class="flex-grow flex items-center justify-center p-6 pt-20 relative">
            <!-- Login Card -->
            <div class="w-full max-w-md relative group">
                <div class="relative bg-popover rounded-xl border shadow-md overflow-hidden">
                    <div class="p-4 flex flex-col items-center text-center">
                        <!-- Branding -->
                        <div class="mb-4">
                            <img src="/src/assets/app-icon.png" alt="Star Manager" class="size-20" />
                            <h1 class="text-3xl font-extrabold text-primary tracking-tighter mb-2">Star Manager</h1>
                            <p class="text-t-primary text-sm font-medium tracking-wide">Elevate Your GitHub Curation</p>
                        </div>

                        <!-- CTA Action -->
                        <div class="w-full space-y-4">
                            <div class="mb-4 text-left w-full">
                                <label class="block text-sm font-semibold tracking-wider mb-3 ml-1" for="pat-token">
                                    Personal Access Token
                                </label>
                                <MyInput
                                    v-model="authStore.token"
                                    id="pat-token"
                                    placeholder="Enter Personal Access Token..." />
                            </div>

                            <!-- Error Message -->
                            <div v-if="error" class="p-3 bg-[var(--sv-modal-color)] rounded-md text-sm text-error">
                                {{ error }}
                            </div>

                            <button class="w-full primary plain" @click="handleLogin" :disabled="loading">
                                <span class="i-md-login text-2xl group-hover:rotate-12 transition-transform"></span>
                                {{ loading ? 'Logging in...' : 'Login with GitHub' }}
                            </button>

                            <div class="pt-2 flex flex-col gap-3">
                                <div
                                    class="text-t-secondary hover:text-primary text-sm font-medium transition-colors flex items-center justify-center gap-1.5 cursor-pointer"
                                    @click="
                                        openUrl(
                                            'https://github.com/settings/tokens/new?scopes=read:user,user:email,repo:status',
                                        )
                                    ">
                                    Learn more about GitHub tokens
                                    <span
                                        class="i-md-arrow-forward group-hover:translate-x-0.5 transition-transform"></span>
                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- Footer Tonal Layer -->
                    <div class="bg-light-bg-mute/30 py-2 px-8 text-center border-t border-light-border-secondary/10">
                        <p class="text-xs text-light-text-secondary/60 font-medium">
                            Secure connection powered by GitHub OAuth 2.0
                        </p>
                    </div>
                </div>

                <!-- Supporting Text / Visuals -->
                <div class="mt-8 grid grid-cols-3 gap-4 px-2">
                    <div
                        v-for="feature in features"
                        :key="feature.icon"
                        class="flex flex-col items-center gap-1 opacity-40 hover:opacity-100 transition-opacity">
                        <span :class="`i-md-${feature.icon}`" class="text-2xl text-fg"></span>
                        <span class="text-[10px] uppercase tracking-widest text-secondary">{{ feature.label }}</span>
                    </div>
                </div>
            </div>
        </main>

        <!-- Decorative Bottom Gradient -->
        <div
            class="fixed bottom-0 left-0 right-0 h-1 bg-gradient-to-r from-transparent via-primary/20 to-transparent"></div>
    </div>
</template>
