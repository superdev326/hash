//! XXH32 hash algorithm implementation

use crate::constants::*;
use crate::error::XXHashResult;

/// XXH32 hash type
pub type XXH32Hash = u32;

/// XXH32 streaming state
#[derive(Debug, Clone)]
pub struct XXH32State {
    total_len: u64,
    large_len: bool,
    v: [u32; 4],
    mem32: [u8; 16],
    memsize: usize,
    seed: u32,
}

impl XXH32State {
    /// Create a new XXH32 state with seed
    pub fn new(seed: u32) -> Self {
        Self {
            total_len: 0,
            large_len: false,
            v: [
                seed.wrapping_add(XXH32_PRIME1).wrapping_add(XXH32_PRIME2),
                seed.wrapping_add(XXH32_PRIME2),
                seed,
                seed.wrapping_sub(XXH32_PRIME1),
            ],
            mem32: [0; 16],
            memsize: 0,
            seed,
        }
    }

    /// Reset state with new seed
    pub fn reset(&mut self, seed: u32) -> XXHashResult<()> {
        *self = Self::new(seed);
        Ok(())
    }

    /// Update hash with new data
    pub fn update(&mut self, data: &[u8]) -> XXHashResult<()> {
        self.total_len = self.total_len.wrapping_add(data.len() as u64);
        self.large_len |= (data.len() >= 16) | (self.total_len >= 16);

        let mut input = data;

        // Fill buffer if we have leftover data
        if self.memsize > 0 {
            let to_fill = 16 - self.memsize;
            if input.len() < to_fill {
                // Not enough data to fill buffer
                self.mem32[self.memsize..self.memsize + input.len()].copy_from_slice(input);
                self.memsize += input.len();
                return Ok(());
            }

            // Fill buffer and process
            self.mem32[self.memsize..16].copy_from_slice(&input[..to_fill]);
            self.consume_buffer();
            input = &input[to_fill..];
            self.memsize = 0;
        }

        // Process complete 16-byte chunks
        while input.len() >= 16 {
            self.v[0] = xxh32_round(self.v[0], read_u32_le(&input[0..]));
            self.v[1] = xxh32_round(self.v[1], read_u32_le(&input[4..]));
            self.v[2] = xxh32_round(self.v[2], read_u32_le(&input[8..]));
            self.v[3] = xxh32_round(self.v[3], read_u32_le(&input[12..]));
            input = &input[16..];
        }

        // Store remaining data in buffer
        if !input.is_empty() {
            self.mem32[..input.len()].copy_from_slice(input);
            self.memsize = input.len();
        }

        Ok(())
    }

    /// Finalize and get hash
    pub fn digest(&self) -> XXH32Hash {
        let mut h = if self.large_len {
            rotl32(self.v[0], 1)
                .wrapping_add(rotl32(self.v[1], 7))
                .wrapping_add(rotl32(self.v[2], 12))
                .wrapping_add(rotl32(self.v[3], 18))
        } else {
            self.seed.wrapping_add(XXH32_PRIME5)
        };

        h = h.wrapping_add(self.total_len as u32);

        // Process remaining bytes
        let mut remaining = &self.mem32[..self.memsize];
        
        // Process 4-byte chunks
        while remaining.len() >= 4 {
            h = h.wrapping_add(read_u32_le(remaining).wrapping_mul(XXH32_PRIME3));
            h = rotl32(h, 17).wrapping_mul(XXH32_PRIME4);
            remaining = &remaining[4..];
        }

        // Process remaining bytes
        for &byte in remaining {
            h = h.wrapping_add((byte as u32).wrapping_mul(XXH32_PRIME5));
            h = rotl32(h, 11).wrapping_mul(XXH32_PRIME1);
        }

        // Avalanche
        h ^= h >> 15;
        h = h.wrapping_mul(XXH32_PRIME2);
        h ^= h >> 13;
        h = h.wrapping_mul(XXH32_PRIME3);
        h ^= h >> 16;

        h
    }

    fn consume_buffer(&mut self) {
        self.v[0] = xxh32_round(self.v[0], read_u32_le(&self.mem32[0..]));
        self.v[1] = xxh32_round(self.v[1], read_u32_le(&self.mem32[4..]));
        self.v[2] = xxh32_round(self.v[2], read_u32_le(&self.mem32[8..]));
        self.v[3] = xxh32_round(self.v[3], read_u32_le(&self.mem32[12..]));
    }
}

#[inline]
fn xxh32_round(acc: u32, input: u32) -> u32 {
    let acc = acc.wrapping_add(input.wrapping_mul(XXH32_PRIME2));
    rotl32(acc, 13).wrapping_mul(XXH32_PRIME1)
}

/// Compute XXH32 hash with seed 0
pub fn xxh32(data: &[u8]) -> XXH32Hash {
    xxh32_with_seed(data, 0)
}

/// Compute XXH32 hash with specified seed
pub fn xxh32_with_seed(data: &[u8], seed: u32) -> XXH32Hash {
    if data.len() < 16 {
        xxh32_finalize(seed.wrapping_add(XXH32_PRIME5), data, data.len() as u64)
    } else {
        let mut v = [
            seed.wrapping_add(XXH32_PRIME1).wrapping_add(XXH32_PRIME2),
            seed.wrapping_add(XXH32_PRIME2),
            seed,
            seed.wrapping_sub(XXH32_PRIME1),
        ];

        let mut input = data;
        while input.len() >= 16 {
            v[0] = xxh32_round(v[0], read_u32_le(&input[0..]));
            v[1] = xxh32_round(v[1], read_u32_le(&input[4..]));
            v[2] = xxh32_round(v[2], read_u32_le(&input[8..]));
            v[3] = xxh32_round(v[3], read_u32_le(&input[12..]));
            input = &input[16..];
        }

        let h = rotl32(v[0], 1)
            .wrapping_add(rotl32(v[1], 7))
            .wrapping_add(rotl32(v[2], 12))
            .wrapping_add(rotl32(v[3], 18));

        xxh32_finalize(h, input, data.len() as u64)
    }
}

#[inline]
fn xxh32_finalize(mut h: u32, remaining: &[u8], total_len: u64) -> u32 {
    h = h.wrapping_add(total_len as u32);

    let mut input = remaining;
    
    // Process 4-byte chunks
    while input.len() >= 4 {
        h = h.wrapping_add(read_u32_le(input).wrapping_mul(XXH32_PRIME3));
        h = rotl32(h, 17).wrapping_mul(XXH32_PRIME4);
        input = &input[4..];
    }

    // Process remaining bytes
    for &byte in input {
        h = h.wrapping_add((byte as u32).wrapping_mul(XXH32_PRIME5));
        h = rotl32(h, 11).wrapping_mul(XXH32_PRIME1);
    }

    // Avalanche
    h ^= h >> 15;
    h = h.wrapping_mul(XXH32_PRIME2);
    h ^= h >> 13;
    h = h.wrapping_mul(XXH32_PRIME3);
    h ^= h >> 16;

    h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xxh32_empty() {
        assert_eq!(xxh32(b""), 0x02cc5d05);
        assert_eq!(xxh32_with_seed(b"", 0x12345678), 0xbd209070);
    }

    #[test]
    fn test_xxh32_single_byte() {
        assert_eq!(xxh32(b"a"), 0x550d7456);
        assert_eq!(xxh32_with_seed(b"a", 0x12345678), 0x0d5a8e75);
    }

    #[test]
    fn test_xxh32_streaming() {
        let mut state = XXH32State::new(0);
        state.update(b"hell").unwrap();
        state.update(b"o world").unwrap();
        assert_eq!(state.digest(), xxh32(b"hello world"));
    }
}