use std::io;
use std::io::Write;

fn read_int() -> i32 {
    io::stdout().write(b"Write int: ");
    io::stdout().flush();

    let mut input_line = String::new();
    io::stdin() // the rough equivalent of `std::cin`
        .read_line(&mut input_line) // actually read the line
        .expect("Failed to read line"); // which can fail, however
    let x: i32 = input_line
        .trim() // ignore whitespace around input
        .parse() // convert to integers
        .expect("Input not an integer"); // which, again, can fail
    x
}

fn single_line_in_if_without_braces() {
    let i = 0;
    // if i == 0
    //     println!("single_line_in_if_without_braces");
    // error: expected `{`, found `println`
    //   --> src/main.rs:10:9
    //    |
    // 10 |         println!("assigment_in_if");
    //    |         ^^^^^^^ expected `{`
    //    |
    // note: the `if` expression is missing a block after this condition
    //   --> src/main.rs:9:8
    //    |
    // 9  |     if i == 0
    //    |        ^^^^^^
    // help: try placing this code inside a block
    //    |
    // 10 |         { println!("assigment_in_if"); }
    //    |         +                              +
    if i == 0 {
        println!("single_line_in_if_without_braces");
    }
}

fn assigment_in_if() {
    let i = 0;
    // if i = 0 {
    // error[E0308]: mismatched types
    //  --> src/main.rs:8:8
    //   |
    // 8 |     if i = 0 {
    //   |        ^^^^^ expected `bool`, found `()`
    //   |
    // help: you might have meant to compare for equality
    //   |
    // 8 |     if i == 0 {
    //   |           +
    if i == 0 {
        println!("assigment_in_if");
    }
}

fn unnecessary_parentheses_in_if() {
    let i = 0;
    // if (i == 0) {
    // warning: unnecessary parentheses around `if` condition
    //   --> src/main.rs:51:8
    //    |
    // 51 |     if (i == 0) {
    //    |        ^      ^
    //    |
    //    = note: `#[warn(unused_parens)]` on by default
    // help: remove these parentheses
    //    |
    // 51 -     if (i == 0) {
    // 51 +     if i == 0 {
    //    |
    if i == 0 {
        println!("unnecessary_parentheses_in_if");
    }
}

fn explicit_type_conversion() {
    let i = 0;
    let f = 0.5;
    // let i2 = i + f;
    // error[E0277]: cannot add a float to an integer
    //   --> src/main.rs:76:16
    //    |
    // 76 |     let i2 = i + d;
    //    |                ^ no implementation for `{integer} + {float}`
    //    |
    //    = help: the trait `Add<{float}>` is not implemented for `{integer}`
    //    = help: the following other types implement trait `Add<Rhs>`:
    //              <&'a f32 as Add<f32>>
    //              <&'a f64 as Add<f64>>
    //              <&'a i128 as Add<i128>>
    //              <&'a i16 as Add<i16>>
    //              <&'a i32 as Add<i32>>
    //              <&'a i64 as Add<i64>>
    //              <&'a i8 as Add<i8>>
    //              <&'a isize as Add<isize>>
    //            and 48 others
    let i2 = i + f as i32;
    let f2 = i as f32 + f;
    println!("explicit_type_conversion {i2} {f2}");
}

fn uninitialized_variable() {
    // let i: i32;
    // error[E0381]: used binding `i` isn't initialized
    //    --> src/main.rs:104:39
    //     |
    // 103 |     let i: i32;
    //     |         - binding declared here but left uninitialized
    // 104 |     println!("uninitialized_variable {i}");
    //     |                                       ^ `i` used here but it isn't initialized
    //     |
    //     = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
    // help: consider assigning a value
    //     |
    // 103 |     let i: i32 = 0;
    //     |                +++
    let i = 42;
    println!("uninitialized_variable {i}");
}

fn uninitialized_variable_with_condition() {
    let i;
    if read_int() != 0 {
        i = 1;
    }
    // println!("{i}");
    // error[E0381]: used binding `i` is possibly-uninitialized
    //    --> src/main.rs:138:16
    //     |
    // 134 |     let i;
    //     |         - binding declared here but left uninitialized
    // 135 |     if read_int() != 0 {
    //     |        --------------- if this `if` condition is `false`, `i` is not initialized
    // 136 |         i = 1;
    // 137 |     }
    //     |      - an `else` arm might be missing here, initializing `i`
    // 138 |     println!("{i}");
    //     |                ^ `i` used here but it is possibly-uninitialized
    //     |
    //     = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
    let i;
    if read_int() != 0 {
        i = 1;
    } else {
        i = 0;
    }
    println!("uninitialized_variable_with_condition {i}");
}

fn use_after_move() {
    print!("use_after_move: ");

    let a = 42;
    // Copy
    let b = a;
    print!("a {a} ");
    print!("b {b} ");

    let a = Box::new(42);
    // Move
    let b = a;
    // print!("a {a} ");
    // error[E0382]: borrow of moved value: `a`
    //    --> src/main.rs:144:16
    //     |
    // 141 |     let a = Box::new(42);
    //     |         - move occurs because `a` has type `Box<i32>`, which does not implement the `Copy` trait
    // 142 |     // Move
    // 143 |     let b = a;
    //     |             - value moved here
    // 144 |     print!("a {a} ");
    //     |                ^ value borrowed here after move
    //     |
    //     = note: this error originates in the macro `$crate::format_args` which comes from the expansion of the macro `print` (in Nightly builds, run with -Z macro-backtrace for more info)
    // help: consider cloning the value if the performance cost is acceptable
    //     |
    // 143 |     let b = a.clone();
    //     |              ++++++++
    let a = Box::new(42);
    let b = a.clone();
    print!("a {a} ");
    print!("b {b} ");
}

fn const_by_default() {
    let a = 1;
    // a = 2;
    // error[E0384]: cannot assign twice to immutable variable `a`
    //    --> src/main.rs:168:5
    //     |
    // 167 |     let a = 1;
    //     |         -
    //     |         |
    //     |         first assignment to `a`
    //     |         help: consider making this binding mutable: `mut a`
    // 168 |     a = 2;
    //     |     ^^^^^ cannot assign twice to immutable variable
    let mut a = 1;
    a = 2;
    println!("const_by_default {a}");
}

fn main() {
    single_line_in_if_without_braces();
    assigment_in_if();
    unnecessary_parentheses_in_if();
    explicit_type_conversion();
    uninitialized_variable();
    uninitialized_variable_with_condition();
    use_after_move();
    const_by_default();
}
