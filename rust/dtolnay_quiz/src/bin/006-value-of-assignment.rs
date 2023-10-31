// https://github.com/dtolnay/rust-quiz/blob/master/questions/006-value-of-assignment.md

use std::mem;

fn main() {
    let a;
    let a = a = true;
    print!("{}", mem::size_of_val(&a));
}
