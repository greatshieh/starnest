<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { flip, offset, useFloating } from '@floating-ui/vue'
import { useCategoriesStore } from '@/stores/categories'
import { presetColors } from '@/constants/colors'

const props = defineProps<{
    repoId?: number
    relatedCategoryIds?: number[]
}>()

const visible = defineModel({
    type: Boolean,
    default: false,
})

const emit = defineEmits<{
    (e: 'close'): void
    (
        e: 'confirm',
        categories: Array<{ id: number; name: string; color: string; count: number; checked: boolean }>,
    ): void
}>()

const store = useCategoriesStore()
const newCategoryName = ref('')
const selectedColor = ref(presetColors[0])

// 使用 floating-ui 管理浮动元素的位置
const reference = ref<HTMLInputElement | null>(null)
const floating = ref<HTMLElement | null>(null)

const { floatingStyles } = useFloating(reference, floating, {
    placement: 'bottom',
    strategy: 'absolute',
    middleware: [
        flip({
            fallbackPlacements: ['top', 'bottom'],
        }),
        offset(5),
    ],
})

// 虚拟滚动相关
const containerRef = ref<HTMLElement | null>(null)
const scrollTop = ref(0)
const itemHeight = 64 // 每个分类项的高度（包括padding）
const visibleCount = ref(10) // 可见项目数量

const startIndex = computed(() => {
    return Math.max(0, Math.floor(scrollTop.value / itemHeight))
})

const endIndex = computed(() => {
    return Math.min(store.categories.length, startIndex.value + visibleCount.value)
})

const visibleItems = computed(() => {
    return store.categories.slice(startIndex.value, endIndex.value)
})

const scrollHandler = () => {
    if (containerRef.value) {
        scrollTop.value = containerRef.value.scrollTop
    }
}

onMounted(() => {
    if (containerRef.value) {
        containerRef.value.addEventListener('scroll', scrollHandler)
        // 初始化可见项目数量
        visibleCount.value = Math.ceil(containerRef.value.clientHeight / itemHeight) + 2
    }
})

onUnmounted(() => {
    if (containerRef.value) {
        containerRef.value.removeEventListener('scroll', scrollHandler)
    }
    // 清空操作记录
    store.clearOperations()
})

const suggestions = computed(() => {
    if (!newCategoryName.value.trim()) return []
    const input = newCategoryName.value.toLowerCase()
    return store.categories.filter(c => c.name.toLowerCase().includes(input)).slice(0, 5)
})

const isEmpty = computed(() => store.categories.length === 0 && !store.isLoading)

const toggleCategory = (id: number) => {
    store.toggleCategory(id)
}

const selectColor = (color: string) => {
    selectedColor.value = color
}

const addCategory = async () => {
    if (!newCategoryName.value.trim()) return

    try {
        await store.addCategory(newCategoryName.value.trim(), selectedColor.value)
        newCategoryName.value = ''
    } catch (error) {
        console.error('Failed to create category:', error)
    }
}

const selectSuggestion = (category: {
    id: number
    name: string
    color: string
    repoCount: number
    selected: boolean
}) => {
    newCategoryName.value = ''

    // 滚动到该分类位置，让用户自己决定是否选中
    if (containerRef.value) {
        const index = store.categories.findIndex(c => c.id === category.id)
        if (index !== -1) {
            const targetScrollTop = index * itemHeight
            // 滚动到中间位置
            const containerHeight = containerRef.value.clientHeight
            const scrollTo = targetScrollTop - containerHeight / 2 + itemHeight / 2
            containerRef.value.scrollTo({
                top: Math.max(0, scrollTo),
                behavior: 'smooth',
            })
        }
    }
}

const deleteCategory = async (id: number) => {
    try {
        await store.deleteCategory(id)
    } catch (error) {
        console.error('Failed to delete category:', error)
    }
}

const handleConfirm = async () => {
    if (props.repoId && store.operations.length > 0) {
        try {
            await store.saveChanges(props.repoId)
        } catch (error) {
            console.error('Failed to save category changes:', error)
        }
    }

    // 转换为组件需要的格式
    const categoriesForEmit = store.categories.map(c => ({
        id: c.id,
        name: c.name,
        color: c.color,
        count: c.repoCount,
        checked: c.selected,
    }))

    emit('confirm', categoriesForEmit)
    emit('close')
}

const fetchCategories = async () => {
    await store.fetchCategories()
    // 根据 relatedCategoryIds 设置选中状态
    if (props.relatedCategoryIds) {
        store.setSelectedCategories(props.relatedCategoryIds)
    }
}

onMounted(() => {
    fetchCategories()
    newCategoryName.value = ''
})

onUnmounted(() => {
    // 关闭时清空操作记录
    store.clearOperations()
})
</script>

<template>
    <Modal v-model="visible" width="auto" height="auto" :show-header="false">
        <div class="w-full max-w-2xl rounded-md flex flex-col max-h-[870px] overflow-hidden">
            <header class="px-6 py-4 flex items-center justify-between bg-card">
                <h2 class="text-xl font-bold tracking-tight">Manage Categories</h2>
                <button @click="emit('close')">
                    <span class="i-md-close"></span>
                </button>
            </header>
            <div
                ref="containerRef"
                class="flex-1 max-h-80 min-h-[120px] overflow-y-auto px-6 py-4 relative bg-[var(--sv-page-color)]">
                <!-- 虚拟滚动内容 -->
                <div
                    class="relative"
                    :style="{
                        height: `${store.categories.length * itemHeight}px`,
                    }">
                    <div
                        v-for="(category, index) in visibleItems"
                        :key="category.id"
                        class="flex items-center gap-4 p-3 rounded-lg group hover:bg-hover transition-colors absolute left-0 right-0"
                        :style="{
                            top: `${(startIndex + index) * itemHeight}px`,
                        }">
                        <button class="cursor-grab active:cursor-grabbing opacity-50 group-hover:opacity-100">
                            <span class="i-md-drag-indicator text-lg"></span>
                        </button>
                        <div
                            class="relative inline-block w-5 h-5 rounded-md border-2 border-solid border-[var(--sv-border-color)] transition-colors cursor-pointer"
                            :class="category.selected ? 'border-primary bg-primary' : 'hover:border-primary'"
                            @click="toggleCategory(category.id)">
                            <span
                                v-if="category.selected"
                                class="absolute inset-0 flex items-center justify-center text-[#ffffff] text-sm">
                                <span class="i-md-check"></span>
                            </span>
                        </div>
                        <div
                            class="w-4 h-4 rounded-full"
                            :style="{
                                backgroundColor: category.color,
                            }"></div>
                        <span class="flex-1 font-medium">{{ category.name }}</span>
                        <span class="bg-[var(--sv-tag-color)] px-2 py-0.5 rounded-md tracking-wider"
                            >{{ category.repoCount }} Repos</span
                        >
                        <button
                            class="p-1 hover:text-[var(--sv-error-solid)] transition-colors"
                            @click="deleteCategory(category.id)">
                            <span class="i-md-delete text-lg"></span>
                        </button>
                    </div>
                </div>

                <!-- 空状态 -->
                <div
                    v-if="isEmpty"
                    class="flex flex-col items-center justify-center py-16 px-4 text-center absolute inset-0 bg-card">
                    <div class="w-16 h-16 rounded-full flex items-center justify-center mb-4">
                        <span class="i-md-local-offer text-3xl"></span>
                    </div>
                    <h4 class="font-bold text-lg mb-2">No categories yet</h4>
                    <p class="max-w-xs">
                        You haven't created any categories yet. Create one below to organize your repositories.
                    </p>
                </div>
            </div>
            <div class="px-6 py-5 bg-card">
                <h3 class="text-lg font-bold text-t-primary tracking-widest mb-4">Create New Category</h3>
                <div class="flex flex-wrap items-center gap-4">
                    <div class="flex-1 min-w-[200px] relative">
                        <MyInput
                            ref="reference"
                            v-model="newCategoryName"
                            class="w-full"
                            placeholder="e.g. Infrastructure" />
                        <div
                            ref="floating"
                            v-if="suggestions.length > 0"
                            class="absolute left-0 right-0 bg-popover rounded-lg border border-[var(--sv-border-color)] overflow-hidden z-10"
                            :style="floatingStyles">
                            <div
                                v-for="suggestion in suggestions"
                                :key="suggestion.id"
                                class="w-full flex items-center gap-2 p-2"
                                @click="selectSuggestion(suggestion)">
                                <div class="size-3 rounded-full" :style="{ backgroundColor: suggestion.color }"></div>
                                <span>{{ suggestion.name }}</span>
                                <span class="text-xs ml-auto">{{ suggestion.repoCount }} repos</span>
                            </div>
                        </div>
                    </div>
                    <div class="flex items-center gap-2">
                        <div
                            v-for="color in presetColors"
                            :key="color"
                            class="rounded-full"
                            :class="
                                selectedColor === color
                                    ? 'size-8 ring-[[' +
                                      color +
                                      '] border-2 border-white ring-2 ring-offset-2 ring-offset-[#e2e7ff]'
                                    : 'size-6 hover:scale-110 transition-transform'
                            "
                            :style="{ backgroundColor: color }"
                            @click="selectColor(color)"></div>
                    </div>
                    <button class="primary" @click="addCategory">Add</button>
                </div>
            </div>
            <footer class="px-6 py-4 bg-[var(--sv-page-color)] flex items-center justify-end gap-4">
                <button @click="emit('close')">Cancel</button>
                <button class="primary" @click="handleConfirm">Done</button>
            </footer>
        </div>
    </Modal>
</template>
