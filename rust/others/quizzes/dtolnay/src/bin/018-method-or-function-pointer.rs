// https://github.com/dtolnay/rust-quiz/blob/master/questions/018-method-or-function-pointer.md

struct S {
    f: fn(),
}

impl S {
    fn f(&self) {
        print!("1");
    }
}

fn main() {
    let print2 = || print!("2");
    S { f: print2 }.f();
}
