//! XXH64 hash algorithm implementation

use crate::constants::*;
use crate::error::XXHashResult;

/// XXH64 hash type
pub type XXH64Hash = u64;

/// XXH64 streaming state
#[derive(Debug, Clone)]
pub struct XXH64State {
    total_len: u64,
    large_len: bool,
    v: [u64; 4],
    mem64: [u8; 32],
    memsize: usize,
    seed: u64,
}

impl XXH64State {
    /// Create a new XXH64 state with seed
    pub fn new(seed: u64) -> Self {
        Self {
            total_len: 0,
            large_len: false,
            v: [
                seed.wrapping_add(XXH64_PRIME1).wrapping_add(XXH64_PRIME2),
                seed.wrapping_add(XXH64_PRIME2),
                seed,
                seed.wrapping_sub(XXH64_PRIME1),
            ],
            mem64: [0; 32],
            memsize: 0,
            seed,
        }
    }

    /// Reset state with new seed
    pub fn reset(&mut self, seed: u64) -> XXHashResult<()> {
        *self = Self::new(seed);
        Ok(())
    }

    /// Update hash with new data
    pub fn update(&mut self, data: &[u8]) -> XXHashResult<()> {
        self.total_len = self.total_len.wrapping_add(data.len() as u64);
        self.large_len |= (data.len() >= 32) | (self.total_len >= 32);

        let mut input = data;

        // Fill buffer if we have leftover data
        if self.memsize > 0 {
            let to_fill = 32 - self.memsize;
            if input.len() < to_fill {
                // Not enough data to fill buffer
                self.mem64[self.memsize..self.memsize + input.len()].copy_from_slice(input);
                self.memsize += input.len();
                return Ok(());
            }

            // Fill buffer and process
            self.mem64[self.memsize..32].copy_from_slice(&input[..to_fill]);
            self.consume_buffer();
            input = &input[to_fill..];
            self.memsize = 0;
        }

        // Process complete 32-byte chunks
        while input.len() >= 32 {
            self.v[0] = xxh64_round(self.v[0], read_u64_le(&input[0..]));
            self.v[1] = xxh64_round(self.v[1], read_u64_le(&input[8..]));
            self.v[2] = xxh64_round(self.v[2], read_u64_le(&input[16..]));
            self.v[3] = xxh64_round(self.v[3], read_u64_le(&input[24..]));
            input = &input[32..];
        }

        // Store remaining data in buffer
        if !input.is_empty() {
            self.mem64[..input.len()].copy_from_slice(input);
            self.memsize = input.len();
        }

        Ok(())
    }

    /// Finalize and get hash
    pub fn digest(&self) -> XXH64Hash {
        let mut h = if self.large_len {
            let mut h = rotl64(self.v[0], 1)
                .wrapping_add(rotl64(self.v[1], 7))
                .wrapping_add(rotl64(self.v[2], 12))
                .wrapping_add(rotl64(self.v[3], 18));

            h = xxh64_merge_round(h, self.v[0]);
            h = xxh64_merge_round(h, self.v[1]);
            h = xxh64_merge_round(h, self.v[2]);
            h = xxh64_merge_round(h, self.v[3]);
            h
        } else {
            self.seed.wrapping_add(XXH64_PRIME5)
        };

        h = h.wrapping_add(self.total_len);

        // Process remaining bytes
        let mut remaining = &self.mem64[..self.memsize];
        
        // Process 8-byte chunks
        while remaining.len() >= 8 {
            let k1 = xxh64_round(0, read_u64_le(remaining));
            h ^= k1;
            h = rotl64(h, 27).wrapping_mul(XXH64_PRIME1).wrapping_add(XXH64_PRIME4);
            remaining = &remaining[8..];
        }

        // Process 4-byte chunk
        if remaining.len() >= 4 {
            h ^= (read_u32_le(remaining) as u64).wrapping_mul(XXH64_PRIME1);
            h = rotl64(h, 23).wrapping_mul(XXH64_PRIME2).wrapping_add(XXH64_PRIME3);
            remaining = &remaining[4..];
        }

        // Process remaining bytes
        for &byte in remaining {
            h ^= (byte as u64).wrapping_mul(XXH64_PRIME5);
            h = rotl64(h, 11).wrapping_mul(XXH64_PRIME1);
        }

        // Avalanche
        h ^= h >> 33;
        h = h.wrapping_mul(XXH64_PRIME2);
        h ^= h >> 29;
        h = h.wrapping_mul(XXH64_PRIME3);
        h ^= h >> 32;

        h
    }

    fn consume_buffer(&mut self) {
        self.v[0] = xxh64_round(self.v[0], read_u64_le(&self.mem64[0..]));
        self.v[1] = xxh64_round(self.v[1], read_u64_le(&self.mem64[8..]));
        self.v[2] = xxh64_round(self.v[2], read_u64_le(&self.mem64[16..]));
        self.v[3] = xxh64_round(self.v[3], read_u64_le(&self.mem64[24..]));
    }
}

#[inline]
fn xxh64_round(acc: u64, input: u64) -> u64 {
    let acc = acc.wrapping_add(input.wrapping_mul(XXH64_PRIME2));
    rotl64(acc, 31).wrapping_mul(XXH64_PRIME1)
}

#[inline]
fn xxh64_merge_round(mut acc: u64, val: u64) -> u64 {
    let val = xxh64_round(0, val);
    acc ^= val;
    acc.wrapping_mul(XXH64_PRIME1).wrapping_add(XXH64_PRIME4)
}

/// Compute XXH64 hash with seed 0
pub fn xxh64(data: &[u8]) -> XXH64Hash {
    xxh64_with_seed(data, 0)
}

/// Compute XXH64 hash with specified seed
pub fn xxh64_with_seed(data: &[u8], seed: u64) -> XXH64Hash {
    if data.len() < 32 {
        xxh64_finalize(seed.wrapping_add(XXH64_PRIME5), data, data.len() as u64)
    } else {
        let mut v = [
            seed.wrapping_add(XXH64_PRIME1).wrapping_add(XXH64_PRIME2),
            seed.wrapping_add(XXH64_PRIME2),
            seed,
            seed.wrapping_sub(XXH64_PRIME1),
        ];

        let mut input = data;
        while input.len() >= 32 {
            v[0] = xxh64_round(v[0], read_u64_le(&input[0..]));
            v[1] = xxh64_round(v[1], read_u64_le(&input[8..]));
            v[2] = xxh64_round(v[2], read_u64_le(&input[16..]));
            v[3] = xxh64_round(v[3], read_u64_le(&input[24..]));
            input = &input[32..];
        }

        let mut h = rotl64(v[0], 1)
            .wrapping_add(rotl64(v[1], 7))
            .wrapping_add(rotl64(v[2], 12))
            .wrapping_add(rotl64(v[3], 18));

        h = xxh64_merge_round(h, v[0]);
        h = xxh64_merge_round(h, v[1]);
        h = xxh64_merge_round(h, v[2]);
        h = xxh64_merge_round(h, v[3]);

        xxh64_finalize(h, input, data.len() as u64)
    }
}

#[inline]
fn xxh64_finalize(mut h: u64, remaining: &[u8], total_len: u64) -> u64 {
    h = h.wrapping_add(total_len);

    let mut input = remaining;
    
    // Process 8-byte chunks
    while input.len() >= 8 {
        let k1 = xxh64_round(0, read_u64_le(input));
        h ^= k1;
        h = rotl64(h, 27).wrapping_mul(XXH64_PRIME1).wrapping_add(XXH64_PRIME4);
        input = &input[8..];
    }

    // Process 4-byte chunk
    if input.len() >= 4 {
        h ^= (read_u32_le(input) as u64).wrapping_mul(XXH64_PRIME1);
        h = rotl64(h, 23).wrapping_mul(XXH64_PRIME2).wrapping_add(XXH64_PRIME3);
        input = &input[4..];
    }

    // Process remaining bytes
    for &byte in input {
        h ^= (byte as u64).wrapping_mul(XXH64_PRIME5);
        h = rotl64(h, 11).wrapping_mul(XXH64_PRIME1);
    }

    // Avalanche
    h ^= h >> 33;
    h = h.wrapping_mul(XXH64_PRIME2);
    h ^= h >> 29;
    h = h.wrapping_mul(XXH64_PRIME3);
    h ^= h >> 32;

    h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xxh64_empty() {
        assert_eq!(xxh64(b""), 0xef46db3751d8e999);
        assert_eq!(xxh64_with_seed(b"", 0x123456789abcdef0), 0x7fef5b0c316777ed);
    }

    #[test]
    fn test_xxh64_single_byte() {
        assert_eq!(xxh64(b"a"), 0xd24ec4f1a98c6e5b);
        assert_eq!(xxh64_with_seed(b"a", 0x123456789abcdef0), 0x9e29aa7f69e1808f);
    }

    #[test]
    fn test_xxh64_streaming() {
        let mut state = XXH64State::new(0);
        state.update(b"hell").unwrap();
        state.update(b"o world").unwrap();
        assert_eq!(state.digest(), xxh64(b"hello world"));
    }
}