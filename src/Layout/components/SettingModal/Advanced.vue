<script setup lang="ts">
import { ref } from 'vue'
import { useSettingsStore } from '@/stores/settings'
import { useMessage } from '@/composables/useMessage'

const settingsStore = useSettingsStore()
const { success, error } = useMessage()
const showConfirmDialog = ref(false)

const clearCache = () => {
    console.log('Clear cache')
}

const resetSettings = () => {
    showConfirmDialog.value = true
}

const confirmReset = async () => {
    try {
        await settingsStore.resetConfig()
        success('Settings have been reset to default')
    } catch (err) {
        error('Failed to reset settings')
    } finally {
        showConfirmDialog.value = false
    }
}

const cancelReset = () => {
    showConfirmDialog.value = false
}
</script>

<template>
    <div class="flex-1 py-6 pl-4">
        <h2 class="text-lg font-bold text-t-primary">Advanced</h2>

        <Panel
            inline
            title="Cache Duration"
            description="How long cached data remains valid (in hours)"
            columns="1fr 1fr">
            <div class="ml-auto flex items-center gap-1">
                <button @click="settingsStore.cacheDuration = Math.max(1, settingsStore.cacheDuration - 1)">
                    <span class="i-md-minus text-sm text-t-secondary"></span>
                </button>
                <span class="font-bold text-t-primary mx-4">{{ settingsStore.cacheDuration }}</span>
                <button @click="settingsStore.cacheDuration = Math.min(168, settingsStore.cacheDuration + 1)">
                    <span class="i-md-add text-sm text-t-secondary"></span>
                </button>
                <span class="font-bold text-t-primary ml-2">hours</span>
            </div>
        </Panel>

        <Panel>
            <div class="flex items-center justify-center gap-8">
                <button class="warning" @click="clearCache">
                    <span class="i-md-trash-can text-lg"></span>
                    <span>Clear Cache</span>
                </button>
                <button class="error" @click="resetSettings">
                    <span class="i-md-reset text-lg"></span>
                    <span>Reset All Settings</span>
                </button>
            </div>
        </Panel>

        <Modal v-model="showConfirmDialog" title="Confirm Reset" width="480px">
            <div class="py-4">
                <p class="text-t-secondary mb-4">
                    Are you sure you want to reset all settings to default?
                </p>
                <p class="text-t-placeholder text-sm mb-6">
                    This will restore all settings except your account information (username and token) to their default values.
                </p>
                <div class="flex justify-end gap-4">
                    <button class="secondary" @click="cancelReset">Cancel</button>
                    <button class="error" @click="confirmReset">Confirm Reset</button>
                </div>
            </div>
        </Modal>
    </div>
</template>