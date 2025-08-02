use xxhash_migration::*;

/// Test data for comparison - same as C reference
const TEST_STRINGS: &[&str] = &[
    "",
    "a",
    "ab",
    "abc",
    "abcd",
    "abcde",
    "abcdef",
    "abcdefg",
    "abcdefgh",
    "abcdefghi",
    "abcdefghij",
    "abcdefghijk",
    "abcdefghijkl",
    "abcdefghijklm",
    "abcdefghijklmn",
    "abcdefghijklmno",
    "abcdefghijklmnop",
    "hello world",
    "xxHash is a very fast hashing algorithm",
    "This is a longer test string to verify the implementation works correctly",
];

fn print_hash_32(input: &str, seed: u32) {
    let hash = xxh32_with_seed(input.as_bytes(), seed);
    println!("XXH32('{}', 0x{:08x}) = 0x{:08x}", input, seed, hash);
}

fn print_hash_64(input: &str, seed: u64) {
    let hash = xxh64_with_seed(input.as_bytes(), seed);
    println!("XXH64('{}', 0x{:016x}) = 0x{:016x}", input, seed, hash);
}

fn print_hash_3_64(input: &str) {
    let hash = xxh3_64bits(input.as_bytes());
    println!("XXH3_64('{}') = 0x{:016x}", input, hash);
}

fn print_hash_3_64_seed(input: &str, seed: u64) {
    let hash = xxh3_64bits_with_seed(input.as_bytes(), seed);
    println!("XXH3_64('{}', 0x{:016x}) = 0x{:016x}", input, seed, hash);
}

fn print_hash_3_128(input: &str) {
    let hash = xxh3_128bits(input.as_bytes());
    println!("XXH3_128('{}') = 0x{:016x}{:016x}", input, hash.high, hash.low);
}

fn print_hash_3_128_seed(input: &str, seed: u64) {
    let hash = xxh3_128bits_with_seed(input.as_bytes(), seed);
    println!("XXH3_128('{}', 0x{:016x}) = 0x{:016x}{:016x}", input, seed, hash.high, hash.low);
}

fn print_hash_3_64_secret(input: &str) {
    // Create a custom secret based on the default secret  
    let custom_secret = generate_secret_from_seed(0);
    match xxh3_64bits_with_secret(input.as_bytes(), &custom_secret) {
        Ok(hash) => println!("XXH3_64_secret('{}') = 0x{:016x}", input, hash),
        Err(e) => eprintln!("Error computing XXH3_64_secret: {}", e),
    }
}

fn print_hash_3_128_secret(input: &str) {
    // Create a custom secret based on the default secret
    let custom_secret = generate_secret_from_seed(0);
    match xxh3_128bits_with_secret(input.as_bytes(), &custom_secret) {
        Ok(hash) => println!("XXH3_128_secret('{}') = 0x{:016x}{:016x}", input, hash.high, hash.low),
        Err(e) => eprintln!("Error computing XXH3_128_secret: {}", e),
    }
}

fn main() {
    println!("=== xxHash Rust Migration Implementation Test ===\n");
    
    // Test seeds - same as C reference
    let seed32 = 0x12345678u32;
    let seed64 = 0x123456789abcdef0u64;
    
    println!("--- XXH32 Tests ---");
    for test_string in TEST_STRINGS {
        print_hash_32(test_string, 0);
        print_hash_32(test_string, seed32);
    }
    
    println!("\n--- XXH64 Tests ---");
    for test_string in TEST_STRINGS {
        print_hash_64(test_string, 0);
        print_hash_64(test_string, seed64);
    }
    
    println!("\n--- XXH3_64 Tests ---");
    for test_string in TEST_STRINGS {
        print_hash_3_64(test_string);
        print_hash_3_64_seed(test_string, seed64);
    }
    
    println!("\n--- XXH3_128 Tests ---");
    for test_string in TEST_STRINGS {
        print_hash_3_128(test_string);
        print_hash_3_128_seed(test_string, seed64);
    }
    
    println!("\n--- XXH3 Secret Tests ---");
    for test_string in TEST_STRINGS {
        print_hash_3_64_secret(test_string);
        print_hash_3_128_secret(test_string);
    }
    
    println!("\n=== Test Complete ===");
}
