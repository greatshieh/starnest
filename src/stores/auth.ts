import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { api } from '@/api'
import { useMessage } from '@/composables/useMessage'

const STORAGE_KEY_AUTH = 'starnest_auth'

export const useAuthStore = defineStore('auth', () => {
    const message = useMessage()

    const user = ref<{ name: string; avatar: string }>({
        name: '',
        avatar: '',
    })

    const token = ref('')

    const isLoggedIn = computed(() => token.value !== '')

    async function login(tokenValue: string): Promise<void> {
        const result = await api.auth.login({ token: tokenValue })
        user.value = {
            name: result.name,
            avatar: result.avatar,
        }
        token.value = tokenValue
        saveToLocalStorage()
    }

    async function logout(): Promise<void> {
        try {
            await api.auth.logout()
            token.value = ''
            user.value = {
                name: '',
                avatar: '',
            }
            localStorage.removeItem(STORAGE_KEY_AUTH)
        } catch (error) {
            message.error('Logout failed')
            throw error
        }
    }

    function saveToLocalStorage(): void {
        try {
            localStorage.setItem(
                STORAGE_KEY_AUTH,
                JSON.stringify({
                    user: user.value,
                    token: token.value,
                }),
            )
        } catch (error) {
            console.error('Failed to save auth to localStorage:', error)
        }
    }

    function loadFromLocalStorage(): boolean {
        try {
            const stored = localStorage.getItem(STORAGE_KEY_AUTH)
            if (stored) {
                const data = JSON.parse(stored)
                if (data.user) {
                    user.value = data.user
                }
                if (data.token) {
                    token.value = data.token
                }
                return true
            }
        } catch (error) {
            console.error('Failed to load auth from localStorage:', error)
        }
        return false
    }

    async function loadConfig(): Promise<void> {
        const hasLocalData = loadFromLocalStorage()

        if (!hasLocalData || !token.value) {
            try {
                const config = await api.auth.getAuthConfig()
                if (config) {
                    token.value = config.access_token
                    user.value = {
                        name: config.user.login,
                        avatar: config.user.avatar_url,
                    }
                    saveToLocalStorage()
                }
            } catch (error) {
                console.error('Failed to load auth config:', error)
            }
        }
    }

    return {
        user,
        token,
        isLoggedIn,
        login,
        logout,
        loadConfig,
    }
})
