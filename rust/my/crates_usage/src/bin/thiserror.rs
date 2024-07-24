fn main() {
    /// Needs manual [`std::fmt::Display`] and [`std::error::Error`] implementations.
    #[derive(Debug)]
    enum MyError1 {
        SomeErr(String),
    }

    impl std::fmt::Display for MyError1 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                MyError1::SomeErr(err) => write!(f, "my error 1: {err}"),
            }
        }
    }

    impl std::error::Error for MyError1 {}

    let my_error1 = MyError1::SomeErr(String::from("some error"));
    println!("{my_error1}");

    //

    /// [`thiserror::Error`] take care of implementing [`std::fmt::Display`] and [`std::error::Error`].
    #[derive(Debug, thiserror::Error, Clone)]
    enum MyError2 {
        #[error("my error 2: {0}")]
        SomeErr(String),
    }

    let my_error2 = MyError2::SomeErr(String::from("some another error"));
    println!("{my_error2}");

    //

    /// Also can generate [`core::convert::From`] and [`std::error::Error::source`].
    #[derive(Debug, thiserror::Error, Clone)]
    enum MyError3 {
        #[error("my error 3: {0}")]
        SomeErr(#[from] MyError2),
    }

    let my_error3 = MyError3::from(my_error2);
    println!("{my_error3}");

    //

    // It's better not to include an inner source error in the `Display` implementation,
    // as if we iterate through a chain of errors, the inner source error will be seen two times.

    // From https://www.lpalmieri.com/posts/error-handling-rust
    fn error_chain(e: &impl std::error::Error) {
        print!("Error: {e}\n");
        let mut current = e.source();
        while let Some(cause) = current {
            print!("Caused by: {cause}\n");
            current = cause.source();
        }
    }

    #[derive(Debug, thiserror::Error)]
    enum MyError4 {
        #[error("my error 4")]
        SomeErr(#[from] MyError3),
    }

    let my_error4 = MyError4::from(my_error3.clone());
    println!("{my_error4}");
    error_chain(&my_error4);

    //

    // #[source] can be useful when no `From` is required or couple of the same inner types can be as a source.

    // #[derive(Debug, thiserror::Error)]
    // enum MyError5 {
    //     #[error("my error 5: some err 1")]
    //     SomeErr1(#[from] MyError3),

    //     #[error("my error 5: some err 2")]
    //     SomeErr2(#[from] MyError3),
    // }
    //     error: cannot derive From because another variant has the same source type
    //     --> my/crates_usage/src/bin/thiserror.rs:80:18
    //      |
    //   80 |         SomeErr2(#[from] MyError3),
    //      |                  ^^^^^^^^^^^^^^^^

    #[derive(Debug, thiserror::Error)]
    enum MyError5 {
        #[error("my error 5: some err 1")]
        SomeErr1(#[from] MyError3),

        #[error("my error 5: some err 2")]
        SomeErr2(#[source] MyError3),
    }

    let my_error51 = MyError5::from(my_error3.clone());
    println!("{my_error51}");
    error_chain(&my_error51);

    let my_error52 = MyError5::SomeErr2(my_error3);
    println!("{my_error52}");
    error_chain(&my_error52);
}
