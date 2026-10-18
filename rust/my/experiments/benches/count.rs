use divan::black_box;

#[divan::bench(args = [1_000, 1_000_000, 1_000_000_000])]
fn count(n: u64) -> u64 {
    let mut cnt = 0;
    for _ in 0..n {
        cnt += 1;
        black_box(());
    }
    cnt
}
