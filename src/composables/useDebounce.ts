import { ref, watch, toRef, toValue, type MaybeRefOrGetter } from 'vue'

export function useDebounce<T>(value: MaybeRefOrGetter<T>, delay: MaybeRefOrGetter<number> = 300) {
    const debouncedValue = ref<T>(toValue(value))

    watch(toRef(value), (newValue) => {
        const timer = setTimeout(() => {
            debouncedValue.value = newValue
        }, toValue(delay))

        return () => clearTimeout(timer)
    })

    return debouncedValue
}

export function useDebounceFn<T extends (...args: Parameters<T>) => void>(fn: T, delay: MaybeRefOrGetter<number> = 300) {
    const timerRef = ref<ReturnType<typeof setTimeout> | null>(null)

    const debouncedFn = ((...args: Parameters<T>) => {
        if (timerRef.value) {
            clearTimeout(timerRef.value)
        }
        timerRef.value = setTimeout(() => {
            fn(...args)
            timerRef.value = null
        }, toValue(delay))
    }) as T

    return debouncedFn
}