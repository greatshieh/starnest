/**
 * 笔记相关 API 封装
 * 统一处理笔记相关的后端调用
 * @module api/note
 */

import { invoke } from '@tauri-apps/api/core'
import type { api } from '@/types'

/**
 * 创建笔记（前端本地操作）
 * @param request - 笔记数据（不含 id）
 * @returns 创建的笔记（包含自动生成的 id）
 */
export function createNote(request: api.note.CreateNoteRequest): api.note.Note {
  return {
    ...request,
    id: Date.now(),
  }
}

/**
 * 更新笔记（前端本地操作）
 * @param notes - 笔记列表
 * @param request - 更新请求参数
 * @returns 更新后的笔记列表
 */
export function updateNote(
  notes: api.note.Note[],
  request: api.note.UpdateNoteRequest
): api.note.Note[] {
  return notes.map(note =>
    note.id === request.id ? { ...note, ...request.updates } : note
  )
}

/**
 * 删除笔记（前端本地操作）
 * @param notes - 笔记列表
 * @param id - 要删除的笔记 id
 * @returns 删除后的笔记列表
 */
export function deleteNote(notes: api.note.Note[], id: number): api.note.Note[] {
  return notes.filter(note => note.id !== id)
}

/**
 * 根据 id 获取笔记（前端本地操作）
 * @param notes - 笔记列表
 * @param id - 笔记 id
 * @returns 匹配的笔记或 undefined
 */
export function getNoteById(notes: api.note.Note[], id: number): api.note.Note | undefined {
  return notes.find(note => note.id === id)
}

/**
 * 保存笔记（后端调用）
 * @param request - 保存请求参数
 * @returns 保存的笔记信息
 */
export async function saveNote(request: api.note.SaveNoteRequest): Promise<api.note.Note> {
  const result = await invoke('cmd_save_note', { request })
  return result as api.note.Note
}

/**
 * 读取笔记内容（后端调用）
 * @param request - 读取请求参数
 * @returns 笔记内容
 */
export async function readNote(request: api.note.ReadNoteRequest): Promise<string> {
  const result = await invoke('cmd_read_note', { request })
  return result as string
}

/**
 * 获取仓库笔记列表（后端调用）
 * @param request - 请求参数
 * @returns 笔记列表
 */
export async function getNotesByRepo(request: api.note.GetNotesByRepoRequest): Promise<api.note.Note[]> {
  const result = await invoke('cmd_get_notes_by_repo', { request })
  return result as api.note.Note[]
}

/**
 * 获取默认笔记名称（后端调用）
 * @param request - 请求参数
 * @returns 默认笔记名称
 */
export async function getDefaultNoteName(request: api.note.GetDefaultNoteNameRequest): Promise<string> {
  const result = await invoke('cmd_get_default_note_name', { request })
  return result as string
}