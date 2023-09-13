static CONDITION: bool = true;

fn main() {
    // Can handle const with condition
    let a: i32;
    if CONDITION {
        a = 1;
    } else {
        a = 2;
    }
    println!("{a}");

    // This case not valid
    let a: i32;
    if CONDITION {
        a = 1;
    }
    // println!("{a}"); // a is possibly-uninitialized

    // Ternary is the same as IIFE
    let a = if CONDITION { 1 } else { 2 };
    println!("{a}");

    // NOTE: Notice that const is default
}
