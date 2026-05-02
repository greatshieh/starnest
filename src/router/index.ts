/**
 * 路由配置模块
 * 定义应用的路由结构和全局守卫
 * @module router
 */

import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'
import { authGuard, windowSizeGuard } from './guards'

/**
 * 路由配置列表
 */
const routes: RouteRecordRaw[] = [
    {
        path: '/login',
        name: 'Login',
        component: () => import('@/pages/LoginPage/index.vue'),
        meta: { requiresAuth: false },
    },
    {
        path: '/',
        name: 'Layout',
        component: () => import('@/Layout/index.vue'),
        redirect: '/dashboard',
        children: [
            {
                path: '/dashboard',
                name: 'Dashboard',
                component: () => import('@/pages/DashboardPage/index.vue'),
                meta: { requiresAuth: true, idx: 1, icon: 'dashboard' },
            },
            {
                path: '/repo',
                name: 'Repo',
                component: () => import('@/pages/ReposPage/index.vue'),
                meta: { requiresAuth: true, idx: 2, icon: 'star-outline' },
            },
            {
                path: '/repo/:owner/:name',
                name: 'RepoDetail',
                component: () => import('@/pages/RepoDetailPage/index.vue'),
                meta: { requiresAuth: true, idx: 3, hidden: true },
            },
            {
                path: '/tag',
                name: 'Tag',
                component: () => import('@/pages/TagPage/index.vue'),
                meta: { requiresAuth: true, idx: 4, icon: 'tag' },
            },
            {
                path: '/collection',
                name: 'Collection',
                component: () => import('@/pages/CollectionPage/index.vue'),
                meta: { requiresAuth: true, idx: 5, icon: 'collection' },
            },
        ],
    },
]

/**
 * 创建路由实例
 */
const router = createRouter({
    history: createWebHistory(),
    routes,
})

/**
 * 全局前置守卫 - 认证检查
 */
router.beforeEach(async (to, _from, next) => {
    await authGuard(to, next)
})

/**
 * 全局后置守卫 - 窗口尺寸调整
 */
router.afterEach(async to => {
    await windowSizeGuard(to, () => {})
})

export default router
