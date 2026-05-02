<script setup lang="ts">
const visible = defineModel({
    type: Boolean,
    default: false,
})

const props = defineProps({
    title: {
        type: String,
        default: '',
    },
    width: {
        type: String,
        default: '1024px',
    },
    height: {
        type: String,
        default: '800px',
    },
    onClick: {
        type: Function,
        default: () => {},
    },
    showHeader: {
        type: Boolean,
        default: true,
    },
    clickOnOverlay: {
        type: Boolean,
        default: false,
    },
})

const emit = defineEmits(['close'])

const closeModal = () => {
    if (!props.clickOnOverlay) {
        return
    }
    if (props.onClick) {
        props.onClick()
    }
    emit('close')

    visible.value = false
}
</script>
<template>
    <Teleport v-if="visible" to="body">
        <!-- 蒙版 -->
        <div class="fixed inset-0 z-1000 h-full bg-[var(--sv-modal-mask-color)]"></div>

        <Transition name="modal">
            <div class="fixed inset-0 z-1000 overflow-auto outline-0">
                <!-- 弹窗内容 -->
                <div
                    class="absolute left-1/2 top-1/2 -translate-1/2 bg-popover rounded-lg pointer-events-auto z-1000 shadow-md"
                    :style="{ width, height }"
                    v-click-outside="closeModal">
                    <slot name="header">
                        <header v-if="showHeader" class="flex items-center justify-between px-4 py-2">
                            <span class="text-lg font-bold text-fg">{{ title }}</span>
                            <button
                                class="absolute top-4 right-4 text-fg hover:bg-red hover:scale-[1.02] transition-all duration-200 p-1 rounded text-fg"
                                @click="closeModal">
                                <span class="i-md-close"></span>
                            </button>
                        </header>
                    </slot>
                    <slot name="default"></slot>
                </div>
            </div>
        </Transition>
    </Teleport>
</template>

<style lang="css">
.modal-enter-active {
    animation: bounce-in 0.5s cubic-bezier(0.68, -0.55, 0.265, 1.55);
}

.modal-leave-active {
    transition: all 0.3s ease;
}

.modal-leave-to {
    opacity: 0;
    transform: scale(0.9);
}

@keyframes bounce-in {
    0% {
        opacity: 0;
        transform: translate(-50%, -50%) scale(0.3);
    }
    50% {
        transform: translate(-50%, -50%) scale(1.05);
    }
    70% {
        transform: translate(-50%, -50%) scale(0.9);
    }
    100% {
        opacity: 1;
        transform: translate(-50%, -50%) scale(1);
    }
}
</style>
