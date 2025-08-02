//! Constants for xxHash algorithms

// XXH32 constants
pub const XXH32_PRIME1: u32 = 0x9E3779B1;
pub const XXH32_PRIME2: u32 = 0x85EBCA77; 
pub const XXH32_PRIME3: u32 = 0xC2B2AE3D;
pub const XXH32_PRIME4: u32 = 0x27D4EB2F;
pub const XXH32_PRIME5: u32 = 0x165667B1;

// XXH64 constants  
pub const XXH64_PRIME1: u64 = 0x9E3779B185EBCA87;
pub const XXH64_PRIME2: u64 = 0xC2B2AE3D27D4EB4F;
pub const XXH64_PRIME3: u64 = 0x165667B19E3779F9;
pub const XXH64_PRIME4: u64 = 0x85EBCA77C2B2AE63;
pub const XXH64_PRIME5: u64 = 0x27D4EB2F165667C5;

// XXH3 constants
pub const XXH3_SECRET_SIZE_MIN: usize = 136;
pub const XXH3_SECRET_DEFAULT_SIZE: usize = 192;
pub const XXH3_MIDSIZE_MAX: usize = 240;
pub const XXH3_MIDSIZE_STARTOFFSET: usize = 3;
pub const XXH3_MIDSIZE_LASTOFFSET: usize = 17;

// XXH3 mixing constants from C source
pub const PRIME_MX1: u64 = 0x165667919E3779F9;
pub const PRIME_MX2: u64 = 0x9FB21C651E98DF25;

// XXH3 prime constants from C source
pub const XXH_PRIME64_1: u64 = 0x9E3779B185EBCA87;
pub const XXH_PRIME64_2: u64 = 0xC2B2AE3D27D4EB4F;
pub const XXH_PRIME64_4: u64 = 0x85EBCA77C2B2AE63;
pub const XXH_PRIME32_2: u32 = 0x85EBCA77;

// Default secret for XXH3 - exact copy from C source
pub const XXH3_DEFAULT_SECRET: [u8; XXH3_SECRET_DEFAULT_SIZE] = [
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

// Bit manipulation helpers
#[inline]
pub const fn rotl32(x: u32, r: u32) -> u32 {
    (x << r) | (x >> (32 - r))
}

#[inline] 
pub const fn rotl64(x: u64, r: u32) -> u64 {
    (x << r) | (x >> (64 - r))
}

#[inline]
pub const fn read_u32_le(data: &[u8]) -> u32 {
    u32::from_le_bytes([data[0], data[1], data[2], data[3]])
}

#[inline]
pub const fn read_u64_le(data: &[u8]) -> u64 {
    u64::from_le_bytes([
        data[0], data[1], data[2], data[3],
        data[4], data[5], data[6], data[7],
    ])
}

#[inline]
pub const fn read_u128_le(data: &[u8]) -> u128 {
    u128::from_le_bytes([
        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
        data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15],
    ])
}

// XXH64 avalanche function - different from XXH3_avalanche!
#[inline]
pub fn xxh64_avalanche(mut hash: u64) -> u64 {
    hash ^= hash >> 33;
    hash = hash.wrapping_mul(XXH64_PRIME2);
    hash ^= hash >> 29;
    hash = hash.wrapping_mul(XXH64_PRIME3);
    hash ^= hash >> 32;
    hash
}

// XXH3 avalanche function
#[inline]
pub fn xxh3_avalanche(mut h: u64) -> u64 {
    h ^= h >> 37;
    h = h.wrapping_mul(PRIME_MX1);
    h ^= h >> 32;
    h
}

// XXH3 xorshift function
#[inline]
pub fn xxh_xorshift64(v64: u64, shift: i32) -> u64 {
    v64 ^ (v64 >> shift)
}

// XXH3 128-bit multiplication function
#[inline]
pub fn xxh_mult64to128(lhs: u64, rhs: u64) -> (u64, u64) {
    // Portable scalar implementation
    let lo_lo = (lhs & 0xFFFFFFFF).wrapping_mul(rhs & 0xFFFFFFFF);
    let hi_lo = (lhs >> 32).wrapping_mul(rhs & 0xFFFFFFFF);
    let lo_hi = (lhs & 0xFFFFFFFF).wrapping_mul(rhs >> 32);
    let hi_hi = (lhs >> 32).wrapping_mul(rhs >> 32);

    let cross = (lo_lo >> 32) + (hi_lo & 0xFFFFFFFF) + lo_hi;
    let upper = (hi_lo >> 32) + (cross >> 32) + hi_hi;
    let lower = (cross << 32) | (lo_lo & 0xFFFFFFFF);

    (lower, upper)
}

// XXH3 32-bit to 64-bit multiplication
#[inline]
pub fn xxh_mult32to64(x: u64, y: u64) -> u64 {
    (x & 0xFFFFFFFF).wrapping_mul(y & 0xFFFFFFFF)
}