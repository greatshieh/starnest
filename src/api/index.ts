/**
 * API 层汇总
 * 统一导出所有 API 模块
 * @module api
 */

import * as auth from './auth'
import * as repo from './repo'
import * as note from './note'
import * as config from './config'
import * as category from './category'
import * as collection from './collection'

export const api = {
  auth,
  repo,
  note,
  config,
  category,
  collection,
}

export { auth, repo, note, config, category, collection }