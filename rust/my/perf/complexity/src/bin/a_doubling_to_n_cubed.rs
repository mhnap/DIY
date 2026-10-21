// (a) see notes/a_doubling_to_n_cubed.jpg
// for (long i = 1; i < n * n * n; i *= 2)
//     api();
//
// k - number of iters
// 1) i = 1     2^0
// 2) i = 2     2^1
// 3) i = 4     2^2
// ...
// k) i < n^3   2^(k-1) < n^3
//
// log2(2^(k-1)) < log2(n^3)
// k - 1 < 3*log2(n)
// k < 3*log2(n) + 1
//
// O(log n)

// Returns number of iters (k)
fn a(n: u64) -> u32 {
    let mut k = 0;
    let mut i = 1;
    while i < n * n * n {
        k += 1; // api();
        i *= 2;
    }
    k
}

fn main() {
    println!("(a) n = 1000: k = {}", a(1000)); // 3*log2(1000) = 3*9.97 ≈ 30
}
