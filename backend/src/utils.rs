use bcrypt::{hash, verify, DEFAULT_COST};
use crate::error::AppError;

/// 对明文密码进行哈希
pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST)
        .map_err(|_| AppError::InternalError)
}

/// 验证明文密码和哈希值是否匹配
pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password, hash)
        .map_err(|_| AppError::InternalError)
}