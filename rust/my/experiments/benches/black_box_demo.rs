use divan::black_box;

const SIZE: usize = 1_000_000;

// The result is thrown away, so the compiler may remove the work.
#[divan::bench]
fn xor_ints_opt() {
    let _ = (0..SIZE).fold(0, |old, new| old ^ new);
}

// Returning the result keeps the work.
#[divan::bench]
fn xor_ints() -> usize {
    (0..SIZE).fold(0, |old, new| old ^ new)
}

// SIZE is a constant, so the compiler could compute the answer at compile time.
// black_box hides the value, so the loop always runs.
#[divan::bench]
fn xor_ints_black_box() -> usize {
    (0..black_box(SIZE)).fold(0, |old, new| old ^ new)
}
