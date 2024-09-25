use my_experiments::non_exhaustive::{create_config, Config, Error, Foo};

fn main() {
    // let cfg = Config { width: 640, height: 480 };
    //
    // error[E0639]: cannot create non-exhaustive struct using struct expression
    //  --> my/experiments/src/bin/non_exhaustive.rs:3:15
    //   |
    // 3 |     let cfg = Config { width: 640, height: 480 };
    //   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

    let cfg = create_config();

    // let Config { width, height } = cfg;
    //
    // error[E0638]: `..` required with struct marked as non-exhaustive
    //   --> my/experiments/src/bin/non_exhaustive.rs:13:9
    //    |
    // 13 |     let Config { width, height } = cfg;
    //    |         ^^^^^^^^^^^^^^^^^^^^^^^^
    //    |
    // help: add `..` at the end of the field list to ignore all other fields
    //    |
    // 13 |     let Config { width, height , .. } = cfg;
    //    |                                ~~~~~~

    let Config { width, height, .. } = cfg;

    // let err = Error::Other;
    // match err {
    //     Error::Message(str) => println!("message is {str}"),
    //     Error::Other => println!("other"),
    // }
    //
    // error[E0004]: non-exhaustive patterns: `_` not covered
    //   --> my/experiments/src/bin/non_exhaustive.rs:29:11
    //    |
    // 29 |     match err {
    //    |           ^^^ pattern `_` not covered
    //    |
    // note: `my_experiments::non_exhaustive::Error` defined here
    //   --> /home/mhnap/projects/DIY/rust/my/experiments/src/non_exhaustive.rs:13:1
    //    |
    // 13 | pub enum Error {
    //    | ^^^^^^^^^^^^^^
    //    = note: the matched value is of type `my_experiments::non_exhaustive::Error`
    //    = note: `my_experiments::non_exhaustive::Error` is marked as non-exhaustive, so a wildcard `_` is necessary to match exhaustively
    // help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
    //    |
    // 31 ~         Error::Other => println!("other"),
    // 32 ~         _ => todo!(),
    //    |

    let err = Error::Other;
    match err {
        Error::Message(str) => println!("message is {str}"),
        Error::Other => println!("other"),
        _ => println!("new variant"),
    }

    // let foo = Foo;
    //
    // error[E0423]: expected value, found struct `Foo`
    //   --> my/experiments/src/bin/non_exhaustive.rs:60:15
    //    |
    // 60 |     let foo = Foo;
    //    |               ^^^ constructor is not visible here due to private fields
}
