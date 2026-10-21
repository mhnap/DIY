// (c) see notes/c_recursion_split_by_4.jpg
// void f(int n) {
//     if (n == 0) return;
//     for (int i = 0; i < n; i++)
//         api();
//     for (int i = 0; i < 4; i++)
//         f(n / 4);
// }
//
//                16
//       /     /     \     \
//      4     4       4     4        } k = log4(n)
//    ////  ////    ////  ////
//    1111  1111    1111  1111
//    \_________ 16 _________/
//
// cost         size   number
// n/1 * 1      n/1    2^0   4^0
// n/4 * 4      n/4    2^2   4^1
// n/16 * 16    n/16   2^4   4^2
// n/n=1 * n    1      2^2k  4^k
// where k - length
//
// (n*1) + (n/4*4) + (n/16*16) + ... + (1*n)
//   =
// (n + n + n + ... + n + n) times k
//
// stops when n/4^k = 1   n = 4^k   log4(n) = k
//
// O(n*log n)

// Returns number of api() calls
fn c(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut calls = 0;
    for _ in 0..n {
        calls += 1; // api();
    }
    for _ in 0..4 {
        calls += c(n / 4);
    }
    calls
}

fn main() {
    println!("(c) n = 16: calls = {}", c(16)); // n*(log4(n) + 1) = 16*(2 + 1) = 48
}
