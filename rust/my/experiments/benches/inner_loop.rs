use divan::{black_box, counter::ItemsCount, Bencher};

const BIG: usize = 100;
const SMALL: usize = 10;

// Version A: inner loop (10 iterations) is started 100 times
#[divan::bench]
fn a_100_outer_10_inner(bencher: Bencher) {
    bencher
        .counter(ItemsCount::new(BIG * SMALL))
        .bench(|| {
            let mut cnt = 0usize;
            for _ in 0..black_box(BIG) {
                for _ in 0..black_box(SMALL) {
                    cnt = black_box(cnt + 1);
                }
            }
            cnt
        });
}

/*
Comparisons in A:
  Inner "continue": 100 * 10  = 1000
  Inner "stop":     100 * 1   = 100   (one per inner-loop start)
  Outer "continue": 100
  Outer "stop":     1
  Total: 1201
*/

// Version B: inner loop (100 iterations) is started 10 times
#[divan::bench]
fn b_10_outer_100_inner(bencher: Bencher) {
    bencher
        .counter(ItemsCount::new(BIG * SMALL))
        .bench(|| {
            let mut cnt = 0usize;
            for _ in 0..black_box(SMALL) {
                for _ in 0..black_box(BIG) {
                    cnt = black_box(cnt + 1);
                }
            }
            cnt
        });
}

/*
Comparisons in B:
  Inner "continue": 10 * 100 = 1000
  Inner "stop":     10 * 1   = 10
  Outer "continue": 10
  Outer "stop":     1
  Total: 1021
*/

// A loop with k iterations does k + 1 comparisons: k "continue" and one "stop".
// Body executions are the same (1000) in both versions.
//
// A does 180 more comparisons than B (1201 vs 1021):
//   - 90 more inner "stop" checks (100 inner-loop starts vs 10),
//     the ones a branch predictor can mispredict;
//   - 90 more outer "continue" checks (100 outer iterations vs 10),
//     which are predictable.
//
// Big-O is the same, O(a * b); only the constant factor differs.
