<script setup lang="ts">
import { useAuthStore } from '@/stores/auth'
import { useSettingsStore } from '@/stores/settings'
import { ref, watch } from 'vue'

const settingsStore = useSettingsStore()
const authStore = useAuthStore()
const nameEnable = ref(false)
const tokenEnable = ref(false)

watch(
    () => [authStore.user.name, authStore.token],
    () => {
        settingsStore.saveConfig()
    },
)
</script>

<template>
    <div class="flex-1 py-6 pl-4">
        <h2 class="text-lg font-bold text-t-primary">Basic Settings</h2>

        <Panel inline title="Username">
            <span v-show="!nameEnable" class="bg-popover w-66% p-2">
                {{ authStore.user.name }}
            </span>
            <input
                v-click-outside="() => (nameEnable = false)"
                v-show="nameEnable"
                type="text"
                :value="authStore.user.name"
                class="w-66% p-2 rounded bg-popover"
                style="border: 1px solid var(--sv-border-color)"
                @blur="() => (nameEnable = false)" />

            <button class="w-33% ml-8" @click="() => (nameEnable = true)">{{ nameEnable ? 'Done' : 'Edit' }}</button>
        </Panel>

        <Panel inline title="Token">
            <span v-show="!tokenEnable" class="bg-popover w-66% p-2">
                {{ authStore.token }}
            </span>
            <input
                v-click-outside="() => (tokenEnable = false)"
                v-show="tokenEnable"
                type="text"
                :value="authStore.token"
                class="w-66% p-2 rounded bg-popover"
                style="border: 1px solid var(--sv-border-color)"
                @blur="() => (tokenEnable = false)" />

            <button class="w-33% ml-8" @click="() => (tokenEnable = true)">{{ tokenEnable ? 'Done' : 'Edit' }}</button>
        </Panel>
    </div>
</template>