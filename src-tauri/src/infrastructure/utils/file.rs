//! 文件操作工具
//!
//! 提供文件和目录操作相关的工具函数。

use std::fs::{self};
use std::path::Path;

/// 复制目录
///
/// # 参数
/// - `src`: 源目录路径
/// - `dst`: 目标目录路径
///
/// # 返回值
/// 成功返回 Ok(())，失败返回 Err(String)
pub fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), String> {
    if src.is_dir() {
        fs::create_dir_all(dst).map_err(|e| format!("Failed to create directory: {:?}", e))?;

        for entry in fs::read_dir(src).map_err(|e| format!("Failed to read directory: {:?}", e))? {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {:?}", e))?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if entry
                .file_type()
                .map_err(|e| format!("Failed to get file type: {:?}", e))?
                .is_dir()
            {
                copy_dir_all(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)
                    .map_err(|e| format!("Failed to copy file: {:?}", e))?;
            }
        }
    }
    Ok(())
}
