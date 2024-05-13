// https://doc.rust-lang.org/nightly/unstable-book/library-features/test.html

#![feature(test)]

extern crate test;

const SIZE: usize = 1_000_000;
const STR: &'static str = "hello";

fn main() {
    compare_vec();
}

fn compare_vec() {
    from_empty_vec();
    with_capacity_vec();
    from_elem_vec();
    from_iter_vec();
}

fn from_empty_vec() -> Vec<&'static str> {
    let mut v = vec![];
    for _ in 0..SIZE {
        v.push(STR);
    }
    v
}

fn with_capacity_vec() -> Vec<&'static str> {
    let mut v = Vec::with_capacity(SIZE);
    for _ in 0..SIZE {
        v.push(STR);
    }
    v
}

fn from_elem_vec() -> Vec<&'static str> {
    let v = vec![STR; SIZE];
    v
}

fn from_iter_vec() -> Vec<&'static str> {
    let v = Vec::from_iter(std::iter::repeat(STR).take(SIZE));
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_from_empty_vec() {
        let v = from_empty_vec();
        assert_eq!(v.len(), SIZE);
        assert_eq!(v.first(), Some(&STR));
        assert_eq!(v.last(), Some(&STR));
    }

    #[test]
    fn test_with_capacity_vec() {
        let v = with_capacity_vec();
        assert_eq!(v.len(), SIZE);
        assert_eq!(v.first(), Some(&STR));
        assert_eq!(v.last(), Some(&STR));
    }

    #[test]
    fn test_from_elem_vec() {
        let v = from_elem_vec();
        assert_eq!(v.len(), SIZE);
        assert_eq!(v.first(), Some(&STR));
        assert_eq!(v.last(), Some(&STR));
    }

    #[test]
    fn test_from_iter_vec() {
        let v = from_iter_vec();
        assert_eq!(v.len(), SIZE);
        assert_eq!(v.first(), Some(&STR));
        assert_eq!(v.last(), Some(&STR));
    }

    #[bench]
    fn bench_from_empty_vec(b: &mut Bencher) {
        b.iter(|| from_empty_vec());
    }

    #[bench]
    fn bench_with_capacity_vec(b: &mut Bencher) {
        b.iter(|| with_capacity_vec());
    }

    #[bench]
    fn bench_from_elem_vec(b: &mut Bencher) {
        b.iter(|| from_elem_vec());
    }

    #[bench]
    fn bench_from_iter_vec(b: &mut Bencher) {
        b.iter(|| from_iter_vec());
    }

    // This will be optimized.
    #[bench]
    fn bench_xor_ints_opt(b: &mut Bencher) {
        b.iter(|| {
            (0..SIZE).fold(0, |old, new| old ^ new);
        });
    }

    // This will not be optimized.
    #[bench]
    fn bench_xor_ints(b: &mut Bencher) {
        b.iter(|| (0..SIZE).fold(0, |old, new| old ^ new));
    }

    // This will not be optimized.
    #[bench]
    fn bench_xor_ints_black_box(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(SIZE);
            (0..n).fold(0, |old, new| old ^ new)
        })
    }
}
