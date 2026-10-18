
const SIZE: usize = 1_000_000;
const STR: &str = "hello";

#[divan::bench]
fn from_empty_vec() -> Vec<&'static str> {
    let mut v = vec![];
    for _ in 0..SIZE {
        v.push(STR);
    }
    v
}

#[divan::bench]
fn with_capacity_vec() -> Vec<&'static str> {
    let mut v = Vec::with_capacity(SIZE);
    for _ in 0..SIZE {
        v.push(STR);
    }
    v
}

#[divan::bench]
fn from_elem_vec() -> Vec<&'static str> {
    vec![STR; SIZE]
}

#[divan::bench]
fn from_iter_vec() -> Vec<&'static str> {
    Vec::from_iter(std::iter::repeat_n(STR, SIZE))
}
