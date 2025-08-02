//! # Hello World Template - xxHash Project Structure
//! 
//! This is a template showing the same project structure as the xxHash migration
//! but with simple hello-world functionality for testing purposes.

pub mod error;
pub mod constants;

pub use error::{XXHashError, XXHashResult};

/// Version information
pub const VERSION: &str = "0.8.1";

/// Simple hello world function
pub fn hello_world() -> String {
    "Hello, World!".to_string()
}

/// Simple function with error handling
pub fn greet_user(name: Option<&str>) -> XXHashResult<String> {
    match name {
        Some(n) if !n.is_empty() => Ok(format!("Hello, {}!", n)),
        Some(_) => Err(XXHashError::InvalidInputLength(0)),
        None => Ok("Hello, Anonymous!".to_string()),
    }
}

/// Function that demonstrates Result<T,E> usage
pub fn safe_divide(a: f64, b: f64) -> XXHashResult<f64> {
    if b == 0.0 {
        Err(XXHashError::OperationFailed("Division by zero".to_string()))
    } else {
        Ok(a / b)
    }
}

/// Function that demonstrates Option<T> usage
pub fn find_item(items: &[&str], target: &str) -> Option<usize> {
    items.iter().position(|&item| item == target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        assert_eq!(hello_world(), "Hello, World!");
    }

    #[test]
    fn test_greet_user() {
        assert_eq!(greet_user(Some("Alice")).unwrap(), "Hello, Alice!");
        assert_eq!(greet_user(None).unwrap(), "Hello, Anonymous!");
        assert!(greet_user(Some("")).is_err());
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10.0, 2.0).unwrap(), 5.0);
        assert!(safe_divide(10.0, 0.0).is_err());
    }

    #[test]
    fn test_find_item() {
        let items = ["apple", "banana", "cherry"];
        assert_eq!(find_item(&items, "banana"), Some(1));
        assert_eq!(find_item(&items, "grape"), None);
    }
}