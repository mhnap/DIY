// https://github.com/dtolnay/rust-quiz/blob/master/questions/032-or-pattern-guard.md

fn check(x: i32) -> bool {
    print!("{}", x);
    false
}

fn main() {
    match (1, 2) {
        (x, _) | (_, x) if check(x) => {
            print!("3")
        }
        _ => print!("4"),
    }
}
