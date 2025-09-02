// Bit operations for i32 with standard shifting.
//
// Conventions:
// - Bit positions use i32: 0 = LSB, 31 = MSB
// - Right shift (>>) on i32 is arithmetic (sign-extending).
// - We avoid signed overflows in helpers (notably lowbit / clear_lowest_set_bit).

/// 1<<k mask (0..31). Standard shift (overshift may panic in debug).
fn bit(k: i32) -> i32 {
    1 << k
}

/// Test if k-th bit is 1.
fn test_bit(x: i32, k: i32) -> bool {
    (x & bit(k)) != 0
}

/// Set k-th bit to 1.
fn set_bit(x: i32, k: i32) -> i32 {
    x | bit(k)
}

/// Clear k-th bit to 0.
fn clear_bit(x: i32, k: i32) -> i32 {
    x & !bit(k)
}

/// Toggle k-th bit.
fn toggle_bit(x: i32, k: i32) -> i32 {
    x ^ bit(k)
}

/// Get k-th bit as 0/1.
fn get_bit(x: i32, k: i32) -> i32 {
    (x >> k) & 1
}

/// Multiply by 2^k (left shift).
fn mul_pow2(x: i32, k: i32) -> i32 {
    x << k
}

/// Divide by 2^k.
/// NOTE: `x >> k` rounds toward -∞ for negatives (differs from `/` which rounds toward 0).
fn div_pow2(x: i32, k: i32) -> i32 {
    x >> k
}

/// Returns the lowest set bit of `x` (isolates the least-significant `1`).
/// Two’s complement identity: `x & -x`. Special-case MIN to avoid `-x` overflow in debug.
fn lowbit(x: i32) -> i32 {
    if x == i32::MIN { i32::MIN } else { x & -x }
}

/// True iff x is a (positive) power of two.
fn is_power_of_two(x: i32) -> bool {
    x > 0 && lowbit(x) == x
}

/// XOR-swap two i32s.
fn swap_xor(a: &mut i32, b: &mut i32) {
    *a ^= *b;
    *b ^= *a;
    *a ^= *b;
}

/// Clear the lowest set bit of `x`.
/// Uses `x & (x ^ lowbit(x))` to avoid `x-1` overflow at i32::MIN.
fn clear_lowest_set_bit(x: i32) -> i32 {
    if x == i32::MIN { 0 } else { x & (x - 1) }
}

/// Count set bits (Hamming weight).
fn popcount(mut x: i32) -> i32 {
    let mut c: i32 = 0;
    while x != 0 {
        x = clear_lowest_set_bit(x);
        c += 1;
    }
    c
}

/// Reverse all 32 bits (bit i -> bit 31-i).
fn reverse_bits(mut x: i32) -> i32 {
    let mut r = 0;
    for _ in 0..32 {
        r = (r << 1) | (x & 1);
        x >>= 1;
    }
    r
}

/// Hamming distance between two i32 values (number of differing bit positions).
fn hamming_distance(a: i32, b: i32) -> i32 {
    popcount(a ^ b)
}

/// Add two i32 numbers without using `+` or `-`.
///
/// Works for negatives too, because two's complement makes bitwise
/// addition identical for signed and unsigned integers.
/// - `^` computes the partial sum (ignores carries).
/// - `&` finds carry positions, then `<< 1` shifts them left.
/// Repeat until carry is 0.
fn add_i32(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let sum = a ^ b; // sum without carry
        let carry = (a & b) << 1; // carry bits
        a = sum;
        b = carry;
    }
    a
}

/// 1-bit full adder: a,b,cin ∈ {0,1} → (sum, cout)
fn full_adder_bit(a: i32, b: i32, cin: i32) -> (i32, i32) {
    let sum = (a ^ b ^ cin) & 1;
    let cout = ((a & b) | (cin & (a ^ b))) & 1;
    (sum, cout)
}

/// 32-bit ripple-carry adder built from full_adder_bit. Returns (sum, carry_out).
fn add_i32_bitwise(x: i32, y: i32) -> (i32, i32) {
    let mut s: i32 = 0;
    let mut c: i32 = 0;
    for k in 0..32 {
        let a = get_bit(x, k);
        let b = get_bit(y, k);
        let (bit, carry) = full_adder_bit(a, b, c);
        s |= bit << k;
        c = carry;
    }
    (s, c)
}

fn main() {
    let x: i32 = 0b0001_0100;
    assert!(test_bit(x, 2));
    assert!(test_bit(x, 4));
    assert!(!test_bit(x, 5));

    assert_eq!(set_bit(x, 0), 0b0001_0101);
    assert_eq!(set_bit(x, 2), x);
    assert_eq!(clear_bit(x, 4), 0b0000_0100);
    assert_eq!(clear_bit(x, 1), x);
    assert_eq!(toggle_bit(x, 2), 0b0001_0000);
    assert_eq!(toggle_bit(x, 3), 0b0001_1100);
    assert_eq!(get_bit(0b1010_1000, 3), 1);
    assert_eq!(get_bit(0b1010_1000, 2), 0);

    assert_eq!(mul_pow2(7, 0), 7);
    assert_eq!(mul_pow2(7, 1), 14);
    assert_eq!(mul_pow2(7, 2), 28);

    // NOTE: right shift rounds toward -∞ for negatives.
    assert_eq!(div_pow2(29, 0), 29);
    assert_eq!(div_pow2(29, 1), 14);
    assert_eq!(div_pow2(29, 2), 7);
    assert_eq!(div_pow2(-3, 1), -2); // differs from -3/2 (which is -1)
    assert_eq!(-3 / 2, -1);

    for v in [1, 2, 4, 8, 16, 32, 64, 128] {
        assert!(is_power_of_two(v));
    }
    for v in [0, 3, 5, 6, 7, 9, -1, i32::MIN] {
        assert!(!is_power_of_two(v));
    }

    assert_eq!(lowbit(0), 0);
    assert_eq!(lowbit(1), 1);
    assert_eq!(lowbit(0b0010_0000), 0b0010_0000);
    assert_eq!(lowbit(0b1011_0000), 0b0001_0000);
    assert_eq!(lowbit(i32::MIN), i32::MIN); // MSB isolated

    let (mut a, mut b) = (5, 12);
    swap_xor(&mut a, &mut b);
    assert_eq!((a, b), (12, 5));

    let (mut c, mut d) = (42, 42);
    swap_xor(&mut c, &mut d);
    assert_eq!((c, d), (42, 42));

    assert_eq!(clear_lowest_set_bit(0b1011_0000), 0b1010_0000);
    assert_eq!(clear_lowest_set_bit(0b0000_0001), 0b0000_0000);
    assert_eq!(clear_lowest_set_bit(i32::MIN), 0);

    assert_eq!(popcount(0b1010_1110), 5);
    assert_eq!(hamming_distance(0b1010_0001, 0b0011_0101), 3);

    assert_eq!(
        reverse_bits(0b0000_0000_0000_0000_0000_0000_0001_0010),
        0b0100_1000_0000_0000_0000_0000_0000_0000
    );
    assert_eq!(reverse_bits(18), 1_207_959_552);

    assert_eq!(add_i32(5, 7), 12);
    assert_eq!(add_i32(-5, 7), 2);
    assert_eq!(add_i32(-3, -4), -7);
    assert_eq!(add_i32(123, -123), 0);

    let (s, c_out) = add_i32_bitwise(25, 17);
    assert_eq!((s, c_out), (42, 0));

    let (s, c_out) = add_i32_bitwise(200, 100);
    assert_eq!((s, c_out), (300, 0));

    let (s, c_out) = add_i32_bitwise(i32::MAX, 1);
    assert_eq!((s, c_out), (i32::MIN, 0)); // correct: carry-out is 0

    let (s, c_out) = add_i32_bitwise(-1, 1);
    assert_eq!((s, c_out), (0, 1)); // example where carry-out is 1

    println!("ok ✅");
}
