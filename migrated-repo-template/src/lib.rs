//! # Hello World Library
//!
//! A simple demonstration library with basic functionality and proper error handling.
//! This template shows how to structure a Rust project with Result<T, E> error handling.

use std::fmt;

/// Custom error type for the library
#[derive(Debug, Clone, PartialEq)]
pub enum HelloError {
    /// Invalid input provided
    InvalidInput(String),
    /// Division by zero error
    DivisionByZero,
    /// Item not found
    NotFound,
}

impl fmt::Display for HelloError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HelloError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            HelloError::DivisionByZero => write!(f, "Division by zero"),
            HelloError::NotFound => write!(f, "Item not found"),
        }
    }
}

impl std::error::Error for HelloError {}

/// Result type for this library
pub type HelloResult<T> = Result<T, HelloError>;

/// Returns a friendly greeting
pub fn hello_world() -> String {
    "Hello, World!".to_string()
}

/// Greets a specific user with proper validation
pub fn greet_user(name: &str) -> HelloResult<String> {
    if name.trim().is_empty() {
        return Err(HelloError::InvalidInput("Name cannot be empty".to_string()));
    }
    Ok(format!("Hello, {}!", name.trim()))
}

/// Safely divides two numbers with error handling
pub fn safe_divide(a: f64, b: f64) -> HelloResult<f64> {
    if b == 0.0 {
        Err(HelloError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

/// Finds an item in a list and returns its index
pub fn find_item<T: PartialEq>(items: &[T], target: &T) -> HelloResult<usize> {
    items.iter()
        .position(|item| item == target)
        .ok_or(HelloError::NotFound)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        assert_eq!(hello_world(), "Hello, World!");
    }

    #[test]
    fn test_greet_user_valid() {
        let result = greet_user("Alice");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, Alice!");
    }

    #[test]
    fn test_greet_user_empty() {
        let result = greet_user("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), HelloError::InvalidInput("Name cannot be empty".to_string()));
    }

    #[test]
    fn test_safe_divide_success() {
        let result = safe_divide(10.0, 2.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5.0);
    }

    #[test]
    fn test_safe_divide_by_zero() {
        let result = safe_divide(10.0, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), HelloError::DivisionByZero);
    }

    #[test]
    fn test_find_item_found() {
        let items = vec![1, 2, 3, 4, 5];
        let result = find_item(&items, &3);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn test_find_item_not_found() {
        let items = vec![1, 2, 3, 4, 5];
        let result = find_item(&items, &6);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), HelloError::NotFound);
    }
}