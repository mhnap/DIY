// https://github.com/dtolnay/rust-quiz/blob/master/questions/016-prefix-decrement.md

fn main() {
    let mut x = 4;
    --x;
    print!("{}{}", --x, --x);
}
