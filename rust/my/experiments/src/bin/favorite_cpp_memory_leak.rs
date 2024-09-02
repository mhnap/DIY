// https://www.youtube.com/watch?v=LKKmPAQFNgE

fn main() {
    #[derive(Debug)]
    struct V(Vec<V>);

    let mut v = V(vec![V(vec![])]);
    dbg!(&v);

    std::mem::swap(&mut v, &mut v.0.pop().unwrap());
}
