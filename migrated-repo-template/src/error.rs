//! Error handling for xxHash operations

use std::fmt;

/// Result type for xxHash operations
pub type XXHashResult<T> = Result<T, XXHashError>;

/// Errors that can occur during xxHash operations
#[derive(Debug, Clone, PartialEq)]
pub enum XXHashError {
    /// Invalid input length
    InvalidInputLength(usize),
    /// Invalid secret size (must be >= 136 bytes)
    InvalidSecretSize(usize),
    /// Invalid state - corrupted or uninitialized
    InvalidState,
    /// Buffer too small for operation
    BufferTooSmall { required: usize, available: usize },
    /// Invalid seed value
    InvalidSeed,
    /// Operation failed
    OperationFailed(String),
}

impl fmt::Display for XXHashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XXHashError::InvalidInputLength(len) => {
                write!(f, "Invalid input length: {}", len)
            }
            XXHashError::InvalidSecretSize(size) => {
                write!(f, "Invalid secret size: {} (must be >= 136 bytes)", size)
            }
            XXHashError::InvalidState => {
                write!(f, "Invalid hash state - corrupted or uninitialized")
            }
            XXHashError::BufferTooSmall { required, available } => {
                write!(f, "Buffer too small: need {} bytes, have {} bytes", required, available)
            }
            XXHashError::InvalidSeed => {
                write!(f, "Invalid seed value")
            }
            XXHashError::OperationFailed(msg) => {
                write!(f, "Operation failed: {}", msg)
            }
        }
    }
}

impl std::error::Error for XXHashError {}

/// Convert XXHashError to a result indicating success (0) or error (1)
impl XXHashError {
    pub fn to_error_code(&self) -> i32 {
        match self {
            XXHashError::InvalidInputLength(_) => 1,
            XXHashError::InvalidSecretSize(_) => 2,
            XXHashError::InvalidState => 3,
            XXHashError::BufferTooSmall { .. } => 4,
            XXHashError::InvalidSeed => 5,
            XXHashError::OperationFailed(_) => 6,
        }
    }
}