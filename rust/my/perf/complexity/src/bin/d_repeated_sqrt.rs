// (d) see notes/d_repeated_sqrt.jpg
// void f(int n) {
//     while (n > 2) {
//         n = (int) Math.sqrt(n);
//         api();
//     }
// }
//
// (n^(1/2))^(1/2) = n^(1/2 * 1/2) = n^(1/4) = n^(1/2^k)
//
// n = 256:  1) 16  2) 4  3) 2
//
// we can have i = n at the first line, so i from n to 2
//
// for (int i = n; i > 2; i = sqrt(i))
//     api();
//
// 2^k-th root of n = n^(1/2^k)
//
// stops when n^(1/2^k) = 2               | ^(2^k)
//            (n^(1/2^k))^(2^k) = 2^(2^k)
//            n^(1/2^k * 2^k) = 2^(2^k)   | log2
//            log2(n) = 2^k
//            log2(log2(n)) = k
//
// O(log(log n))

// Returns number of api() calls
fn d(mut n: u64) -> u32 {
    let mut calls = 0;
    while n > 2 {
        n = n.isqrt();
        calls += 1; // api();
    }
    calls
}

fn main() {
    println!("(d) n = 256: calls = {}", d(256)); // log2(log2(n)) = log2(log2(256)) = log2(8) = 3
}
