fn main() {
    // let guess = "42".parse().expect("Not a number!");
    // error[E0282]: type annotations needed
    //  --> src/lessons/type_infer.rs:2:9
    //   |
    // 2 |     let guess = "42".parse().expect("Not a number!");
    //   |         ^^^^^
    //   |
    // help: consider giving `guess` an explicit type
    //   |
    // 2 |     let guess: /* Type */ = "42".parse().expect("Not a number!");
    //   |              ++++++++++++

    let guess: u8 = "42".parse().expect("Not a number!");
    dbg!(guess);
}
