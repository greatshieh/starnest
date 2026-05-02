<script setup lang="ts">
import { themeColors } from '@/constants/colors'
import { useAppStore } from '@/stores/apps'

const appStore = useAppStore()

const selectTheme = (themeMode: string) => {
    appStore.updateThemeMode(themeMode)
}

const selectColor = (themeName: string) => {
    appStore.updateThemeColor(themeName)
}
</script>

<template>
    <div class="flex-1 py-6 pl-4">
        <h2 class="text-lg font-bold text-t-primary">Appearance</h2>

        <Panel title="Theme" description="Choose your preferred theme.">
            <div class="grid grid-cols-3 gap-4 mt-4 mr-12">
                <div
                    class="group relative bg-[#ebeef2] rounded-xl p-4 cursor-pointer border border-solid border-[#ffffff]/24 transition-all"
                    :class="{ 'border-primary': appStore.themeMode === 'light' }"
                    @click="selectTheme('light')">
                    <div
                        class="aspect-video rounded-lg bg-[#fafbfc] mb-3 mx-2 flex items-center justify-center overflow-hidden border border-solid border-[#d0d7de]">
                        <div class="w-full h-full p-2">
                            <div class="h-2 w-1/2 bg-[#d0d7de] rounded"></div>
                            <div class="h-8 w-full bg-[#d8dee4] rounded mt-1"></div>
                            <div class="h-2 w-3/4 bg-[#d0d7de] rounded mt-1"></div>
                        </div>
                    </div>
                    <span
                        class="block text-center text-md tracking-widest transition-colors"
                        :class="
                            appStore.themeMode === 'light' ? 'text-[var(--sv-primary)] font-bold' : 'text-[#1f2328]'
                        "
                        >Light</span
                    >
                    <div
                        v-if="appStore.themeMode === 'light'"
                        class="absolute top-2 right-2 h-4 w-4 rounded-full flex items-center justify-center border border-solid border-primary">
                        <span class="i-md-check text-[12px] text-primary"></span>
                    </div>
                </div>

                <div
                    class="group relative bg-[#30313a] rounded-xl p-4 cursor-pointer border border-solid transition-all"
                    :class="{ 'border-primary': appStore.themeMode === 'dark' }"
                    @click="selectTheme('dark')">
                    <div
                        class="aspect-video rounded-lg mb-3 mx-2 flex items-center justify-center overflow-hidden border border-solid border-[#4a4a55]">
                        <div class="w-full h-full p-2">
                            <div class="h-2 w-1/2 bg-[#4a4a55] rounded"></div>
                            <div class="h-8 w-full bg-[#3f3f46] rounded mt-1"></div>
                            <div class="h-2 w-3/4 bg-[#4a4a55] rounded mt-1"></div>
                        </div>
                    </div>
                    <span
                        class="block text-center text-md tracking-widest transition-colors"
                        :class="appStore.themeMode === 'dark' ? 'text-primary font-bold' : 'text-[#a1a1aa]'"
                        >Dark</span
                    >
                    <div
                        v-if="appStore.themeMode === 'dark'"
                        class="absolute top-2 right-2 h-4 w-4 rounded-full flex items-center justify-center border border-solid border-primary">
                        <span class="i-md-check text-[12px] text-primary"></span>
                    </div>
                </div>

                <div
                    class="group relative bg-card rounded-xl p-4 cursor-pointer border-light transition-all"
                    @click="selectTheme('system')">
                    <div
                        class="aspect-video rounded-lg bg-card mb-3 mx-2 flex items-center justify-center overflow-hidden border-light">
                        <span
                            class="i-md-settings-brightness text-3xl"
                            :class="appStore.themeMode === 'system' ? 'text-primary' : 'text-fg'"></span>
                    </div>
                    <span
                        class="block text-center text-md tracking-widest transition-colors"
                        :class="appStore.themeMode === 'system' ? 'text-primary font-bold' : 'text-t-secondary'"
                        >System</span
                    >
                    <div
                        v-if="appStore.themeMode === 'system'"
                        class="absolute top-2 right-2 h-4 w-4 rounded-full flex items-center justify-center border border-solid border-primary">
                        <span class="i-md-check text-[12px] text-primary"></span>
                    </div>
                </div>
            </div>
        </Panel>

        <Panel title="Theme Color" description="Choose your preferred theme color.">
            <div class="flex items-center flex-wrap gap-4 justify-between mt-4 mr-12">
                <div
                    v-for="theme in themeColors"
                    :key="theme.name"
                    class="w-23% h-12 rounded-md flex items-center justify-center cursor-pointer relative"
                    @click="selectColor(theme.name)"
                    :style="{ backgroundColor: `${theme.color}` }">
                    <div
                        v-if="appStore.themeColor === theme.name"
                        class="absolute -top-2 -right-2 size-6 rounded-full border border-solid border-success text-center bg-success">
                        <span class="i-md-check m-auto text-[16px] text-[#000] dark:text-white font-bold"></span>
                    </div>
                    <span class="text-md text-[#000] dark:text-white">{{ theme.name }}</span>
                </div>
            </div>
        </Panel>
    </div>
</template>
