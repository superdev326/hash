#!/usr/bin/env python3
# Manual test script for XXH3_128 implementation
# This simulates the key functions to verify our implementation

import struct

# Constants from our implementation
XXH_PRIME64_1 = 0x9E3779B185EBCA87
XXH_PRIME64_2 = 0xC2B2AE3D27D4EB4F
XXH_PRIME64_3 = 0x165667B19E3779F9
XXH_PRIME64_4 = 0x85EBCA77C2B2AE63
XXH_PRIME32_2 = 0x85EBCA77
PRIME_MX1 = 0x165667919E3779F9
PRIME_MX2 = 0x9FB21C651E98DF25

# Default secret from C reference
XXH3_DEFAULT_SECRET = bytes([
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
])

def read_u32_le(data):
    return struct.unpack('<I', data[:4])[0]

def read_u64_le(data):
    return struct.unpack('<Q', data[:8])[0]

def rotl32(x, r):
    return ((x << r) | (x >> (32 - r))) & 0xFFFFFFFF

def swap32(x):
    return ((x << 24) & 0xFF000000) | ((x << 8) & 0x00FF0000) | ((x >> 8) & 0x0000FF00) | ((x >> 24) & 0x000000FF)

def xxh64_avalanche(hash_val):
    hash_val ^= hash_val >> 33
    hash_val = (hash_val * XXH_PRIME64_2) & 0xFFFFFFFFFFFFFFFF
    hash_val ^= hash_val >> 29
    hash_val = (hash_val * XXH_PRIME64_3) & 0xFFFFFFFFFFFFFFFF
    hash_val ^= hash_val >> 32
    return hash_val

def xxh3_avalanche(h):
    h ^= h >> 37
    h = (h * PRIME_MX1) & 0xFFFFFFFFFFFFFFFF
    h ^= h >> 32
    return h

def xxh_xorshift64(v64, shift):
    return v64 ^ (v64 >> shift)

def xxh_mult64to128(lhs, rhs):
    # Portable scalar implementation
    lo_lo = (lhs & 0xFFFFFFFF) * (rhs & 0xFFFFFFFF)
    hi_lo = (lhs >> 32) * (rhs & 0xFFFFFFFF)
    lo_hi = (lhs & 0xFFFFFFFF) * (rhs >> 32)
    hi_hi = (lhs >> 32) * (rhs >> 32)

    cross = (lo_lo >> 32) + (hi_lo & 0xFFFFFFFF) + lo_hi
    upper = (hi_lo >> 32) + (cross >> 32) + hi_hi
    lower = (cross << 32) | (lo_lo & 0xFFFFFFFF)

    return lower, upper

def xxh_mult32to64(x, y):
    return (x & 0xFFFFFFFF) * (y & 0xFFFFFFFF)

def xxh3_len_1to3_128b(data, secret, seed):
    length = len(data)
    assert 1 <= length <= 3
    
    c1 = data[0]
    c2 = data[length >> 1]
    c3 = data[length - 1]
    
    combinedl = ((c1 << 16) | (c2 << 24) | (c3 << 0) | (length << 8)) & 0xFFFFFFFF
    combinedh = rotl32(swap32(combinedl), 13)
    
    bitflipl = (read_u32_le(secret[0:]) ^ read_u32_le(secret[4:])) + seed
    bitfliph = (read_u32_le(secret[8:]) ^ read_u32_le(secret[12:])) - seed
    
    keyed_lo = combinedl ^ bitflipl
    keyed_hi = combinedh ^ bitfliph
    
    return (
        xxh64_avalanche(keyed_hi),
        xxh64_avalanche(keyed_lo)
    )

def xxh3_len_4to8_128b(data, secret, seed):
    length = len(data)
    assert 4 <= length <= 8
    
    seed = seed ^ ((seed ^ 0xFFFFFFFFFFFFFFFF) << 32)
    
    input_lo = read_u32_le(data[0:]) & 0xFFFFFFFF
    input_hi = read_u32_le(data[length - 4:]) & 0xFFFFFFFF
    input_64 = input_lo + (input_hi << 32)
    
    bitflip = (read_u64_le(secret[16:]) ^ read_u64_le(secret[24:])) + seed
    keyed = input_64 ^ bitflip
    
    low64, high64 = xxh_mult64to128(keyed, XXH_PRIME64_1 + (length << 2))
    
    m128_low = low64
    m128_high = high64
    
    m128_high += m128_low << 1
    m128_low ^= m128_high >> 3
    
    m128_low = xxh_xorshift64(m128_low, 35)
    m128_low = (m128_low * PRIME_MX2) & 0xFFFFFFFFFFFFFFFF
    m128_low = xxh_xorshift64(m128_low, 28)
    m128_high = xxh3_avalanche(m128_high)
    
    return (m128_high, m128_low)

def xxh3_len_9to16_128b(data, secret, seed):
    length = len(data)
    assert 9 <= length <= 16
    
    bitflipl = (read_u64_le(secret[32:]) ^ read_u64_le(secret[40:])) - seed
    bitfliph = (read_u64_le(secret[48:]) ^ read_u64_le(secret[56:])) + seed
    
    input_lo = read_u64_le(data[0:])
    input_hi = read_u64_le(data[length - 8:])
    
    low64, high64 = xxh_mult64to128(input_lo ^ input_hi ^ bitflipl, XXH_PRIME64_1)
    
    m128_low = low64 + ((length - 1) << 54)
    m128_high = high64
    
    input_hi ^= bitfliph
    
    m128_high += (input_hi & 0xFFFFFFFF00000000) + xxh_mult32to64(input_hi, XXH_PRIME32_2)
    
    m128_low ^= m128_high ^ 0xFFFFFFFFFFFFFFFF
    
    h128_low, h128_high = xxh_mult64to128(m128_low, XXH_PRIME64_2)
    final_high = h128_high + m128_high * XXH_PRIME64_2
    
    return (
        xxh3_avalanche(final_high),
        xxh3_avalanche(h128_low)
    )

def xxh3_len_0to16_128b(data, secret, seed):
    length = len(data)
    
    if length == 0:
        bitflipl = read_u64_le(secret[64:]) ^ read_u64_le(secret[72:])
        bitfliph = read_u64_le(secret[80:]) ^ read_u64_le(secret[88:])
        low64 = xxh64_avalanche(seed ^ bitflipl)
        high64 = xxh64_avalanche(seed ^ bitfliph)
        return (high64, low64)
    elif length > 8:
        return xxh3_len_9to16_128b(data, secret, seed)
    elif length >= 4:
        return xxh3_len_4to8_128b(data, secret, seed)
    else:
        return xxh3_len_1to3_128b(data, secret, seed)

def main():
    print("=== Manual XXH3_128 Test ===")
    
    # Test cases from C reference
    test_cases = [
        ("", 0x99aa06d3014798d8, 0x6001c324468d497f),
        ("a", 0xa96faf705af16834, 0xe6c632b61e964e1f),
        ("ab", 0x89c65ebc828eebac, 0xa873719c24d5735c),
        ("abc", 0x06b05ab6733a6185, 0x78af5f94892f3950),
    ]
    
    for input_str, expected_high, expected_low in test_cases:
        result_high, result_low = xxh3_len_0to16_128b(input_str.encode(), XXH3_DEFAULT_SECRET, 0)
        passed = result_high == expected_high and result_low == expected_low
        
        print(f"Input: '{input_str}'")
        print(f"  Expected: 0x{expected_high:016x}{expected_low:016x}")
        print(f"  Got:      0x{result_high:016x}{result_low:016x}")
        print(f"  {'✅ PASS' if passed else '❌ FAIL'}")
        print()

if __name__ == "__main__":
    main()