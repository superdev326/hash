use xxhash_migration::*;

fn main() {
    println!("=== Hello World Template ===");
    
    // Basic hello world
    println!("{}", hello_world());
    
    // Error handling demonstration
    match greet_user(Some("Rust Developer")) {
        Ok(greeting) => println!("{}", greeting),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    match greet_user(None) {
        Ok(greeting) => println!("{}", greeting),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    // Result<T,E> demonstration
    println!("Division results:");
    match safe_divide(20.0, 4.0) {
        Ok(result) => println!("20.0 / 4.0 = {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("10.0 / 0.0 = {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    // Option<T> demonstration
    let fruits = ["apple", "banana", "cherry", "date"];
    println!("Searching for fruits:");
    
    match find_item(&fruits, "banana") {
        Some(index) => println!("Found 'banana' at index {}", index),
        None => println!("'banana' not found"),
    }
    
    match find_item(&fruits, "grape") {
        Some(index) => println!("Found 'grape' at index {}", index),
        None => println!("'grape' not found"),
    }
    
    println!("=== Template Complete ===");
}
