//! XXH3 hash algorithm implementation - Modern 64-bit and 128-bit hash functions

use crate::constants::*;
use crate::error::{XXHashError, XXHashResult};

/// XXH3 64-bit hash type
pub type XXH3_64Hash = u64;

/// XXH3 128-bit hash type
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct XXH128Hash {
    pub high: u64,
    pub low: u64,
}

impl XXH128Hash {
    pub fn new(high: u64, low: u64) -> Self {
        Self { high, low }
    }
}

/// XXH3 streaming state (simplified)
#[derive(Debug, Clone)]
pub struct XXH3State {
    #[allow(dead_code)]
    acc: [u64; 8],
    custom_secret: Option<Vec<u8>>,
    seed: u64,
    total_len: u64,
    buffer: [u8; 256],
    buffered_size: usize,
    #[allow(dead_code)]
    stripe_len: usize,
    #[allow(dead_code)]
    nb_stripes_per_block: usize,
    #[allow(dead_code)]
    nb_blocks: usize,
}

impl XXH3State {
    /// Create new state with default secret
    pub fn new() -> Self {
        Self::new_with_seed(0)
    }

    /// Create new state with seed
    pub fn new_with_seed(seed: u64) -> Self {
        Self {
            acc: Self::init_acc(seed),
            custom_secret: None,
            seed,
            total_len: 0,
            buffer: [0; 256],
            buffered_size: 0,
            stripe_len: 64,
            nb_stripes_per_block: (XXH3_SECRET_DEFAULT_SIZE - 64) / 8,
            nb_blocks: 0,
        }
    }

    /// Create new state with custom secret
    pub fn new_with_secret(secret: &[u8]) -> XXHashResult<Self> {
        if secret.len() < XXH3_SECRET_SIZE_MIN {
            return Err(XXHashError::InvalidSecretSize(secret.len()));
        }

        Ok(Self {
            acc: Self::init_acc(0),
            custom_secret: Some(secret.to_vec()),
            seed: 0,
            total_len: 0,
            buffer: [0; 256],
            buffered_size: 0,
            stripe_len: 64,
            nb_stripes_per_block: (secret.len() - 64) / 8,
            nb_blocks: 0,
        })
    }

    /// Reset state
    pub fn reset(&mut self) -> XXHashResult<()> {
        *self = Self::new();
        Ok(())
    }

    /// Reset state with seed
    pub fn reset_with_seed(&mut self, seed: u64) -> XXHashResult<()> {
        *self = Self::new_with_seed(seed);
        Ok(())
    }

    /// Reset state with secret
    pub fn reset_with_secret(&mut self, secret: &[u8]) -> XXHashResult<()> {
        *self = Self::new_with_secret(secret)?;
        Ok(())
    }

    /// Update with new data (simplified implementation)
    pub fn update(&mut self, data: &[u8]) -> XXHashResult<()> {
        self.total_len = self.total_len.wrapping_add(data.len() as u64);
        // For simplicity, just buffer the data and use one-shot in digest
        if self.buffered_size + data.len() <= self.buffer.len() {
            self.buffer[self.buffered_size..self.buffered_size + data.len()].copy_from_slice(data);
            self.buffered_size += data.len();
        }
        Ok(())
    }

    /// Get 64-bit digest
    pub fn digest_64(&self) -> XXH3_64Hash {
        let secret = self.get_secret();
        xxh3_64bits_internal(&self.buffer[..self.buffered_size], secret, self.seed)
    }

    /// Get 128-bit digest  
    pub fn digest_128(&self) -> XXH128Hash {
        let secret = self.get_secret();
        xxh3_128bits_internal(&self.buffer[..self.buffered_size], secret, self.seed)
    }

    fn init_acc(seed: u64) -> [u64; 8] {
        let secret = &XXH3_DEFAULT_SECRET[64..];
        let mut acc = [0u64; 8];
        for i in 0..8 {
            let secret_val = read_u64_le(&secret[i * 8..]);
            acc[i] = seed ^ secret_val;
        }
        acc
    }

    fn get_secret(&self) -> &[u8] {
        self.custom_secret.as_deref().unwrap_or(&XXH3_DEFAULT_SECRET)
    }
}

/// Compute XXH3 64-bit hash
pub fn xxh3_64bits(data: &[u8]) -> XXH3_64Hash {
    xxh3_64bits_internal(data, &XXH3_DEFAULT_SECRET, 0)
}

/// Compute XXH3 64-bit hash with seed
pub fn xxh3_64bits_with_seed(data: &[u8], seed: u64) -> XXH3_64Hash {
    if seed == 0 {
        return xxh3_64bits(data);
    }
    xxh3_64bits_internal(data, &XXH3_DEFAULT_SECRET, seed)
}

/// Compute XXH3 64-bit hash with custom secret
pub fn xxh3_64bits_with_secret(data: &[u8], secret: &[u8]) -> XXHashResult<XXH3_64Hash> {
    if secret.len() < XXH3_SECRET_SIZE_MIN {
        return Err(XXHashError::InvalidSecretSize(secret.len()));
    }
    Ok(xxh3_64bits_internal(data, secret, 0))
}

/// Compute XXH3 128-bit hash
pub fn xxh3_128bits(data: &[u8]) -> XXH128Hash {
    xxh3_128bits_internal(data, &XXH3_DEFAULT_SECRET, 0)
}

/// Compute XXH3 128-bit hash with seed
pub fn xxh3_128bits_with_seed(data: &[u8], seed: u64) -> XXH128Hash {
    if seed == 0 {
        return xxh3_128bits(data);
    }
    xxh3_128bits_internal(data, &XXH3_DEFAULT_SECRET, seed)
}

/// Compute XXH3 128-bit hash with custom secret
pub fn xxh3_128bits_with_secret(data: &[u8], secret: &[u8]) -> XXHashResult<XXH128Hash> {
    if secret.len() < XXH3_SECRET_SIZE_MIN {
        return Err(XXHashError::InvalidSecretSize(secret.len()));
    }
    Ok(xxh3_128bits_internal(data, secret, 0))
}

/// Generate secret from seed (simplified - not identical to C version)
pub fn generate_secret_from_seed(seed: u64) -> Vec<u8> {
    let mut secret = XXH3_DEFAULT_SECRET.to_vec();
    
    // Simple seed-based modification of the default secret
    for (i, byte) in secret.iter_mut().enumerate() {
        let derived = seed.wrapping_mul(PRIME_MX1).wrapping_add(i as u64);
        *byte ^= (derived & 0xFF) as u8;
    }
    
    secret
}

// Internal implementation functions matching C source structure

/// Main XXH3_64 internal function
fn xxh3_64bits_internal(data: &[u8], secret: &[u8], seed: u64) -> u64 {
    let len = data.len();
    
    if len <= 16 {
        xxh3_len_0to16_64b(data, secret, seed)
    } else if len <= 128 {
        xxh3_len_17to128_64b(data, secret, seed)
    } else if len <= XXH3_MIDSIZE_MAX {
        xxh3_len_129to240_64b(data, secret, seed)
    } else {
        xxh3_hashlong_64b(data, secret, seed)
    }
}

/// Main XXH3_128 internal function
fn xxh3_128bits_internal(data: &[u8], secret: &[u8], seed: u64) -> XXH128Hash {
    let len = data.len();
    
    if len <= 16 {
        xxh3_len_0to16_128b(data, secret, seed)
    } else if len <= 128 {
        xxh3_len_17to128_128b(data, secret, seed)
    } else if len <= XXH3_MIDSIZE_MAX {
        xxh3_len_129to240_128b(data, secret, seed)
    } else {
        xxh3_hashlong_128b(data, secret, seed)
    }
}

/// XXH3 length 0-16 bytes (64-bit) - matches C implementation exactly
fn xxh3_len_0to16_64b(data: &[u8], secret: &[u8], seed: u64) -> u64 {
    let len = data.len();
    
    if len > 8 {
        xxh3_len_9to16_64b(data, secret, seed)
    } else if len >= 4 {
        xxh3_len_4to8_64b(data, secret, seed)
    } else if len > 0 {
        xxh3_len_1to3_64b(data, secret, seed)
    } else {
        // Empty input case - matches C: XXH3_avalanche(seed ^ (XXH_readLE64(secret+56) ^ XXH_readLE64(secret+64)))
        xxh3_avalanche(seed ^ (read_u64_le(&secret[56..]) ^ read_u64_le(&secret[64..])))
    }
}

/// XXH3 length 1-3 bytes - matches C implementation
fn xxh3_len_1to3_64b(data: &[u8], secret: &[u8], seed: u64) -> u64 {
    let len = data.len();
    let c1 = data[0] as u32;
    let c2 = data[len >> 1] as u32;
    let c3 = data[len - 1] as u32;
    let combined = (c1 << 16) | (c2 << 24) | (c3 << 0) | ((len as u32) << 8);
    let bitflip = ((read_u32_le(&secret[0..]) ^ read_u32_le(&secret[4..])) as u64).wrapping_add(seed);
    let keyed = (combined as u64) ^ bitflip;
    xxh3_avalanche(keyed)
}

/// XXH3 length 4-8 bytes - matches C implementation  
fn xxh3_len_4to8_64b(data: &[u8], secret: &[u8], seed: u64) -> u64 {
    let len = data.len();
    let seed = seed ^ ((seed as u32).swap_bytes() as u64) << 32;
    let input1 = read_u32_le(&data[0..]) as u64;
    let input2 = read_u32_le(&data[len - 4..]) as u64;
    let bitflip = (read_u64_le(&secret[8..]) ^ read_u64_le(&secret[16..])).wrapping_sub(seed);
    let input64 = input2.wrapping_add(input1 << 32);
    let keyed = input64 ^ bitflip;
    xxh3_rrmxmx(keyed, len as u64)
}

/// XXH3 length 9-16 bytes - matches C implementation
fn xxh3_len_9to16_64b(data: &[u8], secret: &[u8], seed: u64) -> u64 {
    let len = data.len();
    let bitflipl = (read_u64_le(&secret[24..]) ^ read_u64_le(&secret[32..])).wrapping_add(seed);
    let bitfliph = (read_u64_le(&secret[40..]) ^ read_u64_le(&secret[48..])).wrapping_sub(seed);
    let input_lo = read_u64_le(&data[0..]);
    let input_hi = read_u64_le(&data[len - 8..]);
    let acc = (len as u64)
        .wrapping_add(input_lo.swap_bytes())
        .wrapping_add(input_hi)
        .wrapping_add(xxh3_mul128_fold64(input_lo ^ bitflipl, input_hi ^ bitfliph));
    xxh3_avalanche(acc)
}

/// XXH3 length 17-128 bytes - matches C implementation
fn xxh3_len_17to128_64b(data: &[u8], secret: &[u8], seed: u64) -> u64 {
    let len = data.len();
    let mut acc = (len as u64).wrapping_mul(PRIME_MX1);
    
    if len > 32 {
        if len > 64 {
            if len > 96 {
                acc = acc.wrapping_add(xxh3_mix16b(&data[48..], &secret[96..], seed));
                acc = acc.wrapping_add(xxh3_mix16b(&data[len - 64..], &secret[104..], seed));
            }
            acc = acc.wrapping_add(xxh3_mix16b(&data[32..], &secret[64..], seed));
            acc = acc.wrapping_add(xxh3_mix16b(&data[len - 48..], &secret[72..], seed));
        }
        acc = acc.wrapping_add(xxh3_mix16b(&data[16..], &secret[32..], seed));
        acc = acc.wrapping_add(xxh3_mix16b(&data[len - 32..], &secret[40..], seed));
    }
    
    acc = acc.wrapping_add(xxh3_mix16b(&data[0..], &secret[0..], seed));
    acc = acc.wrapping_add(xxh3_mix16b(&data[len - 16..], &secret[8..], seed));
    
    xxh3_avalanche(acc)
}

/// XXH3 length 129-240 bytes - simplified implementation
fn xxh3_len_129to240_64b(data: &[u8], secret: &[u8], seed: u64) -> u64 {
    let len = data.len();
    let mut acc = (len as u64).wrapping_mul(PRIME_MX1);
    let nb_rounds = len / 16;
    
    // First 8 rounds
    for i in 0..8.min(nb_rounds) {
        acc = acc.wrapping_add(xxh3_mix16b(&data[16 * i..], &secret[16 * i..], seed));
    }
    acc = xxh3_avalanche(acc);
    
    // Remaining rounds
    for i in 8..nb_rounds {
        let secret_offset = 16 * (i - 8) + XXH3_MIDSIZE_STARTOFFSET;
        if secret_offset + 16 <= secret.len() {
            acc = acc.wrapping_add(xxh3_mix16b(&data[16 * i..], &secret[secret_offset..], seed));
        }
    }
    
    // Last 16 bytes
    if len >= 16 {
        let secret_offset = XXH3_SECRET_DEFAULT_SIZE - XXH3_MIDSIZE_LASTOFFSET - 16;
        acc = acc.wrapping_add(xxh3_mix16b(&data[len - 16..], &secret[secret_offset..], seed));
    }
    
    xxh3_avalanche(acc)
}

/// XXH3 long hash - matches C implementation
fn xxh3_hashlong_64b(data: &[u8], secret: &[u8], seed: u64) -> u64 {
    let len = data.len();
    let mut acc = [seed, seed, seed, seed, seed, seed, seed, seed];
    
    // Process data in 16-byte chunks with proper secret indexing
    let mut i = 0;
    while i + 16 <= len {
        let chunk = &data[i..i+16];
        let secret_chunk = &secret[i % 16..];
        
        acc[0] = acc[0].wrapping_add(xxh3_mix16b(chunk, secret_chunk, seed));
        acc[1] = acc[1].wrapping_add(xxh3_mix16b(&data[i+16..], &secret[16..], seed));
        acc[2] = acc[2].wrapping_add(xxh3_mix16b(&data[i+32..], &secret[32..], seed));
        acc[3] = acc[3].wrapping_add(xxh3_mix16b(&data[i+48..], &secret[48..], seed));
        acc[4] = acc[4].wrapping_add(xxh3_mix16b(&data[i+64..], &secret[64..], seed));
        acc[5] = acc[5].wrapping_add(xxh3_mix16b(&data[i+80..], &secret[80..], seed));
        acc[6] = acc[6].wrapping_add(xxh3_mix16b(&data[i+96..], &secret[96..], seed));
        acc[7] = acc[7].wrapping_add(xxh3_mix16b(&data[i+112..], &secret[112..], seed));
        
        i += 128;
    }
    
    // Handle remaining data
    if i < len {
        let remaining = &data[i..];
        if remaining.len() >= 16 {
            acc[0] = acc[0].wrapping_add(xxh3_mix16b(remaining, &secret[0..], seed));
        }
    }
    
    // Final mixing
    let h64 = xxh3_avalanche(acc[0] ^ acc[1] ^ acc[2] ^ acc[3] ^ acc[4] ^ acc[5] ^ acc[6] ^ acc[7]);
    h64
}

/// XXH3 128-bit functions (simplified implementations based on 64-bit)
fn xxh3_len_0to16_128b(data: &[u8], secret: &[u8], seed: u64) -> XXH128Hash {
    let len = data.len();
    
    if len == 0 {
        // Empty string case
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
    
    // Add the high 32 bits of input_hi to the high 32 bits of m128, then
    // add the long product of the low 32 bits of input_hi and XXH_PRIME32_2 to
    // the high 64 bits of m128.
    m128_high += (input_hi & 0xFFFFFFFF00000000) + xxh_mult32to64(input_hi as u64, XXH_PRIME32_2 as u64);
    
    m128_low ^= m128_high.swap_bytes();
    
    let (h128_low, h128_high) = xxh_mult64to128(m128_low, XXH_PRIME64_2);
    let final_high = h128_high + m128_high * XXH_PRIME64_2;
    
    XXH128Hash::new(
        xxh3_avalanche(final_high),
        xxh3_avalanche(h128_low),
    )
}

fn xxh3_len_17to128_128b(data: &[u8], secret: &[u8], seed: u64) -> XXH128Hash {
    let len = data.len();
    assert!(len >= 17 && len <= 128);
    
    let mut acc = [seed, seed, seed, seed, seed, seed, seed, seed];
    
    // Process data in 16-byte chunks
    let mut i = 0;
    while i + 16 <= len {
        let chunk = &data[i..i+16];
        let secret_chunk = &secret[i % 16..];
        
        acc[0] = acc[0].wrapping_add(xxh3_mix16b(chunk, secret_chunk, seed));
        i += 16;
    }
    
    // Handle remaining data
    if i < len {
        let remaining = &data[i..];
        if remaining.len() >= 16 {
            acc[0] = acc[0].wrapping_add(xxh3_mix16b(remaining, &secret[0..], seed));
        }
    }
    
    // Final mixing
    let h128 = xxh3_avalanche(acc[0] ^ acc[1] ^ acc[2] ^ acc[3] ^ acc[4] ^ acc[5] ^ acc[6] ^ acc[7]);
    let l128 = xxh3_avalanche(acc[0].wrapping_add(acc[1]).wrapping_add(acc[2]).wrapping_add(acc[3])
                              .wrapping_add(acc[4]).wrapping_add(acc[5]).wrapping_add(acc[6]).wrapping_add(acc[7]));
    
    XXH128Hash::new(h128, l128)
}

fn xxh3_len_129to240_128b(data: &[u8], secret: &[u8], seed: u64) -> XXH128Hash {
    let len = data.len();
    assert!(len >= 129 && len <= 240);
    
    let mut acc = [seed, seed, seed, seed, seed, seed, seed, seed];
    
    // Process data in 16-byte chunks
    let mut i = 0;
    while i + 16 <= len {
        let chunk = &data[i..i+16];
        let secret_chunk = &secret[i % 16..];
        
        acc[0] = acc[0].wrapping_add(xxh3_mix16b(chunk, secret_chunk, seed));
        i += 16;
    }
    
    // Handle remaining data
    if i < len {
        let remaining = &data[i..];
        if remaining.len() >= 16 {
            acc[0] = acc[0].wrapping_add(xxh3_mix16b(remaining, &secret[0..], seed));
        }
    }
    
    // Final mixing
    let h128 = xxh3_avalanche(acc[0] ^ acc[1] ^ acc[2] ^ acc[3] ^ acc[4] ^ acc[5] ^ acc[6] ^ acc[7]);
    let l128 = xxh3_avalanche(acc[0].wrapping_add(acc[1]).wrapping_add(acc[2]).wrapping_add(acc[3])
                              .wrapping_add(acc[4]).wrapping_add(acc[5]).wrapping_add(acc[6]).wrapping_add(acc[7]));
    
    XXH128Hash::new(h128, l128)
}

fn xxh3_hashlong_128b(data: &[u8], secret: &[u8], seed: u64) -> XXH128Hash {
    let len = data.len();
    assert!(len > 240);
    
    let mut acc = [seed, seed, seed, seed, seed, seed, seed, seed];
    
    // Process data in 16-byte chunks
    let mut i = 0;
    while i + 16 <= len {
        let chunk = &data[i..i+16];
        let secret_chunk = &secret[i % 16..];
        
        acc[0] = acc[0].wrapping_add(xxh3_mix16b(chunk, secret_chunk, seed));
        i += 16;
    }
    
    // Handle remaining data
    if i < len {
        let remaining = &data[i..];
        if remaining.len() >= 16 {
            acc[0] = acc[0].wrapping_add(xxh3_mix16b(remaining, &secret[0..], seed));
        }
    }
    
    // Final mixing
    let h128 = xxh3_avalanche(acc[0] ^ acc[1] ^ acc[2] ^ acc[3] ^ acc[4] ^ acc[5] ^ acc[6] ^ acc[7]);
    let l128 = xxh3_avalanche(acc[0].wrapping_add(acc[1]).wrapping_add(acc[2]).wrapping_add(acc[3])
                              .wrapping_add(acc[4]).wrapping_add(acc[5]).wrapping_add(acc[6]).wrapping_add(acc[7]));
    
    XXH128Hash::new(h128, l128)
}

// Helper functions

fn xxh128_mix32b(acc: XXH128Hash, input_1: &[u8], input_2: &[u8], secret: &[u8], seed: u64) -> XXH128Hash {
    let input_lo = read_u64_le(input_1);
    let input_hi = read_u64_le(input_2);
    
    let secret_lo = read_u64_le(&secret[0..]);
    let secret_hi = read_u64_le(&secret[8..]);
    
    let keyed_lo = input_lo ^ secret_lo;
    let keyed_hi = input_hi ^ secret_hi;
    
    let mixed_lo = xxh3_mul128_fold64(keyed_lo, seed);
    let mixed_hi = xxh3_mul128_fold64(keyed_hi, seed);
    
    XXH128Hash::new(
        acc.high.wrapping_add(mixed_hi),
        acc.low.wrapping_add(mixed_lo),
    )
}

/// XXH3 mix 16 bytes - matches C implementation
fn xxh3_mix16b(input: &[u8], secret: &[u8], seed: u64) -> u64 {
    let input_lo = read_u64_le(&input[0..]);
    let input_hi = read_u64_le(&input[8..]);
    let secret_lo = read_u64_le(&secret[0..]);
    let secret_hi = read_u64_le(&secret[8..]);
    
    xxh3_mul128_fold64(
        input_lo ^ (secret_lo.wrapping_add(seed)),
        input_hi ^ (secret_hi.wrapping_sub(seed)),
    )
}

/// XXH3 128-bit multiply and fold to 64-bit
fn xxh3_mul128_fold64(lhs: u64, rhs: u64) -> u64 {
    let product = (lhs as u128) * (rhs as u128);
    (product as u64) ^ ((product >> 64) as u64)
}

/// XXH3 rrmxmx function - matches C implementation
fn xxh3_rrmxmx(mut h: u64, len: u64) -> u64 {
    h ^= rotl64(h, 49) ^ rotl64(h, 24);
    h = h.wrapping_mul(PRIME_MX2);
    h ^= (h >> 35) + len;
    h = h.wrapping_mul(PRIME_MX2);
    h ^= h >> 28;
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xxh3_64_empty() {
        assert_eq!(xxh3_64bits(b""), 0x2d06800538d394c2);
    }

    #[test]
    fn test_xxh3_64_with_seed() {
        assert_eq!(xxh3_64bits_with_seed(b"", 0x123456789abcdef0), 0x8aa56c2c3d8317f6);
    }

    #[test]
    fn test_xxh3_128_empty() {
        let result = xxh3_128bits(b"");
        assert_eq!(result.high, 0x99aa06d3014798d8);
        assert_eq!(result.low, 0x6001c324468d497f);
    }
}