use my_perf_practices::order_format::{baseline, v2a, v2b, v3};

// Each implementation is wrapped as: write `prefix`, then the value.
// Before the real work it writes u32::MAX and resets, so stale bytes
// left in the buffer must not leak into the output.
type Run = fn(&str, u32) -> Vec<u8>;

macro_rules! impl_run {
    ($m:ident) => {
        |prefix: &str, value: u32| {
            let mut f = $m::Format::new();
            f.decimal_append_non_neg(u32::MAX);
            f.reset();
            f.append_str(prefix);
            f.decimal_append_non_neg(value);
            f.data().to_vec()
        }
    };
}

const IMPLS: &[(&str, Run)] = &[
    ("baseline", impl_run!(baseline)),
    ("v2a", impl_run!(v2a)),
    ("v2b", impl_run!(v2b)),
    ("v3", impl_run!(v3)),
];

// Core values: zero, digit-count boundaries (n and n+1 around powers of ten),
// i32::MAX, u32::MAX
const VALUES: [u32; 16] = [
    0,
    1,
    9,
    10,
    99,
    100,
    999,
    1_000,
    99_999,
    100_000,
    999_999_999,
    1_000_000_000,
    1_234_567_890,
    i32::MAX as u32,
    u32::MAX - 1,
    u32::MAX,
];

// Empty prefix, and a typical one so digits start at a non-zero position
const PREFIXES: [&str; 2] = ["", "NEW AAPL "];

#[test]
fn matches_to_string() {
    for (name, run) in IMPLS {
        for prefix in PREFIXES {
            for v in VALUES {
                let expected = format!("{prefix}{v}").into_bytes();
                assert_eq!(
                    run(prefix, v),
                    expected,
                    "{name}: prefix {prefix:?}, value {v}"
                );
            }
        }
    }
}

#[test]
fn small_values_exhaustive() {
    for (name, run) in IMPLS {
        for v in 0..10_000u32 {
            assert_eq!(run("", v), v.to_string().into_bytes(), "{name}: value {v}");
        }
    }
}

#[test]
fn around_every_power() {
    for (name, run) in IMPLS {
        for i in 0..=9u32 {
            let p = 10u32.pow(i);
            for v in [p - 1, p, p + 1] {
                assert_eq!(run("", v), v.to_string().into_bytes(), "{name}: value {v}");
            }
        }
        for i in 0..32u32 {
            let p = 1u32 << i;
            for v in [p - 1, p, p.saturating_add(1)] {
                assert_eq!(
                    run("NEW ", v),
                    format!("NEW {v}").into_bytes(),
                    "{name}: value {v}"
                );
            }
        }
    }
}
