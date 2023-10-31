// https://github.com/dtolnay/rust-quiz/blob/master/questions/017-unary-decrement.md

fn main() {
    let mut a = 5;
    let mut b = 3;
    print!("{}", a-- - --b);
}
