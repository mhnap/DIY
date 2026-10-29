//! Cache effect of element size.
//!
//! Same number of elements (65,536) and the same dependent random lookups
//! for every type; only the bytes per element change. Each lookup depends
//! on the previous one, so loads cannot overlap and the prefetcher cannot
//! help. Time per lookup therefore shows where the array stops fitting in
//! a cache level.

use divan::{black_box, counter::ItemsCount, Bencher};

// Array size in bytes: u8 = 64 KB, u16 = 128 KB, u32 = 256 KB,
// u64 = 512 KB, u128 = 1 MB.
const N: usize = 1 << 16;
const LOOKUPS: usize = 1 << 16;

#[divan::bench(types = [u8, u16, u32, u64, u128])]
fn random_lookups<T>(bencher: Bencher)
where
    T: Copy + TryFrom<u64> + Into<u128>,
{
    let data: Vec<T> = (0..N)
        .map(|i| T::try_from((i % 251) as u64).ok().unwrap())
        .collect();
    let mask = N - 1;

    bencher
        .counter(ItemsCount::new(LOOKUPS))
        .bench_local(|| {
            let data = black_box(&data);
            let mut state: u64 = 12345;
            let mut i = 0usize;
            for _ in 0..LOOKUPS {
                let x: u128 = data[i].into();
                // The next index depends on the loaded value.
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407 + x as u64);
                i = ((state >> 32) as usize) & mask;
            }
            state
        });
}

/*
Results (Apple M4, `cargo bench`, median of 1000 samples, 65,536 lookups):

  type   array    median     per lookup   vs u8
  u8     64 KB    122.9 us   1.9 ns       1.0x
  u16    128 KB   123.9 us   1.9 ns       1.0x
  u32    256 KB   293.2 us   4.5 ns       2.4x
  u64    512 KB   376.5 us   5.7 ns       3.1x
  u128   1 MB     446.4 us   6.8 ns       3.6x

Time follows the array size in bytes, not the element count: flat while the
array fits in L1D (about 128 KB), a large step at 256 KB when it does not,
then a slower rise while it is served from L2.

Caveats: this shows the effect through time only, not miss counts (use
Instruments / perf for L1D misses). The L1 size is an estimate for the M4
performance cores. The small u64 -> u128 step may include TLB effects.
Max values in the raw output are noise; compare medians.
*/
