export function hexToRgb(hex: string): { r: number; g: number; b: number } | null {
    const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex)
    return result
        ? {
              r: parseInt(result[1], 16),
              g: parseInt(result[2], 16),
              b: parseInt(result[3], 16),
          }
        : null
}

function hexToHSL(hex: string): { h: number; s: number; l: number } {
    // 1. hex → RGB（0~1）
    const r = parseInt(hex.slice(1, 3), 16) / 255
    const g = parseInt(hex.slice(3, 5), 16) / 255
    const b = parseInt(hex.slice(5, 7), 16) / 255

    // 2. 找最大值和最小值
    const max = Math.max(r, g, b)
    const min = Math.min(r, g, b)
    const l = (max + min) / 2

    // 3. 如果 max === min，则是灰色，H=0, S=0
    if (max === min) return { h: 0, s: 0, l: Math.round(l * 100) }

    // 4. 计算饱和度 S
    const d = max - min
    const s = l > 0.5 ? d / (2 - max - min) : d / (max + min)

    // 5. 计算色相 H
    let h = 0
    switch (max) {
        case r:
            h = ((g - b) / d + (g < b ? 6 : 0)) / 6
            break
        case g:
            h = ((b - r) / d + 2) / 6
            break
        case b:
            h = ((r - g) / d + 4) / 6
            break
    }

    return {
        h: Math.round(h * 360),
        s: Math.round(s * 100),
        l: Math.round(l * 100),
    }
}

function hslToHex(h: number, s: number, l: number): string {
    s /= 100
    l /= 100

    // 1. 计算色度 c 和中间值 x
    const c = (1 - Math.abs(2 * l - 1)) * s
    const x = c * (1 - Math.abs(((h / 60) % 2) - 1))

    // 2. 根据 H 所在区间确定 RGB1（r1, g1, b1）
    let r1 = 0,
        g1 = 0,
        b1 = 0
    if (h < 60) {
        r1 = c
        g1 = x
        b1 = 0
    } else if (h < 120) {
        r1 = x
        g1 = c
        b1 = 0
    } else if (h < 180) {
        r1 = 0
        g1 = c
        b1 = x
    } else if (h < 240) {
        r1 = 0
        g1 = x
        b1 = c
    } else if (h < 300) {
        r1 = x
        g1 = 0
        b1 = c
    } else {
        r1 = c
        g1 = 0
        b1 = x
    }

    // 3. 加上亮度偏移 m，得到最终 RGB
    const m = l - c / 2
    const r = Math.round((r1 + m) * 255)
    const g = Math.round((g1 + m) * 255)
    const b = Math.round((b1 + m) * 255)

    // 4. 转为 hex
    return '#' + [r, g, b].map(v => v.toString(16).padStart(2, '0')).join('')
}

export function colorStates(hex: string, theme = 'light') {
    const { h, s, l } = hexToHSL(hex)
    const delta = theme === 'light' ? { hover: 9, pressed: -10 } : { hover: 6, pressed: -6 }

    const { r, g, b } = hexToRgb(hex) || {}
    return {
        '--sv-primary': `${r} ${g} ${b}`,
        '--sv-primary-solid': hex,
        '--sv-primary-hover': hslToHex(h, s, l + delta.hover),
        '--sv-primary-pressed': hslToHex(h, s, l + delta.pressed),
        '--sv-primary-suppl': hslToHex(h, s, l + delta.hover), // = hover
    }
}
