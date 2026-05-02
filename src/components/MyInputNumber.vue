<script setup lang="ts">
// 导入Vue响应式API
import { ref } from 'vue'

// 定义组件名称
defineOptions({
    name: 'MyInputNumber',
})

// 定义组件属性
defineProps({
    // 输入框占位符文本
    placeholder: {
        type: String,
        default: '请输入',
    },
})

// 定义双向绑定的输入值（数字类型）
const inputValue = defineModel({
    type: Number,
    default: 0,
})

// 跟踪输入框是否获得焦点
const isFocus = ref(false)
</script>

<template>
    <!-- 数字输入框外层容器 -->
    <div class="input">
        <!-- 输入框包装器，包含焦点状态样式 -->
        <div class="input__wrapper" :class="{ 'is-focus': isFocus }" tabindex="-1">
            <!-- 数字输入框主体 -->
            <input
                class="input__inner"
                type="number"
                autocomplete="off"
                tabindex="0"
                :min="0"
                :placeholder="placeholder"
                :value="inputValue"
                @input="inputValue = parseInt((<HTMLInputElement>$event.target).value)"
                @focus="isFocus = true"
                @blur="isFocus = false" />
        </div>
    </div>
</template>

<style lang="css" scoped>
/* 数字输入框样式 */
.input {
    --input-height: 40px;
    --input-inner-height: calc(var(--input-height, 40px) - 2px);

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
    background-color: #ffffff;
    background-image: none;
    border-radius: 0.5rem;
    border: 1px solid #e2e8f0;
    transform: translateZ(0);
    transition: all 0.2s ease;
}

/* 鼠标悬停效果 */
.input .input__wrapper:hover {
    border-color: #cfc2d7;
    background-color: #faf8ff;
}

/* 获得焦点时的样式 */
.input .input__wrapper.is-focus {
    border-color: #9333ea;
    box-shadow: 0 0 0 3px rgba(147, 51, 234, 0.1);
}

/* 禁用状态 */
.input .input__wrapper:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    background-color: #f2f3ff;
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
    color: #131b2e;
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
    color: #7e7386;
    font-weight: 500;
}

/* 移除Webkit浏览器中的数字输入框上下箭头按钮 */
.input .input__wrapper .input__inner::-webkit-outer-spin-button,
.input .input__wrapper .input__inner::-webkit-inner-spin-button {
    appearance: none !important;
}

/* 设置数字输入框为文本域外观，移除默认样式 */
.input .input__wrapper .input__inner[type='number'] {
    appearance: textfield;
}
</style>
