// https://github.com/dtolnay/rust-quiz/blob/master/questions/013-mutable-zst.md

struct S;

fn main() {
    let [x, y] = &mut [S, S];
    let eq = x as *mut S == y as *mut S;
    print!("{}", eq as u8);
}
