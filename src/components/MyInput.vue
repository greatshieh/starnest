<script setup lang="ts">
import { ref } from 'vue'

defineOptions({
    name: 'MyInput',
})

defineProps({
    placeholder: {
        type: String,
        default: '请输入',
    },
    type: {
        type: String,
        default: 'text',
    },
})

const inputValue = defineModel({
    type: String,
    default: '',
})

const isFocus = ref(false)
const innerInputRef = ref<HTMLInputElement | null>(null)

function focus() {
    innerInputRef.value?.focus()
}

function blur() {
    innerInputRef.value?.blur()
}

defineExpose({
    focus,
    blur,
})
</script>

<template>
    <!-- 输入框外层容器 -->
    <div
        class="input"
        :class="{
            'input--suffix': $slots.suffix, // 如果有后缀插槽，添加后缀样式
            'input--prefix': $slots.prefix, // 如果有前缀插槽，添加前缀样式
        }">
        <!-- 输入框包装器，包含前缀、输入框和后缀 -->
        <div class="input__wrapper" :class="{ 'is-focus': isFocus }" tabindex="-1">
            <!-- 前缀区域，用于显示图标或其他内容 -->
            <span v-if="$slots.prefix" class="input__prefix">
                <span class="input__prefix-inner">
                    <!-- 前缀插槽 -->
                    <slot name="prefix" />
                </span>
            </span>
            <!-- 输入框主体 -->
            <input
                ref="innerInputRef"
                class="input__inner"
                :type="type"
                autocomplete="off"
                tabindex="0"
                :placeholder="placeholder"
                :value="inputValue"
                @input="inputValue = (<HTMLInputElement>$event.target).value"
                @focus="isFocus = true"
                @blur="isFocus = false" />
            <!-- 后缀区域，用于显示图标或其他内容 -->
            <span v-if="$slots.suffix" class="input__suffix">
                <span class="input__suffix-inner">
                    <!-- 后缀插槽 -->
                    <slot name="suffix" />
                </span>
            </span>
        </div>
    </div>
</template>

<style lang="css" scoped>
/* 输入框样式 */
.input {
    --input-height: 24px;
    --input-inner-height: calc(var(--input-height, 24px) - 2px);

    position: relative;
    box-sizing: border-box;
    display: inline-flex;
    width: 100%;
    font-size: 14px;
    line-height: 20px;
    vertical-align: middle;
    font-family: 'Inter', sans-serif;
}

/* 输入框包装器样式 */
.input .input__wrapper {
    display: inline-flex;
    flex-grow: 1;
    align-items: center;
    justify-content: center;
    padding: 8px 16px;
    cursor: text;
    background-color: var(--sv-card-color);
    background-image: none;
    border-radius: 6px;
    border: 1px solid var(--sv-border-color);
    transform: translateZ(0);
    transition: all 0.2s ease;
}

/* 鼠标悬停效果 */
.input .input__wrapper:hover {
    border-color: var(--sv-primary-hover-solid);
}

/* 获得焦点时的样式 */
.input .input__wrapper.is-focus {
    border-color: var(--sv-primary-hover-solid);
}

/* 禁用状态 */
.input .input__wrapper:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    background-color: var(--sv-input-color-disabled);
}

/* 前缀样式 */
.input .input__wrapper .input__prefix {
    display: inline-flex;
    flex-shrink: 0;
    flex-wrap: nowrap;
    height: 100%;
    line-height: var(--input-inner-height);
    color: var(--sv-text-color-2);
    text-align: center;
    white-space: nowrap;
    pointer-events: none;
    transition: all 0.2s ease;
    margin-right: 8px;
}

.input .input__wrapper .input__prefix .input__prefix-inner {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    pointer-events: all;
}

.input .input__wrapper .input__prefix .input__prefix-inner > :first-child {
    margin-left: 0;
}

.input .input__wrapper .input__prefix .input__prefix-inner > :last-child {
    margin-right: 0;
}

/* 输入框内部样式 */
.input .input__wrapper .input__inner {
    box-sizing: border-box;
    flex-grow: 1;
    width: 100%;
    height: var(--input-inner-height);
    padding: 0;
    font-size: inherit;
    line-height: var(--input-inner-height);
    color: var(--sv-text-color-1);
    outline: none;
    background: none;
    border: none;
}

/* 获得焦点时的样式 */
.input .input__wrapper .input__inner:focus {
    outline: none;
}

/* 占位符样式 */
.input .input__wrapper .input__inner::placeholder {
    color: var(--color-light-text-muted, #64748b);
    font-weight: 500;
}

/* 后缀样式 */
.input .input__wrapper .input__suffix {
    display: inline-flex;
    flex-shrink: 0;
    flex-wrap: nowrap;
    height: 100%;
    line-height: var(--input-inner-height);
    color: var(--sv-text-color-2);
    text-align: center;
    white-space: nowrap;
    pointer-events: none;
    transition: all 0.2s ease;
    margin-left: 8px;
}

.input .input__wrapper .input__suffix .input__suffix-inner {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    pointer-events: all;
}
</style>
