/// Needs manual [`std::fmt::Display`] and [`std::error::Error`] implementations.
#[derive(Debug)]
enum MyError1 {
    SomeErr(String),
}

impl std::fmt::Display for MyError1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError1::SomeErr(err) => write!(f, "my error 1 is {err}"),
        }
    }
}

impl std::error::Error for MyError1 {}

/// [`thiserror::Error`] take care of implementing [`std::fmt::Display`] and [`std::error::Error`].
#[derive(Debug, thiserror::Error)]
enum MyError2 {
    #[error("my error 2 is {0}")]
    SomeErr(String),
}

/// Also can generate [`core::convert::From`] and [`std::error::Error::source`].
#[derive(Debug, thiserror::Error)]
enum MyError3 {
    #[error("my error 3 is {0}")]
    SomeErr(#[from] MyError2),
}

fn main() {
    let my_error1 = MyError1::SomeErr(String::from("some error"));
    println!("{my_error1}");

    let my_error2 = MyError2::SomeErr(String::from("some another error"));
    println!("{my_error2}");

    let my_error3 = MyError3::from(my_error2);
    println!("{my_error3}");
}
