<script setup lang="ts">
import { ref, computed, markRaw, defineAsyncComponent } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import type { Repository } from '@/stores/repos'
import { useReposStore } from '@/stores/repos'
import { formatNumber } from '@/utils/format'
import { openUrl } from '@tauri-apps/plugin-opener'
import { convertFileSrc } from '@tauri-apps/api/core'
import { api } from '@/api'
import { useMessage } from '@/composables/useMessage'

const statusMap: Record<string, string> = {
    active: 'success',
    archived: 'info',
    inactive: 'warning',
    deprecated: 'error',
}

const router = useRouter()
const route = useRoute()
const reposStore = useReposStore()

const { success, error: messageErr } = useMessage()

const owner = route.params.owner as string
const repoName = route.params.name as string
const repo = ref(JSON.parse(history.state.repo) as Repository)
const isStarred = ref(true)

// 当前激活的标签页
const activeTab = ref('readme')

// 标签页列表
const tabs = [
    {
        key: 'readme',
        label: 'README',
        component: markRaw(defineAsyncComponent(() => import('./components/ReadMeTab.vue'))),
    },
    {
        key: 'notes',
        label: 'Note',
        component: markRaw(defineAsyncComponent(() => import('./components/NoteTab.vue'))),
    },
    {
        key: 'progress',
        label: 'Activity',
        component: markRaw(defineAsyncComponent(() => import('./components/ProgressTab.vue'))),
    },
]

const avatarUrl = computed(() => {
    if (!repo.value) return ''
    return convertFileSrc(repo.value.owner_avatar_url || '')
})

// 返回列表：优先使用浏览器历史后退，无历史时跳转到仓库列表
function goBack() {
    router.back()
}

// 复制仓库名称到剪贴板
function copyToClipboard(repoName: string) {
    navigator.clipboard.writeText(`git clone https://github.com/${repoName}.git`)
    success('Copied to clipboard')
}

// 切换标星状态
async function toggleStar() {
    try {
        if (isStarred.value) {
            await api.repo.unstarRepo({ owner, repo_name: repoName })
            isStarred.value = false
            repo.value.stargazers_count = Math.max(0, repo.value.stargazers_count - 1)
            reposStore.removeRepo(owner, repoName)
            success('Unstarred')
        } else {
            await api.repo.starRepo({ owner, repo_name: repoName })
            isStarred.value = true
            repo.value.stargazers_count += 1
            success('Starred')
        }
    } catch (err) {
        console.error('Failed to toggle star:', err)
        messageErr('Operation failed')
    }
}
</script>

<template>
    <div class="p-6 overflow-y-auto h-full">
        <!-- 面包屑导航 -->
        <div class="flex items-center gap-2 mb-8 text-lg">
            <span class="cursor-pointer hover:underline" @click="goBack">Back</span>
            <span class="text-t-placeholder">/</span>
            <span>{{ repo.full_name }}</span>
        </div>

        <!-- 仓库头部信息 -->
        <div class="flex items-center justify-between mb-4 h-12">
            <div class="flex items-center justify-center gap-4">
                <img :src="avatarUrl" :alt="repo?.owner_login" class="size-16 rounded-full hidden 2xl:block" />
                <div>
                    <h1 class="text-xl font-bold" style="color: var(--sv-text)">{{ repo.name }}</h1>
                    <p class="mt-4 line-clamp-1 text-t-placeholder">
                        {{ repo.description }}
                    </p>
                </div>
            </div>

            <!-- 操作按钮 -->
            <div class="flex items-center gap-2 shrink-0">
                <button :class="isStarred ? 'primary' : 'plain'" @click="toggleStar">
                    <span class="text-lg" :class="isStarred ? 'i-md-star' : 'i-md-staroutline'"></span>
                    <span>{{ isStarred ? 'Starred' : 'Star' }}</span>
                </button>
                <button class="plain" @click="copyToClipboard(repo.full_name)">
                    <span class="i-md-copy text-lg"></span>
                    <span class="hidden 2xl:block">Copy</span>
                </button>
                <button class="plain" @click="repo.homepage && openUrl(repo.homepage)">
                    <span class="i-md-cloud-download text-lg"></span>
                    <span class="hidden 2xl:block">Download</span>
                </button>
                <button v-if="repo.homepage && repo.homepage !== ''" class="plain">
                    <span class="i-md-open-in-new text-lg"></span>
                    <span class="hidden 2xl:block">Home Page</span>
                </button>
                <button class="plain" @click="openUrl(`https://zread.ai/${owner}/${repoName}`)">
                    <span class="i-md-zread text-lg"></span>
                    <span class="hidden 2xl:block">Open on Zread</span>
                </button>
                <button class="plain" @click="openUrl(`https://deepwiki.ai/${owner}/${repoName}`)">
                    <span class="i-md-deepwiki text-lg"></span>
                    <span class="hidden 2xl:block">Open on DeepWiki</span>
                </button>
                <button class="plain" @click="openUrl(`https://github.com/${owner}/${repoName}`)">
                    <span class="i-md-github text-lg"></span>
                    <span class="hidden 2xl:block">Open on GitHub</span>
                </button>
            </div>
        </div>

        <!-- 统计栏 -->
        <div class="flex items-center gap-6 my-6 px-4 py-2.5 rounded-md text-md bg-card border-light">
            <div class="flex items-center gap-1.5">
                <span class="i-md-star"></span>
                <span class="font-medium">{{ formatNumber(repo.stargazers_count) }}</span>
            </div>
            <div class="flex items-center gap-1.5">
                <span class="i-md-fork"></span>
                <span class="font-medium">{{ formatNumber(repo.forks_count) }}</span>
            </div>
            <div class="flex items-center gap-1.5">
                <span class="i-md-issues"></span>
                <span class="font-medium">{{ formatNumber(repo.open_issues_count || 0) }}</span>
            </div>
            <div class="flex items-center gap-1.5">
                <span class="i-md-liscense"></span>
                <span class="font-medium">{{ repo.license || 'No License' }}</span>
            </div>
            <div class="flex items-center gap-1.5">
                <span
                    class="inline-block size-2 rounded-full"
                    :style="{ backgroundColor: `var(--sv-${statusMap[repo.status]}-solid)` }"></span>
                <span>{{ repo.status }}</span>
            </div>
        </div>

        <!-- 标签页 -->
        <div class="flex items-center gap-0 mb-4">
            <MyTabs v-model="activeTab">
                <MyTabPane v-for="tab in tabs" :label="tab.label" :name="tab.key">
                    <component :is="tab.component" v-bind="{ repo }"></component>
                </MyTabPane>
            </MyTabs>
        </div>
    </div>
</template>

<style lang="css">
pre.vditor-reset {
    padding: 40px !important;
}
</style>
