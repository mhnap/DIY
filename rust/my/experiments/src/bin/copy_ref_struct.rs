// First version with derive, won't compile.
// Because there would be trait bound for T type in expanded macro.
// #[automatically_derived]
// impl<'a, T: ::core::clone::Clone> ::core::clone::Clone for Ref<'a, T> {
//     #[inline]
//     fn clone(&self) -> Ref<'a, T> {
//         Ref {
//             r: ::core::clone::Clone::clone(&self.r),
//         }
//     }
// }
// #[automatically_derived]
// impl<'a, T: ::core::marker::Copy> ::core::marker::Copy for Ref<'a, T> {}
#[derive(Clone, Copy)]
struct Ref<'a, T> {
    r: &'a T,
}

// Second version with manual implementation, will compile.
// impl<'a, T> Clone for Ref<'a, T> {
//     fn clone(&self) -> Self {
//         *self
//     }
// }
//
// impl<'a, T> Copy for Ref<'a, T> {}

// Need to make Clone + Copy to make first version compile.
// #[derive(Clone, Copy)]

struct A(u8);

fn main() {
    let a = A(1);
    let r1 = Ref { r: &a };
    let r2 = r1;
    // println!("{}", r1.r.0);
    // println!("{}", r2.r.0);
    //
    // error[E0382]: borrow of moved value: `r1`
    //   --> my/experiments/src/bin/copy_ref_struct.rs:37:20
    //    |
    // 35 |     let r1 = Ref { r: &a };
    //    |         -- move occurs because `r1` has type `Ref<'_, A>`, which does not implement the `Copy` trait
    // 36 |     let r2 = r1;
    //    |              -- value moved here
    // 37 |     println!("{}", r1.r.0);
    //    |                    ^^^^^^ value borrowed here after move
    //    |
    // note: if `Ref<'_, A>` implemented `Clone`, you could clone the value
    //   --> my/experiments/src/bin/copy_ref_struct.rs:15:1
    //    |
    // 15 | struct Ref<'a, T> {
    //    | ^^^^^^^^^^^^^^^^^ consider implementing `Clone` for this type
    // ...
    // 36 |     let r2 = r1;
    //    |              -- you could clone this value
    //    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
}
