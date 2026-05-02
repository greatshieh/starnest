<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import type { api as ApiTypes } from '@/types'
import { presetColors } from '@/constants/colors'
import Modal from '@/components/Modal.vue'
import MyInput from '@/components/MyInput.vue'

const props = defineProps<{
    show: boolean
    editingCollection?: (ApiTypes.collection.CollectionWithRepoCount & { repos: ApiTypes.repo.Repository[] }) | null
    allRepos: ApiTypes.repo.Repository[]
}>()

const emit = defineEmits<{
    'update:show': [value: boolean]
    save: [data: { name: string; desc: string; color: string; repoIds: number[] }]
}>()

// 表单数据
const formName = ref('')
const formDesc = ref('')
const formColor = ref(presetColors[0])
const formRepoIds = ref<number[]>([])
const repoSearchQuery = ref('')
const hoveredRepoId = ref<number | null>(null)

// 过滤后的可选仓库
const filteredRepos = computed(() => {
    if (!repoSearchQuery.value.trim()) return props.allRepos
    const q = repoSearchQuery.value.trim().toLowerCase()
    return props.allRepos.filter(r => r.full_name.toLowerCase().includes(q))
})

// 格式化数字
const formatNumber = (num: number): string => {
    if (num >= 10000) {
        return (num / 10000).toFixed(1) + 'k'
    }
    return num.toString()
}

// 仓库选择切换
const toggleRepo = (githubId: number) => {
    const idx = formRepoIds.value.indexOf(githubId)
    if (idx === -1) {
        formRepoIds.value.push(githubId)
    } else {
        formRepoIds.value.splice(idx, 1)
    }
}

// 保存
const handleSave = () => {
    if (!formName.value.trim()) return
    emit('save', {
        name: formName.value.trim(),
        desc: formDesc.value.trim(),
        color: formColor.value,
        repoIds: formRepoIds.value,
    })
}

// 重置表单
const resetForm = () => {
    formName.value = props.editingCollection?.name || ''
    formDesc.value = props.editingCollection?.description || ''
    const editingId = props.editingCollection?.id ?? 0
    formColor.value = presetColors[editingId % presetColors.length]
    formRepoIds.value = props.editingCollection?.repos.map(r => r.github_id) || []
    repoSearchQuery.value = ''
}

// 监听编辑对象变化
watch(
    () => [props.show, props.editingCollection],
    () => {
        if (props.show) {
            resetForm()
        }
    },
)
</script>

<template>
    <Modal
        :modelValue="show"
        @update:modelValue="emit('update:show', $event)"
        width="auto"
        height="auto"
        :showHeader="false">
        <div class="relative p-5 overflow-hidden" style="width: 460px; max-height: 80vh">
            <h3 class="text-sm font-semibold mb-4" style="color: var(--sv-text)">
                {{ editingCollection ? 'Edit Collection' : 'New Collection' }}
            </h3>

            <!-- 合集名称 -->
            <div class="mb-6">
                <label class="mb-4 block">Collection Name</label>
                <MyInput v-model="formName" placeholder="Enter collection name" />
            </div>

            <!-- 描述 -->
            <div class="mb-6">
                <label class="mb-4 block">Description</label>
                <MyInput type="textarea" v-model="formDesc" placeholder="Briefly describe the collection..." />
            </div>

            <!-- 颜色 -->
            <div class="mb-6">
                <label class="mb-4 block">Color</label>
                <div class="flex flex-wrap gap-2">
                    <div
                        v-for="color in presetColors"
                        :key="color"
                        class="size-7 rounded-full cursor-pointer transition-transform hover:scale-110 flex items-center justify-center"
                        :style="{
                            background: color,
                            boxShadow:
                                formColor === color ? '0 0 0 2px var(--sv-card-color), 0 0 0 4px ' + color : 'none',
                        }"
                        @click="formColor = color">
                        <span v-if="formColor === color" class="i-md-check text-white"></span>
                    </div>
                </div>
            </div>

            <!-- 已包含仓库 -->
            <div class="mb-4">
                <label class="mb-4 block">
                    Included Repos <span>({{ formRepoIds.length }} repos)</span>
                </label>
                <!-- 搜索仓库 -->
                <MyInput v-model="repoSearchQuery" placeholder="Search repos..." />
                <!-- 仓库列表 -->
                <div
                    class="overflow-y-auto rounded-md mt-4"
                    style="max-height: 160px; border: 1px solid var(--sv-border-color)">
                    <div
                        v-for="repo in filteredRepos"
                        :key="repo.github_id"
                        class="flex items-center gap-2 px-3 py-1.5 text-xs cursor-pointer transition-colors"
                        :style="{
                            background: formRepoIds.includes(repo.github_id)
                                ? 'rgb(var(--sv-primary)/0.15)'
                                : hoveredRepoId === repo.github_id
                                  ? 'var(--sv-popover-color)'
                                  : 'var(--sv-card-color)',
                            borderBottom: '1px solid var(--sv-border-color)',
                        }"
                        @mouseenter="hoveredRepoId = repo.github_id"
                        @mouseleave="hoveredRepoId = null"
                        @click="toggleRepo(repo.github_id)">
                        <!-- 复选框 -->
                        <span
                            class="size-4 rounded-full shrink-0 flex items-center justify-center"
                            :style="{
                                background: formRepoIds.includes(repo.github_id)
                                    ? 'var(--sv-primary-solid)'
                                    : 'transparent',
                                border: formRepoIds.includes(repo.github_id)
                                    ? '1px solid var(--sv-primary-solid)'
                                    : '1px solid var(--sv-border-color)',
                            }">
                            <span v-if="formRepoIds.includes(repo.github_id)" class="i-md-check text-white"></span>
                        </span>
                        <span class="truncate flex-1" style="color: var(--sv-text-color-1)">{{ repo.full_name }}</span>
                        <span>{{ formatNumber(repo.stargazers_count) }}</span>
                    </div>
                </div>
            </div>

            <!-- 按钮 -->
            <div class="flex items-center justify-end gap-2">
                <button @click="emit('update:show', false)">Cancel</button>
                <button class="primary" @click="handleSave">Save</button>
            </div>
        </div>
    </Modal>
</template>
