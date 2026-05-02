/**
 * 计算元素的最大高度，确保不超出屏幕范围
 * @param element - 需要计算高度的 HTML 元素
 * @param options - 可选配置
 * @returns 元素应设置的最大高度（像素）
 */
export function calculateMaxHeight(
    element: HTMLElement,
    options: {
        /** titlebar 高度（默认 32px） */
        titlebarHeight?: number
        /** statusbar 高度（默认 24px） */
        statusbarHeight?: number
        /** 额外的顶部边距（默认 0） */
        topMargin?: number
        /** 额外的底部边距（默认 0） */
        bottomMargin?: number
        /** 是否考虑元素当前的 marginTop（默认 true） */
        includeElementMargin?: boolean
    } = {}
): number {
    // 默认配置
    const {
        titlebarHeight = 32,
        statusbarHeight = 24,
        topMargin = 0,
        bottomMargin = 0,
        includeElementMargin = true,
    } = options

    // 屏幕总高度
    const screenHeight = window.innerHeight

    // 系统栏占用高度（titlebar + statusbar）
    const systemBarHeight = titlebarHeight + statusbarHeight

    // 计算元素顶部距离视口顶部的距离
    let elementTopOffset = 0
    if (element) {
        const rect = element.getBoundingClientRect()
        elementTopOffset = rect.top
        
        // 如果需要考虑元素的 marginTop
        if (includeElementMargin) {
            const computedStyle = window.getComputedStyle(element)
            const marginTop = parseFloat(computedStyle.marginTop) || 0
            elementTopOffset -= marginTop
        }
    }

    // 计算可用高度 = 屏幕高度 - 系统栏高度 - 元素顶部偏移 - 额外边距
    const availableHeight = screenHeight - systemBarHeight - elementTopOffset - topMargin - bottomMargin

    // 返回最大高度（最小为 0）
    return Math.max(0, availableHeight)
}

/**
 * 直接设置元素的最大高度
 * @param element - 需要设置高度的 HTML 元素
 * @param options - 可选配置
 */
export function setMaxHeight(
    element: HTMLElement,
    options: Parameters<typeof calculateMaxHeight>[1] = {}
): void {
    const maxHeight = calculateMaxHeight(element, options)
    element.style.maxHeight = `${maxHeight}px`
    element.style.height = `${maxHeight}px`
}

/**
 * 创建一个响应式的高度设置函数，当窗口大小变化时自动更新
 * @param element - 需要设置高度的 HTML 元素
 * @param options - 可选配置
 * @returns 清理函数，用于移除事件监听
 */
export function createResponsiveMaxHeight(
    element: HTMLElement,
    options: Parameters<typeof calculateMaxHeight>[1] = {}
): () => void {
    const updateHeight = () => {
        setMaxHeight(element, options)
    }

    // 立即执行一次
    updateHeight()

    // 添加窗口 resize 事件监听
    window.addEventListener('resize', updateHeight)

    // 返回清理函数
    return () => {
        window.removeEventListener('resize', updateHeight)
    }
}
