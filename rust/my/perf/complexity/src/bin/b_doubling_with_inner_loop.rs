// (b) see notes/b_doubling_with_inner_loop.jpg
// for (int i = 1; i < n; i = i + i)
//     for (int j = 0; j < i; j++)
//         api();
//
// i = 1   2^0   j = 0
// i = 2   2^1   j = 0  j = 1
// i = 4   2^2   j = 0  j = 1  j = 2  j = 3
// i = 8   2^3   0 1 2 3 4 5 6 7
// ...     ...
// k) i < n   2^k < n
//            log2(2^k) < log2(n)
//            k < log n
//
// 2^0 + 2^1 + 2^2 + ... + 2^k = (2^k - 1) + 2^k
// \_________ k+1 _________/   = 2*2^k - 1
//
// 2*2^k < 2*n  ->  2*2^k - 1 < 2n - 1
//
// O(n)

// Returns number of api() calls
fn b(n: u64) -> u64 {
    let mut calls = 0;
    let mut i = 1;
    while i < n {
        for _ in 0..i {
            calls += 1; // api();
        }
        i += i;
    }
    calls
}

fn main() {
    println!("(b) n = 9: calls = {}", b(9)); // < 2n - 1 = 2*9 - 1 = 17
}
