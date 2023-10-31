// https://github.com/dtolnay/rust-quiz/blob/master/questions/019-dropped-by-underscore.md

struct S;

impl Drop for S {
    fn drop(&mut self) {
        print!("1");
    }
}

fn main() {
    let s = S;
    let _ = s;
    print!("2");
}
