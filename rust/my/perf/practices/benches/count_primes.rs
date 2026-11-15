use my_perf_practices::count_primes::*;

fn main() {
    divan::main();
}

const SIZES: &[i32] = &[1_000, 10_000, 1_000_000, 100_000_000];

#[divan::bench(args = &SIZES[0..2])] // would take too long
fn trial_division(n: i32) -> i32 {
    trial_division::Solution::count_primes(divan::black_box(n))
}

#[divan::bench(args = SIZES)]
fn sieve(n: i32) -> i32 {
    sieve::Solution::count_primes(divan::black_box(n))
}

#[divan::bench(args = SIZES)]
fn sieve_v2(n: i32) -> i32 {
    sieve_v2::Solution::count_primes(divan::black_box(n))
}

/*

count_primes       fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ sieve                         │               │               │               │         │
│  ├─ 1000         1.082 µs      │ 19.87 µs      │ 1.124 µs      │ 1.36 µs       │ 100     │ 100
│  ├─ 10000        14.7 µs       │ 19.99 µs      │ 15.49 µs      │ 15.45 µs      │ 100     │ 100
│  ├─ 1000000      1.334 ms      │ 2.844 ms      │ 1.356 ms      │ 1.432 ms      │ 100     │ 100
│  ╰─ 100000000    321.8 ms      │ 365 ms        │ 338.5 ms      │ 339.2 ms      │ 100     │ 100
├─ sieve_v2                      │               │               │               │         │
│  ├─ 1000         1.666 µs      │ 8.249 µs      │ 1.979 µs      │ 2.175 µs      │ 100     │ 200
│  ├─ 10000        22.29 µs      │ 33.08 µs      │ 24.87 µs      │ 24.8 µs       │ 100     │ 100
│  ├─ 1000000      2.334 ms      │ 3.116 ms      │ 2.437 ms      │ 2.473 ms      │ 100     │ 100
│  ╰─ 100000000    251.1 ms      │ 276.3 ms      │ 256.1 ms      │ 258.4 ms      │ 100     │ 100
╰─ trial_division                │               │               │               │         │
   ├─ 1000         35.83 µs      │ 51.37 µs      │ 35.91 µs      │ 37.41 µs      │ 100     │ 100
   ╰─ 10000        2.578 ms      │ 3.059 ms      │ 2.681 ms      │ 2.7 ms        │ 100     │ 100

*/
