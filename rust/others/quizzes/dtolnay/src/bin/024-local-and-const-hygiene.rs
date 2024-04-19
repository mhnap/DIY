// https://github.com/dtolnay/rust-quiz/blob/master/questions/024-local-and-const-hygiene.md

fn main() {
    let x: u8 = 1;
    const K: u8 = 2;

    macro_rules! m {
        () => {
            print!("{}{}", x, K);
        };
    }

    {
        let x: u8 = 3;
        const K: u8 = 4;

        m!();
    }
}
