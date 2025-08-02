use xxhash_migration::*;

fn main() {
    println!("=== Simple XXH3 Test ===");
    
    // Test empty string
    let empty_hash = xxh3_64bits(b"");
    println!("XXH3_64('') = 0x{:016x}", empty_hash);
    println!("Expected:     0x2d06800538d394c2");
    println!("Match: {}", empty_hash == 0x2d06800538d394c2);
    
    // Test "hello world"
    let hello_hash = xxh3_64bits(b"hello world");
    println!("XXH3_64('hello world') = 0x{:016x}", hello_hash);
    println!("Expected:                 0xd447b1ea40e6988b");
    println!("Match: {}", hello_hash == 0xd447b1ea40e6988b);
    
    // Test "a"
    let a_hash = xxh3_64bits(b"a");
    println!("XXH3_64('a') = 0x{:016x}", a_hash);
    println!("Expected:     0xe6c632b61e964e1f");
    println!("Match: {}", a_hash == 0xe6c632b61e964e1f);
    
    // Test "ab"
    let ab_hash = xxh3_64bits(b"ab");
    println!("XXH3_64('ab') = 0x{:016x}", ab_hash);
    println!("Expected:      0xa873719c24d5735c");
    println!("Match: {}", ab_hash == 0xa873719c24d5735c);
    
    // Test "abc"
    let abc_hash = xxh3_64bits(b"abc");
    println!("XXH3_64('abc') = 0x{:016x}", abc_hash);
    println!("Expected:       0x78af5f94892f3950");
    println!("Match: {}", abc_hash == 0x78af5f94892f3950);
}