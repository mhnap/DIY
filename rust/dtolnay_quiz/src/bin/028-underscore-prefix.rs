// https://github.com/dtolnay/rust-quiz/blob/master/questions/028-underscore-prefix.md

struct Guard;

impl Drop for Guard {
    fn drop(&mut self) {
        print!("1");
    }
}

fn main() {
    let _guard = Guard;
    print!("3");
    let _ = Guard;
    print!("2");
}
