// Manual test script for XXH3_128 implementation
// This simulates the key functions to verify our implementation

use std::collections::HashMap;

// Constants from our implementation
const XXH_PRIME64_1: u64 = 0x9E3779B185EBCA87;
const XXH_PRIME64_2: u64 = 0xC2B2AE3D27D4EB4F;
const XXH_PRIME64_4: u64 = 0x85EBCA77C2B2AE63;
const XXH_PRIME32_2: u32 = 0x85EBCA77;
const PRIME_MX1: u64 = 0x165667919E3779F9;
const PRIME_MX2: u64 = 0x9FB21C651E98DF25;

// Default secret from C reference
const XXH3_DEFAULT_SECRET: [u8; 192] = [
    0xb8, 0xfe, 0x6c, 0x39, 0x23, 0xa4, 0x4b, 0xbe, 0x7c, 0x01, 0x81, 0x2c, 0xf7, 0x21, 0xad, 0x1c,
    0xde, 0xd4, 0x6d, 0xe9, 0x83, 0x90, 0x97, 0xdb, 0x72, 0x40, 0xa4, 0xa4, 0xb7, 0xb3, 0x67, 0x1f,
    0xcb, 0x79, 0xe6, 0x4e, 0xcc, 0xc0, 0xe5, 0x78, 0x82, 0x5a, 0xd0, 0x7d, 0xcc, 0xff, 0x72, 0x21,
    0xb8, 0x08, 0x46, 0x74, 0xf7, 0x43, 0x24, 0x8e, 0xe0, 0x35, 0x90, 0xe6, 0x81, 0x3a, 0x26, 0x4c,
    0x3c, 0x28, 0x52, 0xbb, 0x91, 0xc3, 0x00, 0xcb, 0x88, 0xd0, 0x65, 0x8b, 0x1b, 0x53, 0x2e, 0xa3,
    0x71, 0x64, 0x48, 0x97, 0xa2, 0x0d, 0xf9, 0x4e, 0x38, 0x19, 0xef, 0x46, 0xa9, 0xde, 0xac, 0xd8,
    0xa8, 0xfa, 0x76, 0x3f, 0xe3, 0x9c, 0x34, 0x3f, 0xf9, 0xdc, 0xbb, 0xc7, 0xc7, 0x0b, 0x4f, 0x1d,
    0x8a, 0x51, 0xe0, 0x4b, 0xcd, 0xb4, 0x59, 0x31, 0xc8, 0x9f, 0x7e, 0xc9, 0xd9, 0x78, 0x73, 0x64,
    0xea, 0xc5, 0xac, 0x83, 0x34, 0xd3, 0xeb, 0xc3, 0xc5, 0x81, 0xa0, 0xff, 0xfa, 0x13, 0x63, 0xeb,
    0x17, 0x0d, 0xdd, 0x51, 0xb7, 0xf0, 0xda, 0x49, 0xd3, 0x16, 0x55, 0x26, 0x29, 0xd4, 0x68, 0x9e,
    0x2b, 0x16, 0xbe, 0x58, 0x7d, 0x47, 0xa1, 0xfc, 0x8f, 0xf8, 0xb8, 0xd1, 0x7a, 0xd0, 0x31, 0xce,
    0x45, 0xcb, 0x3a, 0x8f, 0x95, 0x16, 0x04, 0x28, 0xaf, 0xd7, 0xfb, 0xca, 0xbb, 0x4b, 0x40, 0x7e,
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct XXH128Hash {
    pub high: u64,
    pub low: u64,
}

impl XXH128Hash {
    pub fn new(high: u64, low: u64) -> Self {
        Self { high, low }
    }
}

// Helper functions
fn read_u32_le(data: &[u8]) -> u32 {
    u32::from_le_bytes([data[0], data[1], data[2], data[3]])
}

fn read_u64_le(data: &[u8]) -> u64 {
    u64::from_le_bytes([
        data[0], data[1], data[2], data[3],
        data[4], data[5], data[6], data[7],
    ])
}

fn rotl32(x: u32, r: u32) -> u32 {
    (x << r) | (x >> (32 - r))
}

fn xxh64_avalanche(mut hash: u64) -> u64 {
    hash ^= hash >> 33;
    hash = hash.wrapping_mul(XXH_PRIME64_2);
    hash ^= hash >> 29;
    hash = hash.wrapping_mul(XXH_PRIME64_3);
    hash ^= hash >> 32;
    hash
}

fn xxh3_avalanche(mut h: u64) -> u64 {
    h ^= h >> 37;
    h = h.wrapping_mul(PRIME_MX1);
    h ^= h >> 32;
    h
}

fn xxh_xorshift64(v64: u64, shift: i32) -> u64 {
    v64 ^ (v64 >> shift)
}

fn xxh_mult64to128(lhs: u64, rhs: u64) -> (u64, u64) {
    let lo_lo = (lhs & 0xFFFFFFFF).wrapping_mul(rhs & 0xFFFFFFFF);
    let hi_lo = (lhs >> 32).wrapping_mul(rhs & 0xFFFFFFFF);
    let lo_hi = (lhs & 0xFFFFFFFF).wrapping_mul(rhs >> 32);
    let hi_hi = (lhs >> 32).wrapping_mul(rhs >> 32);

    let cross = (lo_lo >> 32) + (hi_lo & 0xFFFFFFFF) + lo_hi;
    let upper = (hi_lo >> 32) + (cross >> 32) + hi_hi;
    let lower = (cross << 32) | (lo_lo & 0xFFFFFFFF);

    (lower, upper)
}

fn xxh_mult32to64(x: u64, y: u64) -> u64 {
    (x & 0xFFFFFFFF).wrapping_mul(y & 0xFFFFFFFF)
}

// Missing constant
const XXH_PRIME64_3: u64 = 0x165667B19E3779F9;

// XXH3_128 implementation
fn xxh3_len_1to3_128b(data: &[u8], secret: &[u8], seed: u64) -> XXH128Hash {
    let len = data.len();
    assert!(len >= 1 && len <= 3);
    
    let c1 = data[0];
    let c2 = data[len >> 1];
    let c3 = data[len - 1];
    
    let combinedl = ((c1 as u32) << 16) | ((c2 as u32) << 24) | ((c3 as u32) << 0) | ((len as u32) << 8);
    let combinedh = rotl32(combinedl.swap_bytes(), 13);
    
    let bitflipl = (read_u32_le(&secret[0..]) ^ read_u32_le(&secret[4..])) as u64 + seed;
    let bitfliph = (read_u32_le(&secret[8..]) ^ read_u32_le(&secret[12..])) as u64 - seed;
    
    let keyed_lo = combinedl as u64 ^ bitflipl;
    let keyed_hi = combinedh as u64 ^ bitfliph;
    
    XXH128Hash::new(
        xxh64_avalanche(keyed_hi),
        xxh64_avalanche(keyed_lo),
    )
}

fn xxh3_len_4to8_128b(data: &[u8], secret: &[u8], seed: u64) -> XXH128Hash {
    let len = data.len();
    assert!(len >= 4 && len <= 8);
    
    let mut seed = seed;
    seed ^= (seed.swap_bytes() as u64) << 32;
    
    let input_lo = read_u32_le(&data[0..]) as u64;
    let input_hi = read_u32_le(&data[len - 4..]) as u64;
    let input_64 = input_lo + (input_hi << 32);
    
    let bitflip = (read_u64_le(&secret[16..]) ^ read_u64_le(&secret[24..])) + seed;
    let keyed = input_64 ^ bitflip;
    
    let (low64, high64) = xxh_mult64to128(keyed, XXH_PRIME64_1 + (len as u64 << 2));
    
    let mut m128_low = low64;
    let mut m128_high = high64;
    
    m128_high += m128_low << 1;
    m128_low ^= m128_high >> 3;
    
    m128_low = xxh_xorshift64(m128_low, 35);
    m128_low = m128_low.wrapping_mul(PRIME_MX2);
    m128_low = xxh_xorshift64(m128_low, 28);
    m128_high = xxh3_avalanche(m128_high);
    
    XXH128Hash::new(m128_high, m128_low)
}

fn xxh3_len_9to16_128b(data: &[u8], secret: &[u8], seed: u64) -> XXH128Hash {
    let len = data.len();
    assert!(len >= 9 && len <= 16);
    
    let bitflipl = (read_u64_le(&secret[32..]) ^ read_u64_le(&secret[40..])) - seed;
    let bitfliph = (read_u64_le(&secret[48..]) ^ read_u64_le(&secret[56..])) + seed;
    
    let input_lo = read_u64_le(&data[0..]);
    let mut input_hi = read_u64_le(&data[len - 8..]);
    
    let (low64, high64) = xxh_mult64to128(input_lo ^ input_hi ^ bitflipl, XXH_PRIME64_1);
    
    let mut m128_low = low64 + ((len - 1) as u64 << 54);
    let mut m128_high = high64;
    
    input_hi ^= bitfliph;
    
    m128_high += (input_hi & 0xFFFFFFFF00000000) + xxh_mult32to64(input_hi as u64, XXH_PRIME32_2 as u64);
    
    m128_low ^= m128_high.swap_bytes();
    
    let (h128_low, h128_high) = xxh_mult64to128(m128_low, XXH_PRIME64_2);
    let final_high = h128_high + m128_high * XXH_PRIME64_2;
    
    XXH128Hash::new(
        xxh3_avalanche(final_high),
        xxh3_avalanche(h128_low),
    )
}

fn xxh3_len_0to16_128b(data: &[u8], secret: &[u8], seed: u64) -> XXH128Hash {
    let len = data.len();
    
    if len == 0 {
        let bitflipl = read_u64_le(&secret[64..]) ^ read_u64_le(&secret[72..]);
        let bitfliph = read_u64_le(&secret[80..]) ^ read_u64_le(&secret[88..]);
        let low64 = xxh64_avalanche(seed ^ bitflipl);
        let high64 = xxh64_avalanche(seed ^ bitfliph);
        XXH128Hash::new(high64, low64)
    } else if len > 8 {
        xxh3_len_9to16_128b(data, secret, seed)
    } else if len >= 4 {
        xxh3_len_4to8_128b(data, secret, seed)
    } else {
        xxh3_len_1to3_128b(data, secret, seed)
    }
}

fn main() {
    println!("=== Manual XXH3_128 Test ===");
    
    // Test cases from C reference
    let test_cases = vec![
        ("", 0x99aa06d3014798d8, 0x6001c324468d497f),
        ("a", 0xa96faf705af16834, 0xe6c632b61e964e1f),
        ("ab", 0x89c65ebc828eebac, 0xa873719c24d5735c),
        ("abc", 0x06b05ab6733a6185, 0x78af5f94892f3950),
    ];
    
    for (input, expected_high, expected_low) in test_cases {
        let result = xxh3_len_0to16_128b(input.as_bytes(), &XXH3_DEFAULT_SECRET, 0);
        let passed = result.high == expected_high && result.low == expected_low;
        
        println!("Input: '{}'", input);
        println!("  Expected: 0x{:016x}{:016x}", expected_high, expected_low);
        println!("  Got:      0x{:016x}{:016x}", result.high, result.low);
        println!("  {}", if passed { "✅ PASS" } else { "❌ FAIL" });
        println!();
    }
}