//! 哈希计算工具
//!
//! 提供哈希计算相关的工具函数。

use sha2::{Digest, Sha256};

/// 计算字符串的 SHA256 哈希值
///
/// # 示例
///
/// ```
/// let hash = compute_sha256_hash("hello");
/// assert_eq!(hash.len(), 64); // SHA256 输出 64 位十六进制字符串
/// ```
///
/// # 参数
/// - `input`: 输入字符串
///
/// # 返回值
/// 64 位十六进制哈希字符串
pub fn compute_sha256_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}
