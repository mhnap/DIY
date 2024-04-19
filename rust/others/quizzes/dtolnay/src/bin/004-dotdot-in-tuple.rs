// https://github.com/dtolnay/rust-quiz/blob/master/questions/004-dotdot-in-tuple.md

fn main() {
    let (.., x, y) = (0, 1, ..);
    print!("{}", b"066"[y][x]);
}
