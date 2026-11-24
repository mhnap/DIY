use my_perf_practices::order_format::{baseline, v2a, v2b, v3};

fn main() {
    divan::main();
}

// 1, 5 and 10 digits
const VALUES: [u32; 3] = [7, 12_345, u32::MAX];

// Conversions per run. Divide the reported time by this for one conversion
const REPEATS: usize = 100;

macro_rules! bench_impl {
    ($($module:ident),+ $(,)?) => {
        $(
            #[divan::bench(args = VALUES)]
            fn $module(bencher: divan::Bencher, value: u32) {
                let mut f = $module::Format::new();
                bencher.bench_local(|| {
                    for _ in 0..REPEATS {
                        f.reset();
                        f.decimal_append_non_neg(divan::black_box(value));
                        divan::black_box(f.data());
                    }
                });
            }
        )+
    };
}

bench_impl!(baseline, v2a, v2b, v3);

/*

order_format      fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ baseline                     │               │               │               │         │
│  ├─ 7           82.77 ns      │ 4.582 µs      │ 83.77 ns      │ 144.7 ns      │ 100     │ 100
│  ├─ 12345       299.2 ns      │ 447.7 ns      │ 307 ns        │ 325.4 ns      │ 100     │ 1600
│  ╰─ 4294967295  609.1 ns      │ 880 ns        │ 682 ns        │ 686.7 ns      │ 100     │ 800
├─ v2a                          │               │               │               │         │
│  ├─ 7           158.6 ns      │ 228.9 ns      │ 171.6 ns      │ 171.9 ns      │ 100     │ 3200
│  ├─ 12345       249.7 ns      │ 893 ns        │ 249.7 ns      │ 272.3 ns      │ 100     │ 1600
│  ╰─ 4294967295  426.7 ns      │ 624.7 ns      │ 478.9 ns      │ 493.4 ns      │ 100     │ 800
├─ v2b                          │               │               │               │         │
│  ├─ 7           158.6 ns      │ 227.6 ns      │ 178.1 ns      │ 176.1 ns      │ 100     │ 3200
│  ├─ 12345       247.1 ns      │ 361.7 ns      │ 249.7 ns      │ 268.3 ns      │ 100     │ 1600
│  ╰─ 4294967295  400.8 ns      │ 585.7 ns      │ 450.2 ns      │ 444.2 ns      │ 100     │ 1600
╰─ v3                           │               │               │               │         │
   ├─ 7           179.4 ns      │ 249.7 ns      │ 201.5 ns      │ 196.2 ns      │ 100     │ 3200
   ├─ 12345       201.5 ns      │ 300.5 ns      │ 227.6 ns      │ 222 ns        │ 100     │ 3200
   ╰─ 4294967295  369.5 ns      │ 2.697 µs      │ 419 ns        │ 492.1 ns      │ 100     │ 1600

*/
