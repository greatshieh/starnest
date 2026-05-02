<script setup lang="ts">
/**
 * 选择器组件 - 支持单选和多选模式的现代化下拉选择器
 * 提供美观、可访问的下拉选择功能，支持浮动定位和响应式布局
 */

import { arrow, autoPlacement, autoUpdate, flip, offset, shift, useFloating } from '@floating-ui/vue'
import { computed, onMounted, onUnmounted, PropType, ref, shallowRef, StyleValue, useAttrs } from 'vue'
import { SelectOption } from '.'

/**
 * 组件选项配置
 * 设置组件名称为MySelect
 */
defineOptions({
    name: 'MySelect',
})

/**
 * 组件属性定义
 * @property {SelectOption[]} options - 选择项数组
 * @property {string} placeholder - 占位符文本
 * @property {boolean} multiple - 是否支持多选
 * @property {number} maxTagCount - 最大显示标签数量
 * @property {string | string[]} modelValue - 双向绑定值
 */
const props = defineProps({
    options: {
        type: Object as PropType<SelectOption[] | string[]>,
        default: () => {
            return [] as SelectOption[]
        },
    },
    placeholder: {
        type: String,
        default: 'Please select a value',
    },
    multiple: {
        type: Boolean,
        default: false,
    },
    maxTagCount: {
        type: Number,
        default: 5,
    },
    modelValue: {
        type: [String, Array] as PropType<string | string[]>,
        default: '',
    },
    clearable: {
        type: Boolean,
        default: false,
    },
})

/**
 * 事件定义
 */
const emit = defineEmits<{
    'update:modelValue': [value: string | string[]]
    change: [value: string | string[]]
}>()

const selectOptions = computed(() => {
    return props.options.map(item => {
        if (typeof item === 'string') {
            return {
                label: item,
                value: item,
            }
        }
        return item
    })
})

/**
 * 多选模式下的选中值数组
 */
const selectedValues = computed<string[]>({
    get: () => {
        const value = props.modelValue
        if (props.multiple) {
            return Array.isArray(value) ? [...value] : []
        }
        const val = value as string
        return val ? [val] : []
    },
    set: (val: string[]) => {
        if (props.multiple) {
            emit('update:modelValue', [...val])
        } else {
            emit('update:modelValue', val[0] || '')
        }
    },
})

/**
 * 获取选中的选项对象
 */
const selectedOptions = computed(() => {
    return selectOptions.value.filter(item => selectedValues.value.includes(item.value))
})

/**
 * 样式处理
 */
const rawAttrs = useAttrs()
const containerStyle = computed(() => rawAttrs.style as StyleValue)

/**
 * 下拉框展开状态
 */
const isOpen = ref(false)

/**
 * 搜索筛选输入值
 */
const searchQuery = ref('')

/**
 * DOM元素引用
 */
const selRef = shallowRef<HTMLDivElement | null>(null)
const selFloating = shallowRef<HTMLDivElement | null>(null)
const arrowRef = shallowRef<HTMLDivElement | null>(null)
const searchInputRef = shallowRef<HTMLInputElement | null>(null)

/**
 * Floating UI配置
 */
const { floatingStyles, middlewareData, placement } = useFloating(selRef, selFloating, {
    whileElementsMounted: autoUpdate,
    middleware: [
        autoPlacement({ allowedPlacements: ['bottom', 'top'] }),
        flip(),
        shift({ padding: 5 }),
        offset(5),
        arrow({ element: arrowRef }),
    ],
})

/**
 * 容器宽度跟踪
 */
const containerWidth = ref<number>(0)

function updateContainerWidth() {
    if (selRef.value) {
        containerWidth.value = selRef.value.clientWidth
    }
}

function handleResize() {
    updateContainerWidth()
}

const dropdownStyle = computed(() => {
    const styles: Record<string, string> = {}
    if (containerWidth.value > 0) {
        styles.width = `${containerWidth.value}px`
    }
    return styles
})

/**
 * 筛选后的选项列表
 */
const filteredOptions = computed(() => {
    if (!searchQuery.value.trim()) {
        return selectOptions.value
    }
    const query = searchQuery.value.toLowerCase()
    return selectOptions.value.filter(
        item => item.label.toLowerCase().includes(query) || item.value.toLowerCase().includes(query),
    )
})

/**
 * 排序后的选项列表
 */
const sortedOptions = computed(() => {
    return [...filteredOptions.value].sort((a, b) => {
        const aSelected = selectedValues.value.includes(a.value)
        const bSelected = selectedValues.value.includes(b.value)
        if (aSelected && !bSelected) return -1
        if (!aSelected && bSelected) return 1
        return 0
    })
})

/**
 * 判断选项是否被选中
 */
function isSelected(value: string): boolean {
    return selectedValues.value.includes(value)
}

/**
 * 选择项处理函数
 */
function handleItemClick(item: SelectOption) {
    if (props.multiple) {
        const currentValues = [...selectedValues.value]
        const index = currentValues.indexOf(item.value)
        if (index > -1) {
            currentValues.splice(index, 1)
        } else {
            currentValues.push(item.value)
        }
        selectedValues.value = currentValues
        emit('change', currentValues)
    } else {
        selectedValues.value = [item.value]
        isOpen.value = false
        emit('change', item.value)
    }
}

/**
 * 移除单个选中项
 */
function removeSelectedItem(value: string) {
    console.log(value)
    selectedValues.value = selectedValues.value.filter(v => v !== value)
    emit('change', selectedValues.value)
}

/**
 * 清空所有选中项
 */
function clearSelectedItem() {
    emit('update:modelValue', '')
    emit('change', [])
}

/**
 * 切换下拉框状态
 */
function toggleDropdown() {
    isOpen.value = !isOpen.value
    if (isOpen.value) {
        updateContainerWidth()
        setTimeout(() => {
            searchInputRef.value?.focus()
        }, 100)
    } else {
        searchQuery.value = ''
    }
}

/**
 * 计算属性：单选模式下的选中项显示文本
 */
const selectedLabel = computed(() => {
    return selectOptions.value.find(item => item.value === props.modelValue)?.label || props.placeholder
})

/**
 * 计算属性：多选模式下显示的标签数量和溢出数量
 */
const visibleTags = computed(() => {
    return selectedOptions.value.slice(0, props.maxTagCount)
})

const overflowCount = computed(() => {
    return Math.max(0, selectedOptions.value.length - props.maxTagCount)
})

const arrowPositon = computed(() => {
    const staticSide = {
        top: 'bottom',
        right: 'left',
        bottom: 'top',
        left: 'right',
    }[placement.value.split('-')[0]]
    return staticSide!
})

/**
 * 点击外部关闭下拉框函数
 */
function clickOutSide() {
    isOpen.value = false
    searchQuery.value = ''
}

/**
 * 尺寸观察器实例
 */
let resizeObserver: ResizeObserver | null = null

/**
 * 组件挂载生命周期
 */
onMounted(() => {
    updateContainerWidth()
    window.addEventListener('resize', handleResize)

    if (selRef.value) {
        resizeObserver = new ResizeObserver(() => {
            updateContainerWidth()
        })
        resizeObserver.observe(selRef.value)
    }
})

/**
 * 组件卸载生命周期
 */
onUnmounted(() => {
    window.removeEventListener('resize', handleResize)

    if (resizeObserver && selRef.value) {
        resizeObserver.unobserve(selRef.value)
        resizeObserver.disconnect()
    }
})
</script>

<template>
    <!-- 选择器主容器 -->
    <div ref="selRef" v-click-outside="clickOutSide" class="select relative w-full" :style="containerStyle">
        <!-- 选择器包装器 -->
        <div class="select__wrapper" :class="{ 'is-focused': isOpen }" @click="toggleDropdown">
            <!-- 选择区域 -->
            <div class="selection">
                <!-- 多选模式：选中标签列表 -->
                <template v-if="props.multiple">
                    <!-- 已选中标签 -->
                    <template v-if="selectedOptions.length > 0">
                        <!-- 可见标签 -->
                        <span
                            v-for="item in visibleTags"
                            :key="item.value"
                            class="flex items-center gap-2 bg-[var(--sv-tag-color)] px-2 py-1 rounded-md text-[var(--sv-tag-text-color)]">
                            <span>{{ item.label }}</span>
                            <span
                                class="i-md-close text-sm cursor-pointer hover:bg-popover rounded-full"
                                @click.stop="removeSelectedItem(item.value)"></span>
                        </span>
                        <!-- 溢出标签数量 -->
                        <span v-if="overflowCount > 0" class="select__tag select__tag-overflow">
                            +{{ overflowCount }}
                        </span>
                        <!-- 清空按钮 -->
                        <span
                            v-if="clearable"
                            class="i-md-close text-sm cursor-pointer hover:bg-popover rounded-full ml-1"
                            @click.stop="clearSelectedItem()"></span>
                    </template>

                    <!-- 占位符 -->
                    <div v-if="selectedOptions.length === 0" class="select__placeholder">
                        <span>{{ props.placeholder }}</span>
                    </div>
                </template>

                <!-- 单选模式：显示选中项或占位符 -->
                <template v-else>
                    <div :class="selectedLabel !== placeholder ? 'select__selected' : 'select__placeholder'">
                        <span
                            v-if="clearable && selectedLabel !== placeholder"
                            class="flex items-center gap-2 bg-[var(--sv-tag-color)] px-2 py-1 rounded-md text-[var(--sv-tag-text-color)]">
                            <span>{{ selectedLabel }}</span>
                            <span
                                class="i-md-close text-sm cursor-pointer hover:bg-popover rounded-full"
                                @click.stop="clearSelectedItem()"></span>
                        </span>
                        <span v-else>{{ selectedLabel }}</span>
                    </div>
                </template>
            </div>

            <!-- 后缀图标区域 -->
            <div class="suffix">
                <!-- 下拉箭头图标 -->
                <i class="my-icon select__caret" :class="{ 'is-reverse': isOpen }">
                    <svg
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round">
                        <path d="m6 9 6 6 6-6" />
                    </svg>
                </i>
            </div>
        </div>

        <!-- 下拉框浮动容器 -->
        <teleport to="body">
            <div
                v-show="isOpen && (props.options.length > 0 || $slots.default)"
                ref="selFloating"
                class="select-dropdown"
                :style="[floatingStyles, dropdownStyle]">
                <!-- 多选模式：搜索框 -->
                <div v-if="props.multiple" class="select-dropdown__search">
                    <input
                        ref="searchInputRef"
                        v-model="searchQuery"
                        type="text"
                        placeholder="搜索选项..."
                        class="select-dropdown__search-input" />
                </div>

                <!-- 下拉选项列表 -->
                <ul class="select-dropdown__list" role="listbox" aria-orientation="vertical">
                    <slot v-if="!props.options && $slots.default" name="default"></slot>
                    <!-- 选项项循环 -->
                    <li
                        v-else
                        v-for="item in sortedOptions"
                        :key="item.value"
                        class="select-dropdown__item"
                        :class="{
                            'is-selected': isSelected(item.value),
                        }"
                        role="option"
                        aria-selected="false"
                        @click="handleItemClick(item)">
                        <!-- 多选模式：选中复选框图标 -->
                        <span v-if="props.multiple" class="select-dropdown__checkbox">
                            <span
                                v-if="isSelected(item.value)"
                                class="i-md-check"
                                style="color: var(--sv-primary-solid)"></span>
                        </span>
                        <span>{{ item.label }}</span>
                    </li>

                    <!-- 无结果提示 -->
                    <li v-if="sortedOptions.length === 0" class="select-dropdown__empty">暂无匹配选项</li>
                </ul>

                <!-- 多选模式：底部统计信息 -->
                <div v-if="props.multiple && selectOptions.length > 0" class="select-dropdown__footer">
                    <span class="select-dropdown__count">
                        共{{ selectOptions.length }}项，选中{{ selectedOptions.length }}项
                    </span>
                </div>

                <!-- 浮动箭头指示器 -->
                <div
                    id="arrow"
                    ref="arrowRef"
                    :style="{
                        left: middlewareData.arrow?.x != null ? `${middlewareData.arrow.x}px` : '',
                        top: middlewareData.arrow?.y != null ? `${middlewareData.arrow.y}px` : '',
                        [arrowPositon]: '-4px',
                    }"></div>
            </div>
        </teleport>
    </div>
</template>

<!-- 选择器组件样式 -->
<style lang="css" scoped>
.select {
    position: relative;
    display: inline-block;
    vertical-align: middle;
    font-family: 'Inter', sans-serif;
}

.select .select__wrapper {
    position: relative;
    box-sizing: border-box;
    display: flex;
    gap: 8px;
    align-items: center;
    min-height: 24px;
    padding: 4px 8px;
    font-size: 14px;
    line-height: 20px;
    text-align: left;
    cursor: pointer;
    background-color: var(--sv-card-color);
    border-radius: 6px;
    border: 1px solid var(--sv-border-color);
    transform: translateZ(0);
    transition: all 0.2s ease;
}

.select .select__wrapper.is-focused {
    border-color: var(--sv-primary-solid);
    box-shadow: var(--sv-box-shadow-1);
}

.select .select__wrapper:hover {
    border-color: var(--sv-primary-hover-solid);
}

.select .select__wrapper:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    background-color: var(--sv-input-color-disabled);
}

.select .selection {
    position: relative;
    display: flex;
    flex: 1;
    flex-wrap: wrap;
    gap: 4px;
    align-items: center;
    min-width: 0;
    padding: 2px 0;
}

.select .selection .select__tag {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 2px 8px;
    font-size: 12px;
    font-weight: 500;
    background-color: var(--sv-tag-color);
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.15s ease;
}

.select .selection .select__tag:hover {
    opacity: 0.8;
}

.select .selection .select__tag-text {
    color: var(--sv-text-color-1);
}

.select .selection .select__tag-close {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 16px;
    height: 16px;
    color: var(--sv-text-color-2);
    border-radius: 50%;
    transition: background-color 0.15s ease;
}

.select .selection .select__tag-close:hover {
    background-color: rgba(0, 0, 0, 0.1);
}

.select .selection .select__tag-overflow {
    background-color: var(--sv-border-color);
    color: var(--sv-text-color-2);
}

.select .selection .select__selected {
    display: block;
    overflow: hidden;
    text-overflow: ellipsis;
    color: var(--sv-text-color-1);
    font-weight: 500;
    white-space: nowrap;
}

.select .selection .select__placeholder {
    color: var(--sv-placeholder-color);
}

.suffix {
    display: flex;
    flex-shrink: 0;
    gap: 4px;
    align-items: center;
    color: var(--sv-text-color-2);
}

.select .suffix .select__clear-all {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.15s ease;
}

.select .suffix .select__clear-all:hover {
    background-color: var(--sv-hover-color);
}

.suffix .select__caret {
    font-size: 16px;
    color: var(--sv-text-color-2);
    cursor: pointer;
    transform: rotate(0);
    transition: transform 0.2s ease;
    flex-shrink: 0;
}

.suffix .select__caret.is-reverse {
    transform: rotate(180deg);
}

.select-dropdown {
    z-index: 2026;
    box-sizing: border-box;
    min-width: 160px;
    max-width: 500px;
    font-size: 14px;
    line-height: 20px;
    word-break: normal;
    overflow-wrap: break-word;
    background: var(--sv-popover-color);
    border: 1px solid var(--sv-border-color);
    border-radius: 0.5rem;
    box-shadow: var(--sv-shadow-sm);
}

.select-dropdown__search {
    padding: 8px;
    border-bottom: 1px solid var(--sv-border-color);
}

.select-dropdown__search-input {
    width: 100%;
    padding: 6px 10px;
    font-size: 13px;
    border: 1px solid var(--sv-border-color);
    border-radius: 4px;
    background-color: var(--sv-card-color);
    color: var(--sv-text-color-1);
    outline: none;
    transition: border-color 0.2s ease;
}

.select-dropdown__search-input:focus {
    border-color: var(--sv-primary-solid);
}

.select-dropdown__search-input::placeholder {
    color: var(--sv-placeholder-color);
}

.select-dropdown #arrow {
    position: absolute;
    width: 10px;
    height: 10px;
    background: var(--sv-popover-color);
    border-top: 1px solid var(--sv-border-color);
    border-left: 1px solid var(--sv-border-color);
    transform: rotate(45deg);
    z-index: -1;
}

.select-dropdown .select-dropdown__list {
    box-sizing: border-box;
    max-height: 280px;
    padding: 4px 0;
    margin: 0;
    overflow-y: auto;
    overflow-x: hidden;
    list-style: none;
}

.select-dropdown .select-dropdown__list .select-dropdown__item {
    position: relative;
    box-sizing: border-box;
    height: 40px;
    padding: 0 16px;
    overflow: hidden;
    text-overflow: ellipsis;
    font-size: 14px;
    font-weight: 500;
    line-height: 40px;
    color: var(--sv-text-color-1);
    white-space: nowrap;
    cursor: pointer;
    transition: background-color 0.15s ease;
    display: flex;
    align-items: center;
    gap: 10px;
}

.select-dropdown__list .select-dropdown__item .select-dropdown__checkbox {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 18px;
    height: 18px;
    border: 2px solid var(--sv-border-color);
    border-radius: 4px;
    flex-shrink: 0;
    transition: all 0.15s ease;
}

.select-dropdown__list .select-dropdown__item.is-selected .select-dropdown__checkbox {
    border-color: var(--sv-primary-solid);
    background-color: rgb(var(--sv-primary) / 0.15);
}

.select-dropdown .select-dropdown__list .select-dropdown__item.is-selected {
    color: var(--sv-primary-solid);
    background-color: rgb(var(--sv-primary) / 0.15);
    font-weight: 600;
}

.select-dropdown .select-dropdown__list .select-dropdown__item.is-selected::before {
    content: '';
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 3px;
    background-color: var(--sv-primary-solid);
}

.select-dropdown .select-dropdown__list .select-dropdown__item:not(.is-selected):hover {
    background-color: var(--sv-hover-color);
}

.select-dropdown .select-dropdown__list .select-dropdown__item:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.select-dropdown .select-dropdown__list .select-dropdown__empty {
    padding: 16px;
    text-align: center;
    color: var(--sv-placeholder-color);
    font-style: italic;
}

.select-dropdown__footer {
    padding: 8px 16px;
    border-top: 1px solid var(--sv-border-color);
    background-color: var(--sv-card-color);
}

.select-dropdown__count {
    font-size: 12px;
    color: var(--sv-text-color-2);
}

@media (max-width: 640px) {
    .select {
        width: 100%;
    }

    .select .select__wrapper {
        padding: 8px 12px;
        font-size: 13px;
    }

    .select-dropdown {
        font-size: 13px;
    }

    .select-dropdown .select-dropdown__list .select-dropdown__item {
        padding: 0 12px;
    }
}

@media (min-width: 641px) and (max-width: 1024px) {
    .select .select__wrapper {
        padding: 8px 14px;
    }
}
</style>
