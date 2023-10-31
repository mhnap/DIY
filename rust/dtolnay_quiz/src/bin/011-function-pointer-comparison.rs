// https://github.com/dtolnay/rust-quiz/blob/master/questions/011-function-pointer-comparison.md

fn f<'a>() {}
fn g<'a: 'a>() {}

fn main() {
    let pf = f::<'static> as fn();
    let pg = g::<'static> as fn();
    print!("{}", (pf == pg) as u8);
}
