/**
 * 笔记相关 API 类型定义
 * @module api/note
 */

/**
 * 笔记类型
 */
export type NoteType = 'note' | 'snippet'

/**
 * 笔记信息
 */
export interface Note {
    id: number
    date: string
    content: string
    type: NoteType
    snippet?: string
}

/**
 * 创建笔记请求参数（不含 id）
 */
export type CreateNoteRequest = Omit<Note, 'id'>

/**
 * 更新笔记请求参数
 */
export interface UpdateNoteRequest {
    id: number
    updates: Partial<Omit<Note, 'id'>>
}

/**
 * 保存笔记请求参数（后端调用）
 */
export interface SaveNoteRequest {
    id: number
    github_id: number
    owner: string
    repo_name: string
    note_name: string
    content: string
}

/**
 * 读取笔记请求参数
 */
export interface ReadNoteRequest {
    owner: string
    repo_name: string
    note_name: string
}

/**
 * 获取仓库笔记列表请求参数
 */
export interface GetNotesByRepoRequest {
    github_id: number
}

/**
 * 获取默认笔记名称请求参数
 */
export interface GetDefaultNoteNameRequest {
    owner: string
    repo_name: string
}
