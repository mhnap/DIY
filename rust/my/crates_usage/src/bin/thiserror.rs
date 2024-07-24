#![feature(error_generic_member_access)]

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

    //

    // Errors may use `#[error(transparent)]` to forward the source and Display methods
    // straight through to an underlying error without adding an additional message.
    #[derive(Debug, thiserror::Error)]
    enum MyError6 {
        #[error(transparent)]
        IoErr(#[from] std::io::Error),
    }

    // But it's sometimes hard to understand in what exactly place the error was created.

    fn read_two_files(
        path1: &std::path::Path,
        path2: &std::path::Path,
    ) -> Result<Vec<u8>, MyError6> {
        let mut buf1 = std::fs::read(path1)?;
        let buf2 = std::fs::read(path2)?;
        buf1.extend(buf2);
        Ok(buf1)
    }

    let res = read_two_files("file1".as_ref(), "file2".as_ref());
    match res {
        Ok(buf) => println!("We read buf: {buf:?}"),
        Err(err) => eprintln!("We got err: {err}"),
    }

    // Output is:
    // We got err: No such file or directory (os error 2)
    // But what exactly file could not be read?

    //

    // It can be solved by using backtrace but only on nightly as this is unstable feature.

    // #[derive(Debug, thiserror::Error)]
    // enum MyError7 {
    //     #[error(transparent)]
    //     IoErr(#[from] std::io::Error, std::backtrace::Backtrace),
    // }
    //     error: #[error(transparent)] requires exactly one field
    //     --> my/crates_usage/src/bin/thiserror.rs:143:9
    //      |
    //  143 | /         #[error(transparent)]
    //  144 | |         IoErr(#[from] std::io::Error, std::backtrace::Backtrace),
    //      | |________________________________________________________________^

    #[derive(Debug, thiserror::Error)]
    enum MyError7 {
        #[error("{0}")]
        IoErr(#[from] std::io::Error, std::backtrace::Backtrace),
    }

    fn read_two_files_v2(
        path1: &std::path::Path,
        path2: &std::path::Path,
    ) -> Result<Vec<u8>, MyError7> {
        let mut buf1 = std::fs::read(path1)?;
        let buf2 = std::fs::read(path2)?;
        buf1.extend(buf2);
        Ok(buf1)
    }

    let res = read_two_files_v2("file1".as_ref(), "file2".as_ref());
    match res {
        Ok(buf) => println!("We read buf: {buf:?}"),
        Err(err) => {
            eprintln!("We got err: {err}",);
            if let Some(backtrace) = std::error::request_ref::<std::backtrace::Backtrace>(&err) {
                eprintln!("With backtrace: {backtrace}");
            }
        }
    }
}
