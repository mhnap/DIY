// https://www.youtube.com/watch?v=LKKmPAQFNgE

fn main() {
    #[derive(Debug)]
    struct V(Vec<V>);

    let mut v = V(vec![V(vec![])]);
    dbg!(&v);

    // std::mem::swap(&mut v, &mut v.0.pop().unwrap());
    //
    //     error[E0499]: cannot borrow `v.0` as mutable more than once at a time
    //     --> my/cpp_comparison/src/bin/favorite_memory_leak.rs:10:33
    //      |
    //   10 |     std::mem::swap(&mut v, &mut v.0.pop().unwrap());
    //      |     -------------- ------       ^^^ second mutable borrow occurs here
    //      |     |              |
    //      |     |              first mutable borrow occurs here
    //      |     first borrow later used by call
}
