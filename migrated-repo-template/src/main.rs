use xxhash_migration::{hello_world, greet_user, safe_divide, find_item, HelloResult};

fn main() -> HelloResult<()> {
    println!("=== Hello World Template Demonstration ===\n");

    // Basic hello world
    println!("1. Basic greeting:");
    println!("   {}", hello_world());
    println!();

    // User greeting with error handling
    println!("2. User greetings:");
    match greet_user("Alice") {
        Ok(greeting) => println!("   Success: {}", greeting),
        Err(e) => println!("   Error: {}", e),
    }
    
    match greet_user("") {
        Ok(greeting) => println!("   Success: {}", greeting),
        Err(e) => println!("   Error: {}", e),
    }
    println!();

    // Safe division
    println!("3. Safe division:");
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("   10.0 / 2.0 = {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    
    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("   10.0 / 0.0 = {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    println!();

    // Finding items
    println!("4. Finding items:");
    let numbers = vec![1, 2, 3, 4, 5];
    
    match find_item(&numbers, &3) {
        Ok(index) => println!("   Found 3 at index: {}", index),
        Err(e) => println!("   Error: {}", e),
    }
    
    match find_item(&numbers, &10) {
        Ok(index) => println!("   Found 10 at index: {}", index),
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n=== All tests completed successfully ===");
    
    Ok(())
}
