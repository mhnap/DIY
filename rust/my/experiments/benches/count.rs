//! Latency of u64 add, sub, mul and div on a dependent chain.
//!
//! Each step uses the result of the previous one, so the CPU cannot overlap
//! steps. Every benchmark does N steps per call, and `ItemsCount` makes
//! divan print steps per second.
//!
//! Design choices:
//!
//! - The loops are in `#[inline(never)]` functions, so `cargo asm` shows
//!   only our loop and not divan's code.
//! - No `black_box` inside the loops. It forces the value through memory
//!   on every step and inflates the time. `black_box` is used only on the
//!   inputs and on the final result.
//! - Plain `x + k` or `x * k` repeated N times can be folded or split into
//!   independent chains by LLVM. So add, sub and mul use a second operation
//!   (`^ c` or `+ c`) that prevents this.
//! - Overflow is expected and harmless in release mode (checks are off).
//!   Do not run in debug mode.
//!
//! Run `cargo bench -- --sample-count 1000` and compare the fastest column.
//! Noise only makes runs slower, never faster.
//!
//! Check the assembly before trusting new numbers:
//! `cargo asm --bench count -p my_experiments count::mul_chain`
//! The loop must contain one chain and no `ldr` on the dependency path.
//!
//! Results are at the bottom of the file.

use divan::{black_box, counter::ItemsCount, Bencher};

const N: u64 = 1_000_000;

// add + eor per step. The xor stops LLVM from folding the loop into x + N*k.
#[inline(never)]
fn add_chain() -> u64 {
    let mut x = black_box(12345u64);
    let k = black_box(7u64);
    let c = black_box(1u64);
    for _ in 0..N {
        x = (x + k) ^ c;
    }
    black_box(x)
}

// sub + eor per step, same idea as add.
#[inline(never)]
fn sub_chain() -> u64 {
    let mut x = black_box(12345u64);
    let k = black_box(7u64);
    let c = black_box(1u64);
    for _ in 0..N {
        x = (x - k) ^ c;
    }
    black_box(x)
}

// One madd (multiply and add) per step. The + c stops LLVM from splitting
// the loop into independent chains. k = 3 is not 0, 1 or a power of 2.
#[inline(never)]
fn mul_chain() -> u64 {
    let mut x = black_box(12345u64);
    let k = black_box(3u64);
    let c = black_box(1u64);
    for _ in 0..N {
        x = x * k + c;
    }
    black_box(x)
}

// One udiv per step. x alternates between 3 and u64::MAX / 3, so it never
// becomes 0 and no extra operation is needed.
#[inline(never)]
fn div_chain() -> u64 {
    let mut x = black_box(3u64);
    let m = black_box(u64::MAX);
    for _ in 0..N {
        x = m / x;
    }
    black_box(x)
}

#[divan::bench]
fn add(bencher: Bencher) {
    bencher.counter(ItemsCount::new(N)).bench_local(add_chain);
}

#[divan::bench]
fn sub(bencher: Bencher) {
    bencher.counter(ItemsCount::new(N)).bench_local(sub_chain);
}

#[divan::bench]
fn mul(bencher: Bencher) {
    bencher.counter(ItemsCount::new(N)).bench_local(mul_chain);
}

#[divan::bench]
fn div(bencher: Bencher) {
    bencher.counter(ItemsCount::new(N)).bench_local(div_chain);
}

// ---------------------------------------------------------------------
// Results (fastest column, per single dependent step)
// Machine: Apple M4 Pro, about 4.5 GHz, so 1 cycle = 0.2215 ns.
//
//   op  | what one step does | time per step | steps per second | cycles
//   ----|--------------------|---------------|------------------|-------
//   add | add + eor          | 0.443 ns      | 2.256 billion    | 2
//   sub | sub + eor          | 0.443 ns      | 2.256 billion    | 2
//   mul | madd               | 0.886 ns      | 1.128 billion    | 4
//   div | udiv               | 1.994 ns      | 501 million      | 9
//
// Single add or sub (eor removed, each is 1 cycle): about 0.22 ns.
//
// Conclusions
//
//   - add and sub cost the same, 1 cycle each.
//   - mul (madd) is about 4 times slower than add.
//   - div is about 9 times slower than add, and about 2.25 times slower
//     than mul.
//
// Notes
//
//   - madd is a multiply and an add in one instruction. A pure mul is
//     probably about 3 cycles (published value, not measured here).
//   - div time is an average of two cases, because x alternates between 3
//     and u64::MAX / 3. On some CPUs division time depends on the values.
//   - Numbers are from one machine. Absolute values differ on other CPUs,
//     and division in particular varies a lot between CPU families.
// ---------------------------------------------------------------------
