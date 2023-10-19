// https://doc.rust-lang.org/std/ops/trait.Drop.html#drop-check
// https://doc.rust-lang.org/nomicon/dropck.html
// https://doc.rust-lang.org/reference/destructors.html

use std::fmt::Display;

struct Ref<T> {
    r: T,
}

struct RefWithDrop<T: Display> {
    r: T,
}

impl<T: Display> Drop for RefWithDrop<T> {
    fn drop(&mut self) {
        println!("drop {}", self.r);
    }
}

fn main() {
    let mut n = 0;

    // Will not break borrowing rules.
    let r = &mut n;
    *r += 1;
    println!("{}", n);

    // Will not break borrowing rules.
    let b = Box::new(&mut n);
    **b += 1;
    println!("{}", n);

    // Will not break borrowing rules.
    let r = Ref { r: &mut n };
    *r.r += 1;
    println!("{}", n);

    // Will break borrowing rules.
    let r = RefWithDrop { r: &mut n };
    *r.r += 1;
    // println!("{}", n);
    // error[E0502]: cannot borrow `n` as immutable because it is also borrowed as mutable
    //   --> experiments/src/bin/drop_check.rs:41:20
    //    |
    // 39 |     let r = RefWithDrop { r: &mut n };
    //    |                              ------ mutable borrow occurs here
    // 40 |     *r.r += 1;
    // 41 |     println!("{}", n);
    //    |                    ^ immutable borrow occurs here
    // 42 | }
    //    | - mutable borrow might be used here, when `r` is dropped and runs the `Drop` code for type `RefWithDrop`
    //    |
    //    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
}
