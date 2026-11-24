use my_perf_practices::moving_average::*;

// Every implementation is wrapped as "make a stream from a window size".
// The returned closure takes the next value and gives the current average.
type Stream = Box<dyn FnMut(i32) -> f64>;
type Factory = fn(i32) -> Stream;

// To test a future implementation, add one line here
const IMPLS: &[(&str, Factory)] = &[
    ("naive", |size| {
        let mut m = naive::MovingAverage::new(size);
        Box::new(move |v| m.next(v))
    }),
    ("deque", |size| {
        let mut m = deque::MovingAverage::new(size);
        Box::new(move |v| m.next(v))
    }),
    ("circular_queue", |size| {
        let mut m = circular_queue::MovingAverage::new(size);
        Box::new(move |v| m.next(v))
    }),
    ("circular_queue_v2", |size| {
        let mut m = circular_queue_v2::MovingAverage::new(size);
        Box::new(move |v| m.next(v))
    }),
    ("circular_queue_v3", |size| {
        let mut m = circular_queue_v3::MovingAverage::new(size);
        Box::new(move |v| m.next(v))
    }),
];

// (window size, inputs, expected averages)
const CASES: [(i32, &[i32], &[f64]); 6] = [
    // LeetCode example
    (3, &[1, 10, 3, 5], &[1.0, 5.5, 14.0 / 3.0, 6.0]),
    // Window of 1: always the last value
    (1, &[4, -2, 7], &[4.0, -2.0, 7.0]),
    // Window never fills up: average of everything so far
    (5, &[2, 4, 6], &[2.0, 3.0, 4.0]),
    // Window fills exactly, then slides once
    (2, &[1, 3, 5], &[1.0, 2.0, 4.0]),
    // Negative values and zeros
    (3, &[-3, 0, 3, 0], &[-3.0, -1.5, 0.0, 1.0]),
    // Extreme values: 3 * 100_000, still fits in i32
    (
        3,
        &[100_000, 100_000, 100_000, -100_000],
        &[100_000.0, 100_000.0, 100_000.0, 100_000.0 / 3.0],
    ),
];

fn assert_close(actual: f64, expected: f64, msg: &str) {
    assert!(
        (actual - expected).abs() < 1e-9,
        "{msg}: got {actual}, expected {expected}"
    );
}

#[test]
fn all_implementations_match_known_cases() {
    for (name, make) in IMPLS {
        for (size, inputs, expected) in CASES {
            let mut stream = make(size);
            for (step, (&v, &e)) in inputs.iter().zip(expected).enumerate() {
                assert_close(stream(v), e, &format!("{name}: size {size}, step {step}"));
            }
        }
    }
}

// Slow but obviously correct: average of the last `size` values
fn naive(size: usize, seen: &[i32]) -> f64 {
    let window = &seen[seen.len().saturating_sub(size)..];
    window.iter().map(|&x| x as i64).sum::<i64>() as f64 / window.len() as f64
}

#[test]
fn all_implementations_match_naive_on_long_stream() {
    for (name, make) in IMPLS {
        for size in [1, 2, 3, 10, 1000] {
            let mut stream = make(size);
            let mut seen = Vec::new();
            let mut x: u32 = 12345; // tiny LCG, no extra dependency
            for step in 0..3000 {
                x = x.wrapping_mul(1_103_515_245).wrapping_add(12345);
                let v = (x % 200_001) as i32 - 100_000; // -100_000..=100_000
                seen.push(v);
                assert_close(
                    stream(v),
                    naive(size as usize, &seen),
                    &format!("{name}: size {size}, step {step}"),
                );
            }
        }
    }
}
