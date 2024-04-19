// https://github.com/dtolnay/rust-quiz/blob/master/questions/008-tokenize-punctuation.md

macro_rules! m {
    (==>) => { print!("1"); };
    (= = >) => { print!("2"); };
    (== >) => { print!("3"); };
    (= =>) => { print!("4"); };
}

fn main() {
    m!(==>);
    m!(= = >);
    m!(== >);
    m!(= =>);
}
