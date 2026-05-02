<script setup lang="ts">
import { computed, CSSProperties } from 'vue'
import { RouteRecordNormalized, useRouter } from 'vue-router'

defineOptions({
    name: 'MenuNav',
})

const props = defineProps<{
    isCollapsed?: boolean
}>()

const router = useRouter()
const routes = router
    .getRoutes()
    .filter(route => {
        return route.meta.requiresAuth && !route.meta?.hidden
    })
    .sort((a, b) => (a.meta.idx as number) - (b.meta.idx as number))

const handleClick = (item: RouteRecordNormalized) => {
    router.push(item.path || '')
}

const isActive = (item: RouteRecordNormalized): boolean => {
    const currentPath = router.currentRoute.value.path

    if (item.path) {
        if (currentPath === item.path) {
            return true
        }
        if (currentPath.startsWith(item.path + '/')) {
            return true
        }
    }
    return false
}

const itemStyle = computed(() => {
    return (item: RouteRecordNormalized) => {
        const commonStyle: CSSProperties = {
            position: 'relative',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'flex-start',
            gap: '8px',
            padding: '2px 4px',
            fontSize: '16px',
            cursor: 'pointer',
            transition: 'all 0.2s ease',
            color: 'var(--sv-text-color-1)',
            height: '32px',
            width: '100%',
        }

        const active = isActive(item)

        if (props.isCollapsed) {
            commonStyle['width'] = '32px'
            commonStyle['borderRadius'] = '100%'
            commonStyle['justifyContent'] = 'center'
            if (active) {
                commonStyle['backgroundColor'] = 'var(--sv-primary-solid)'
            }
        } else {
            if (active) {
                commonStyle['backgroundColor'] = 'rgb(var(--sv-primary)/0.15)'
                commonStyle['color'] = 'var(--sv-primary-solid)'
            }
        }

        return commonStyle
    }
})
</script>

<template>
    <div class="flex-1 flex flex-col py-4 overflow-hidden items-center gap-4 w-full">
        <template v-for="item in routes" :key="item.name">
            <div :style="itemStyle(item)" @click="handleClick(item)">
                <span v-show="isActive(item) && !isCollapsed" class="absolute left-0 h-full w-[4px] bg-primary"></span>
                <span :class="[{ 'text-xl !ml-0': isCollapsed }, `i-md-${item.meta.icon}`, 'ml-2']"></span>
                <span :class="{ hidden: isCollapsed }">
                    {{ item.name }}
                </span>
            </div>
        </template>
    </div>
</template>
