#include <stdio.h>
#include <string.h>
#include <stdint.h>
#define XXH_STATIC_LINKING_ONLY
#include "xxhash.h"

// Test data for comparison
const char* test_strings[] = {
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
    NULL
};

void print_hash_32(const char* input, uint32_t seed) {
    uint32_t hash = XXH32(input, strlen(input), seed);
    printf("XXH32('%s', 0x%08x) = 0x%08x\n", input, seed, hash);
}

void print_hash_64(const char* input, uint64_t seed) {
    uint64_t hash = XXH64(input, strlen(input), seed);
    printf("XXH64('%s', 0x%016lx) = 0x%016lx\n", input, seed, hash);
}

void print_hash_3_64(const char* input) {
    uint64_t hash = XXH3_64bits(input, strlen(input));
    printf("XXH3_64('%s') = 0x%016lx\n", input, hash);
}

void print_hash_3_64_seed(const char* input, uint64_t seed) {
    uint64_t hash = XXH3_64bits_withSeed(input, strlen(input), seed);
    printf("XXH3_64('%s', 0x%016lx) = 0x%016lx\n", input, seed, hash);
}

void print_hash_3_128(const char* input) {
    XXH128_hash_t hash = XXH3_128bits(input, strlen(input));
    printf("XXH3_128('%s') = 0x%016lx%016lx\n", input, hash.high64, hash.low64);
}

void print_hash_3_128_seed(const char* input, uint64_t seed) {
    XXH128_hash_t hash = XXH3_128bits_withSeed(input, strlen(input), seed);
    printf("XXH3_128('%s', 0x%016lx) = 0x%016lx%016lx\n", input, seed, hash.high64, hash.low64);
}

void print_hash_3_64_secret(const char* input) {
    // Create a custom secret based on the default secret
    unsigned char custom_secret[192];
    XXH3_generateSecret_fromSeed(custom_secret, 0);
    uint64_t hash = XXH3_64bits_withSecret(input, strlen(input), custom_secret, 192);
    printf("XXH3_64_secret('%s') = 0x%016lx\n", input, hash);
}

void print_hash_3_128_secret(const char* input) {
    // Create a custom secret based on the default secret
    unsigned char custom_secret[192];
    XXH3_generateSecret_fromSeed(custom_secret, 0);
    XXH128_hash_t hash = XXH3_128bits_withSecret(input, strlen(input), custom_secret, 192);
    printf("XXH3_128_secret('%s') = 0x%016lx%016lx\n", input, hash.high64, hash.low64);
}

int main() {
    printf("=== xxHash C Reference Implementation Test ===\n\n");
    
    // Test seeds
    uint32_t seed32 = 0x12345678;
    uint64_t seed64 = 0x123456789abcdef0;
    
    printf("--- XXH32 Tests ---\n");
    for (int i = 0; test_strings[i] != NULL; i++) {
        print_hash_32(test_strings[i], 0);
        print_hash_32(test_strings[i], seed32);
    }
    
    printf("\n--- XXH64 Tests ---\n");
    for (int i = 0; test_strings[i] != NULL; i++) {
        print_hash_64(test_strings[i], 0);
        print_hash_64(test_strings[i], seed64);
    }
    
    printf("\n--- XXH3_64 Tests ---\n");
    for (int i = 0; test_strings[i] != NULL; i++) {
        print_hash_3_64(test_strings[i]);
        print_hash_3_64_seed(test_strings[i], seed64);
    }
    
    printf("\n--- XXH3_128 Tests ---\n");
    for (int i = 0; test_strings[i] != NULL; i++) {
        print_hash_3_128(test_strings[i]);
        print_hash_3_128_seed(test_strings[i], seed64);
    }
    
    printf("\n--- XXH3 Secret Tests ---\n");
    for (int i = 0; test_strings[i] != NULL; i++) {
        print_hash_3_64_secret(test_strings[i]);
        print_hash_3_128_secret(test_strings[i]);
    }
    
    printf("\n=== Test Complete ===\n");
    return 0;
} 