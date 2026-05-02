<script setup lang="ts">
import { ref, defineAsyncComponent, markRaw, computed } from 'vue'

const props = defineProps({
    modelValue: {
        type: Boolean,
        default: false,
    },
})

const emit = defineEmits(['update:modelValue'])

const show = computed({
    get() {
        return props.modelValue
    },
    set(val) {
        emit('update:modelValue', val)
    },
})

const activeName = ref('account')

const pagePanel: Record<string, { name: string; title: string; component: any }> = {
    account: {
        name: 'account',
        title: 'Basic Settings',
        component: markRaw(defineAsyncComponent(() => import('./Account.vue'))),
    },
    theme: {
        name: 'theme',
        title: 'Appearance',
        component: markRaw(defineAsyncComponent(() => import('./Theme.vue'))),
    },
    note: {
        name: 'note',
        title: 'Notes',
        component: markRaw(defineAsyncComponent(() => import('./Note.vue'))),
    },
    // advanced: {
    //     name: 'advanced',
    //     title: 'Advanced',
    //     component: markRaw(defineAsyncComponent(() => import('./Advanced.vue'))),
    // },
    shortcut: {
        name: 'shortcut',
        title: 'Shortcuts',
        component: markRaw(defineAsyncComponent(() => import('./Shortcut.vue'))),
    },
    about: {
        name: 'about',
        title: 'About',
        component: markRaw(defineAsyncComponent(() => import('./About.vue'))),
    },
}

function closeModal() {
    show.value = false
}
</script>
<template>
    <Modal v-model="show" title="Settings" width="1024px" height="800px" :showHeader="false">
        <div class="flex h-full">
            <!-- Sidebar -->
            <div class="w-64 h-full border-r-1px border-r-solid border-r-border px-4 py-8 flex flex-col gap-4">
                <div class="text-[var(--sv-text-color-3)]">Account Settings</div>
                <div class="btn" :class="{ isActive: activeName === 'account' }" @click="activeName = 'account'">
                    Basic Settings
                </div>
                <div class="text-[var(--sv-text-color-3)] mt-4">Preferences</div>
                <div class="btn" :class="{ isActive: activeName === 'theme' }" @click="activeName = 'theme'">
                    Appearance
                </div>
                <div class="btn" :class="{ isActive: activeName === 'note' }" @click="activeName = 'note'">Notes</div>
                <!-- <div class="btn" :class="{ isActive: activeName === 'advanced' }" @click="activeName = 'advanced'">
                    Advanced
                </div> -->
                <div class="btn" :class="{ isActive: activeName === 'shortcut' }" @click="activeName = 'shortcut'">
                    Shortcuts
                </div>
                <div class="btn" :class="{ isActive: activeName === 'about' }" @click="activeName = 'about'">About</div>
            </div>
            <!-- Content Area -->
            <div class="flex-1 relative overflow-hidden">
                <Transition name="fade" mode="out-in">
                    <component :key="activeName" :is="pagePanel[activeName].component" />
                </Transition>
            </div>
            <!-- Close Button -->
            <button
                class="absolute top-4 right-4 hover:scale-[1.02] transition-all duration-200 hover:bg-card hover:text-error !border-none"
                @click="closeModal">
                <span class="i-md-close"></span>
            </button>
        </div>
    </Modal>
</template>

<style>
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

/* Dynamic component switch transition */
.fade-enter-active,
.fade-leave-active {
    transition: opacity 0.15s ease-out;
    will-change: opacity;
}

.fade-enter-from,
.fade-leave-to {
    opacity: 0;
}

/* Styles */
.btn {
    color: var(--sv-text-color-2);
    padding: 8px 16px;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.3s ease;
}

.btn.isActive {
    background-color: var(--sv-btn-secondary-color);
}

.btn:active {
    background-color: var(--sv-btn-secondary-color);
}
</style>
