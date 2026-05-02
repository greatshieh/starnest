# StarNest - GitHub Star Manager

一个现代化的 GitHub Star 仓库管理器，帮助您高效管理和组织收藏的 GitHub 仓库。

## ✨ 功能特性

### 核心功能
- [x] **仓库管理**：浏览、搜索和管理您收藏的 GitHub 仓库
- [x] **标签分类**：为仓库创建自定义标签，支持颜色标记
- [x] **合集功能**：将相关仓库组合成合集统一管理
- [x] **笔记功能**：为每个仓库添加个人笔记和备注
- [x] **实时同步**：自动同步 GitHub 收藏，保持数据最新
- [x] **搜索功能**：支持仓库搜索，带防抖和键盘导航

### 界面体验
- [x] **深色模式**：支持明暗主题切换
- [x] **国际化**：支持中文界面

### 待开发功能
- [ ] 仓库统计分析
- [ ] 批量操作
- [ ] 数据导出
- [ ] 快捷键支持

## 🛠️ 技术栈

- **前端框架**: Vue 3 + TypeScript + Composition API
- **UI 框架**: UnoCSS
- **状态管理**: Pinia
- **路由**: Vue Router
- **国际化**: vue-i18n
- **桌面框架**: Tauri 2
- **后端**: Rust

## 📁 项目结构

```
starhub/
├── src/                      # 前端源码
│   ├── api/                  # API 接口封装
│   ├── components/           # 通用组件
│   ├── composables/          # Vue Composables
│   ├── i18n/                 # 国际化配置
│   ├── Layout/               # 布局组件
│   ├── pages/                # 页面组件
│   ├── router/               # 路由配置
│   ├── stores/               # Pinia 状态管理
│   ├── style/                # 全局样式
│   ├── types/                # TypeScript 类型定义
│   └── utils/                # 工具函数
├── src-tauri/                # Tauri 后端
│   ├── src/
│   │   ├── db/               # 数据库操作
│   │   ├── handlers/         # API 处理器
│   │   ├── infrastructure/   # 基础设施层
│   │   ├── models/           # 数据模型
│   │   ├── repos/            # 数据访问层
│   │   └── services/         # 业务服务层
│   └── icons/                # 应用图标
├── public/                   # 静态资源
└── index.html                # 入口 HTML
```

## 🚀 快速开始

### 前置要求

- Node.js >= 20
- Rust >= 1.75
- pnpm >= 8

### 安装依赖

```bash
cd starhub
pnpm install
```

### 开发模式

```bash
pnpm tauri dev
```

### 构建生产版本

```bash
pnpm tauri build
```

## 📝 使用说明

1. **登录**: 使用 GitHub Token 进行登录
2. **同步**: 点击同步按钮获取最新的收藏仓库
3. **标签管理**: 在标签页面创建和管理自定义标签
4. **合集管理**: 在合集页面创建合集并添加仓库
5. **笔记**: 在仓库详情页添加个人笔记

## 📄 许可证

MIT License

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！