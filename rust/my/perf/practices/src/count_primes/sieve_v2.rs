// Sieve of Eratosthenes, v2 (bit-packed)
//
// Same as sieve, but flags are stored as bits, 8 per byte.
// Flag i lives in byte i / 8, at bit i % 8.
//
// Time: O(n log n), same as sieve. Each step does a bit more work
//       (find the byte, build the mask, read-modify-write).
//
// Space: O(n), but 8x smaller in practice (n / 8 bytes).
//
// Measured: slower for small n (extra work per flag), faster at
//           n = 100_000_000, where the smaller array means fewer waits on memory.

pub struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        let mut cnt: i32 = 0;

        // Store raw bytes with one bit per flag
        // Size would be smaller by byte size (8), but ceiling
        let mut sieve: Vec<u8> = vec![0; n.div_ceil(8)];

        for i in 2..n {
            let (quot, rem) = div_rem(i, 8);
            if !test_bit(sieve[quot] as i32, rem as i32) {
                cnt += 1;
                let mut j = i * i;
                while j < n {
                    let (quot, rem) = div_rem(j, 8);
                    sieve[quot] = set_bit(sieve[quot] as i32, rem as i32) as u8;
                    j += i;
                }
            }
        }

        cnt
    }
}

/// 1<<k mask (0..31). Standard shift (overshift may panic in debug).
#[inline]
fn bit(k: i32) -> i32 {
    1 << k
}

/// Test if k-th bit is 1.
#[inline]
fn test_bit(x: i32, k: i32) -> bool {
    (x & bit(k)) != 0
}

/// Set k-th bit to 1.
#[inline]
fn set_bit(x: i32, k: i32) -> i32 {
    x | bit(k)
}

#[inline]
fn div_rem<T: std::ops::Div<Output = T> + std::ops::Rem<Output = T> + Copy>(x: T, y: T) -> (T, T) {
    let quot = x / y;
    let rem = x % y;
    (quot, rem)
}

/*

[0000 0000, 00..]
[0123 4567, 89..]

8 / 8 = 1
2 / 8 = 0 - byte index
2 % 8 = 2 - bit index

*/
