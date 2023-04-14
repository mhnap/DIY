// Cannot create global variable with let
// let PI = 3.14;
// error: expected item, found keyword `let`
//  --> src/lessons/constants.rs:1:1
//   |
// 1 | let PI = 3.14;
//   | ^^^ consider using `const` or `static` instead of `let` for global variables

// Constants always should have annotated type
// const PI = 3.14;
// error: missing type for `const` item
//  --> src/lessons/constants.rs:9:9
//   |
// 9 | const PI = 3.14;
//   |         ^ help: provide a type for the constant: `: f64`

// Constants can be declared in any scope, including the global scope
const PI: f32 = 3.14;

fn main() {
    dbg!(PI);

    const E: f32 = 2.71;
    dbg!(E);

    // Can calculate on compilation
    const TEN: u8 = 2 * 5;
    dbg!(TEN);

    fn get_twenty() -> u8 {
        4 * 5
    }
    // Constants may be set only to a constant expression, not the result of a value that could only be computed at runtime
    // const TWENTY: u8 = get_twenty();
    // error[E0015]: cannot call non-const fn `get_twenty` in constants
    //   --> src/lessons/constants.rs:34:24
    //    |
    // 34 |     const TWENTY: u8 = get_twenty();
    //    |                        ^^^^^^^^^^^^
    //    |
    //    = note: calls in constants are limited to constant functions, tuple structs and tuple variants
}

// https://users.rust-lang.org/t/why-is-type-declaration-necessary-in-constants/14200
// https://stackoverflow.com/questions/55531587/why-do-i-have-to-specify-the-type-for-const-variables-but-not-for-let-variab
// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/const-and-static.html
// https://users.rust-lang.org/t/const-vs-static/52951
