struct A(i8, u8);
struct B(i8, u8);

type C = (i8, u8);
type D = (i8, u8);

fn main() {
    let a = A(1, 2);
    let b = B(3, 4);

    // Nominal typing says that these are different types.
    // let c: B = a;
    // error[E0308]: mismatched types
    //  --> src/experiments/nominal_vs_structural_typing.rs:9:16
    //   |
    // 9 |     let c: B = a;
    //   |            -   ^ expected `B`, found `A`
    //   |            |
    //   |            expected due to this

    let c: C = (1, 2);
    let d: D = (3, 4);

    // Structural typing says that these are equivalent types.
    let e: D = c;
}
