import { ref, onMounted, onUnmounted } from 'vue'

export function useModal(initialValue = false) {
    const isOpen = ref(initialValue)

    function open() {
        isOpen.value = true
    }

    function close() {
        isOpen.value = false
    }

    function toggle() {
        isOpen.value = !isOpen.value
    }

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === 'Escape' && isOpen.value) {
            close()
        }
    }

    onMounted(() => {
        window.addEventListener('keydown', handleKeydown)
    })

    onUnmounted(() => {
        window.removeEventListener('keydown', handleKeydown)
    })

    return {
        isOpen,
        open,
        close,
        toggle,
    }
}