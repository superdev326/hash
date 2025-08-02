use xxhash_migration::{hello_world, greet_user, safe_divide, find_item, HelloError};

#[test]
fn integration_test_hello_world() {
    let result = hello_world();
    assert_eq!(result, "Hello, World!");
}

#[test]
fn integration_test_greet_user_success() {
    let result = greet_user("Integration Test");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello, Integration Test!");
}

#[test]
fn integration_test_greet_user_failure() {
    let result = greet_user("   ");
    assert!(result.is_err());
    match result.unwrap_err() {
        HelloError::InvalidInput(_) => {}, // Expected
        _ => panic!("Wrong error type"),
    }
}

#[test]
fn integration_test_safe_divide() {
    let result = safe_divide(15.0, 3.0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 5.0);

    let result = safe_divide(1.0, 0.0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), HelloError::DivisionByZero);
}

#[test]
fn integration_test_find_item() {
    let data = vec!["hello", "world", "test"];
    
    let result = find_item(&data, &"world");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);

    let result = find_item(&data, &"missing");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), HelloError::NotFound);
}

#[test]
fn comprehensive_integration_test() {
    // Test that all functions work together properly
    let greeting = hello_world();
    assert!(!greeting.is_empty());

    let user_greeting = greet_user("Test User").expect("Should greet successfully");
    assert!(user_greeting.contains("Test User"));

    let division_result = safe_divide(100.0, 10.0).expect("Should divide successfully");
    assert_eq!(division_result, 10.0);

    let search_items = vec![1, 2, 3, 4, 5];
    let found_index = find_item(&search_items, &3).expect("Should find item");
    assert_eq!(found_index, 2);
}