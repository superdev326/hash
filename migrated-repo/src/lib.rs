//! # xxHash - Extremely Fast Hash Algorithm (Rust Migration)
//! 
//! This is a pure Rust implementation of the xxHash family of hash algorithms,
//! providing the same functionality as the original C implementation.
//! 
//! ## Algorithms
//! 
//! - **XXH32**: 32-bit hash algorithm
//! - **XXH64**: 64-bit hash algorithm  
//! - **XXH3_64**: Modern 64-bit hash algorithm
//! - **XXH3_128**: Modern 128-bit hash algorithm
//! 
//! ## Error Handling
//! 
//! All functions use Result<T, E> for error handling and Option<T> for nullable values.
//! No panics occur in normal control flow.

pub mod xxh32;
pub mod xxh64; 
pub mod xxh3;
pub mod error;
pub mod constants;

pub use error::{XXHashError, XXHashResult};
pub use xxh32::{XXH32State, xxh32, xxh32_with_seed};
pub use xxh64::{XXH64State, xxh64, xxh64_with_seed};
pub use xxh3::{XXH3State, xxh3_64bits, xxh3_64bits_with_seed, xxh3_64bits_with_secret, 
               xxh3_128bits, xxh3_128bits_with_seed, xxh3_128bits_with_secret,
               XXH128Hash, generate_secret_from_seed};

/// Version information
pub const VERSION: &str = "0.8.1";

/// Check if input data is valid
#[inline]
pub fn validate_input(_data: &[u8]) -> XXHashResult<()> {
    // All input is valid in Rust due to memory safety
    Ok(())
}

/// Check if secret is valid for XXH3
#[inline] 
pub fn validate_secret(secret: &[u8]) -> XXHashResult<()> {
    if secret.len() < 136 {
        return Err(XXHashError::InvalidSecretSize(secret.len()));
    }
    Ok(())
}