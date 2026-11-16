use my_perf_practices::moving_average::*;

fn main() {
    divan::main();
}

const WINDOWS: &[i32] = &[100, 1_000, 10_000];
const CALLS: i32 = 10_000;

macro_rules! bench_impl {
    ($($module:ident),+ $(,)?) => {
        $(
            #[divan::bench(args = WINDOWS)]
            fn $module(bencher: divan::Bencher, size: i32) {
                bencher
                    .with_inputs(|| {
                        let mut m = $module::MovingAverage::new(size);
                        for _ in 0..size {
                            m.next(1);
                        }
                        m
                    })
                    .bench_local_values(|mut m| {
                        let mut last = 0.0;
                        for i in 0..CALLS {
                            last = m.next(divan::black_box(i & 0xFF));
                        }
                        last
                    });
            }
        )+
    };
}

bench_impl!(
    naive,
    deque,
    circular_queue,
    circular_queue_v2,
    circular_queue_v3
);

/*

moving_average        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ circular_queue                   │               │               │               │         │
│  ├─ 100             26.66 µs      │ 34.2 µs       │ 30.74 µs      │ 29.43 µs      │ 100     │ 100
│  ├─ 1000            27.24 µs      │ 33.87 µs      │ 30.74 µs      │ 30.31 µs      │ 100     │ 100
│  ╰─ 10000           26.62 µs      │ 44.12 µs      │ 30.64 µs      │ 29.55 µs      │ 100     │ 100
├─ circular_queue_v2                │               │               │               │         │
│  ├─ 100             7.749 µs      │ 9.999 µs      │ 7.832 µs      │ 7.921 µs      │ 100     │ 100
│  ├─ 1000            7.79 µs       │ 14.16 µs      │ 7.833 µs      │ 8.051 µs      │ 100     │ 100
│  ╰─ 10000           6.874 µs      │ 10.08 µs      │ 7.624 µs      │ 7.589 µs      │ 100     │ 100
├─ circular_queue_v3                │               │               │               │         │
│  ├─ 100             7.624 µs      │ 8.082 µs      │ 7.666 µs      │ 7.713 µs      │ 100     │ 100
│  ├─ 1000            7.624 µs      │ 8.166 µs      │ 7.666 µs      │ 7.703 µs      │ 100     │ 100
│  ╰─ 10000           6.665 µs      │ 7.874 µs      │ 7.499 µs      │ 7.344 µs      │ 100     │ 100
├─ deque                            │               │               │               │         │
│  ├─ 100             25.04 µs      │ 28.95 µs      │ 27.99 µs      │ 27.55 µs      │ 100     │ 100
│  ├─ 1000            25.08 µs      │ 42.16 µs      │ 28.79 µs      │ 28.97 µs      │ 100     │ 100
│  ╰─ 10000           28.95 µs      │ 33.37 µs      │ 29.08 µs      │ 29.16 µs      │ 100     │ 100
╰─ naive                            │               │               │               │         │
   ├─ 100             64.37 µs      │ 173 µs        │ 74.91 µs      │ 75.07 µs      │ 100     │ 100
   ├─ 1000            346.4 µs      │ 458.6 µs      │ 393.9 µs      │ 393.3 µs      │ 100     │ 100
   ╰─ 10000           2.846 ms      │ 3.289 ms      │ 3.196 ms      │ 3.169 ms      │ 100     │ 100

*/
