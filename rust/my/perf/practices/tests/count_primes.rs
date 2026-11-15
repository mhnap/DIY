use my_perf_practices::count_primes::*;

type CountFn = fn(i32) -> i32;

const IMPLS: &[(&str, CountFn)] = &[
    ("trial_division", trial_division::Solution::count_primes),
    ("sieve", sieve::Solution::count_primes),
    ("sieve_v2", sieve_v2::Solution::count_primes),
];

// (n, expected number of primes strictly less than n)
const CASES: [(i32, i32); 22] = [
    // Edges: nothing below 2 is prime
    (0, 0),
    (1, 0),
    (2, 0), // strictly less than n, so 2 itself is excluded
    (3, 1), // just 2
    (4, 2),
    // Prime vs. composite boundary (n is prime, then n+1)
    (7, 3),
    (8, 4),
    (10, 4),
    (11, 4), // 11 itself excluded
    (12, 5),
    // Squares of primes (the i * i boundary in the optimized sieve)
    (25, 9),
    (26, 9),
    (49, 15),
    (50, 15),
    // Round numbers
    (100, 25),
    (1_000, 168),
    (10_000, 1_229),
    // Others
    (5, 2),
    (6, 3),
    (13, 5),
    (14, 6),
    (30, 10),
];

#[test]
fn all_implementations_match_known_counts() {
    for (name, count_primes) in IMPLS {
        for (n, expected) in CASES {
            assert_eq!(count_primes(n), expected, "{name}: count_primes({n}) should be {expected}");
        }
    }
}
