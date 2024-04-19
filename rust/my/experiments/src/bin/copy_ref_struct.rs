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
    println!("{}", r1.r.0);
    println!("{}", r2.r.0);
}
