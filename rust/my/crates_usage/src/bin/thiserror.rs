#![feature(error_generic_member_access)]
#![feature(error_iter)]

macro_rules! print_err {
    ($err:expr) => {
        eprintln!("----- {} at {} -----", stringify!($err), line!());
        eprintln!("Display:\n{}", $err);
        eprintln!("Display alternate:\n{:#}", $err);
        eprintln!("Debug:\n{:?}", $err);
        eprintln!("Debug alternate:\n{:#?}", $err);
        let as_dyn: &dyn std::error::Error = &$err;
        for source in as_dyn.sources().skip(1) {
            eprintln!("Caused by: {source}");
        }
        eprintln!("----------------------");
    };
}

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
    dbg!(format!("{my_error1}"));

    //

    /// [`thiserror::Error`] take care of implementing [`std::fmt::Display`] and [`std::error::Error`].
    #[derive(Debug, thiserror::Error, Clone)]
    enum MyError2 {
        #[error("my error 2: {0}")]
        SomeErr(String),
    }

    let my_error2 = MyError2::SomeErr(String::from("some another error"));
    dbg!(format!("{my_error2}"));

    //

    /// Also can generate [`core::convert::From`] and [`std::error::Error::source`].
    #[derive(Debug, thiserror::Error, Clone)]
    enum MyError3 {
        #[error("my error 3: {0}")]
        SomeErr(#[from] MyError2),
    }

    let my_error3 = MyError3::from(my_error2);
    dbg!(format!("{my_error3}"));

    //

    // It's better not to include an inner source error in the `Display` implementation,
    // as if we iterate through a chain of errors, the inner source error will be seen two times.

    // From https://www.lpalmieri.com/posts/error-handling-rust
    fn error_chain(e: &impl std::error::Error) {
        eprint!("Error: {e}\n");
        let mut current = e.source();
        while let Some(cause) = current {
            eprint!("Caused by: {cause}\n");
            current = cause.source();
        }
    }

    #[derive(Debug, thiserror::Error)]
    enum MyError4 {
        #[error("my error 4")]
        SomeErr(#[from] MyError3),
    }

    let my_error4 = MyError4::from(my_error3.clone());
    dbg!(format!("{my_error4}"));
    error_chain(&my_error4);

    //

    // #[source] can be useful when no `From` is required or couple of the same inner types can be as a source.
    //
    // #[derive(Debug, thiserror::Error)]
    // enum MyError5 {
    //     #[error("my error 5: some err 1")]
    //     SomeErr1(#[from] MyError3),
    //
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
    dbg!(format!("{my_error51}"));
    error_chain(&my_error51);

    let my_error52 = MyError5::SomeErr2(my_error3);
    dbg!(format!("{my_error52}"));
    error_chain(&my_error52);

    //

    {
        // Errors may use `#[error(transparent)]` to forward the source and Display methods
        // straight through to an underlying error without adding an additional message.
        #[derive(Debug, thiserror::Error)]
        enum MyError {
            #[error(transparent)]
            Io(#[from] std::io::Error),
        }

        // But it's sometimes hard to understand in what exactly place the error was created.

        fn read_two_files(
            path1: &std::path::Path,
            path2: &std::path::Path,
        ) -> Result<Vec<u8>, MyError> {
            let mut buf1 = std::fs::read(path1)?;
            let buf2 = std::fs::read(path2)?;
            buf1.extend(buf2);
            Ok(buf1)
        }

        let res = read_two_files("file1".as_ref(), "file2".as_ref());
        match res {
            Ok(buf) => println!("We read buf: {buf:?}"),
            Err(err) => {
                print_err!(err);
            }
        }

        // Output is:
        // We got err: No such file or directory (os error 2)
    }

    // But what exactly file could not be read?

    //

    // It can be solved by using backtrace but only on nightly as this is unstable feature.
    // Note that usage of backtrace is automatically detected and `provide` method is always generated,
    // and cannot be turned off, so it's not possible to have backtrace with thiserror on stable.
    //
    //     error[E0658]: use of unstable library feature 'error_generic_member_access'
    //     --> my/crates_usage/src/bin/thiserror.rs:146:21
    //      |
    //  146 |     #[derive(Debug, thiserror::Error)]
    //      |                     ^^^^^^^^^^^^^^^^
    //      |
    //      = note: see issue #99301 <https://github.com/rust-lang/rust/issues/99301> for more information
    //      = note: this error originates in the derive macro `thiserror::Error` (in Nightly builds, run with -Z macro-backtrace for more info)

    // #[error(transparent)] requires exactly one field
    //
    // #[derive(Debug, thiserror::Error)]
    // enum MyError {
    //     #[error(transparent)]
    //     Io(#[from] std::io::Error, std::backtrace::Backtrace),
    // }
    //     error: #[error(transparent)] requires exactly one field
    //     --> my/crates_usage/src/bin/thiserror.rs:143:9
    //      |
    //  143 | /         #[error(transparent)]
    //  144 | |         Io(#[from] std::io::Error, std::backtrace::Backtrace),
    //      | |________________________________________________________________^

    {
        #[derive(Debug, thiserror::Error)]
        enum MyError {
            #[error("Io error")]
            Io(#[from] std::io::Error, std::backtrace::Backtrace),
        }

        // For variants that use #[from] and also contain a Backtrace field, a backtrace is captured from within the From impl.
        //
        // Expanded:
        // #[allow(unused_qualifications)]
        // impl ::core::convert::From<std::io::Error> for MyError {
        //     #[allow(deprecated)]
        //     fn from(source: std::io::Error) -> Self {
        //         MyError::Io {
        //             0: source,
        //             1: ::core::convert::From::from(std::backtrace::Backtrace::capture()),
        //         }
        //     }
        // }

        fn read_two_files(
            path1: &std::path::Path,
            path2: &std::path::Path,
        ) -> Result<Vec<u8>, MyError> {
            let mut buf1 = std::fs::read(path1)?;
            let buf2 = std::fs::read(path2)?;
            buf1.extend(buf2);
            Ok(buf1)
        }

        let res = read_two_files("file1".as_ref(), "file2".as_ref());
        match res {
            Ok(buf) => println!("We read buf: {buf:?}"),
            Err(err) => {
                print_err!(err);
                if let Some(backtrace) = std::error::request_ref::<std::backtrace::Backtrace>(&err)
                {
                    eprintln!("With backtrace: {backtrace}");
                }
            }
        }
    }

    //

    // I found one solution how to use backtrace with `thiserror` on stable:
    // It's needed to create new type alias for backtrace.

    {
        type MyBacktrace = std::backtrace::Backtrace;

        #[derive(Debug, thiserror::Error)]
        enum MyError {
            #[error("Io error")]
            Io(#[source] std::io::Error, MyBacktrace),
        }

        impl From<std::io::Error> for MyError {
            fn from(err: std::io::Error) -> Self {
                MyError::Io(err, MyBacktrace::capture())
            }
        }

        fn read_two_files(
            path1: &std::path::Path,
            path2: &std::path::Path,
        ) -> Result<Vec<u8>, MyError> {
            let mut buf1 = std::fs::read(path1)?;
            let buf2 = std::fs::read(path2)?;
            buf1.extend(buf2);
            Ok(buf1)
        }

        let res = read_two_files("file1".as_ref(), "file2".as_ref());
        match res {
            Ok(buf) => println!("We read buf: {buf:?}"),
            Err(err) => {
                print_err!(err);
            }
        }
    }

    //

    // Note that `#[error("{0}")]` with `#[source]` or `#[from]` is incorrect usage, as it will be duplicated in source.
    // Only `#[error(transparent)]` can be used with `#[source]` or `#[from]` or new error string.
    //
    // #[error(transparent)]
    // Io(#[from] std::io::Error),
    //
    // or
    //
    // #[error("Io error")]
    // Io(#[from] std::io::Error, std::backtrace::Backtrace),
    //
    // not
    //
    // #[error("{0}")]
    // Io(#[from] std::io::Error, std::backtrace::Backtrace),

    //

    // If a field is both a source (named source, or has #[source] or #[from] attribute) and is marked #[backtrace],
    // then the Error trait’s provide() method is forwarded to the source’s provide so that both layers of the error share the same backtrace.
    // The #[backtrace] attribute requires a nightly compiler with Rust version 1.73 or newer.

    {
        // External error from some sub-crate we nothing knows about.
        #[derive(Debug, thiserror::Error)]
        enum ExternalError {
            #[error("Io error")]
            Io(#[from] std::io::Error, std::backtrace::Backtrace),

            #[error("Other error")]
            Other,
        }

        fn read_two_files(
            path1: &std::path::Path,
            path2: &std::path::Path,
        ) -> Result<Vec<u8>, ExternalError> {
            let mut buf1 = std::fs::read(path1)?;
            let buf2 = std::fs::read(path2)?;
            buf1.extend(buf2);
            Ok(buf1)
        }

        // Our own error.
        #[derive(Debug, thiserror::Error)]
        enum MyError {
            // Variant for storing external error.
            #[error(transparent)]
            External {
                #[from]
                // Adds `provide` for taking backtrace from source (if exists).
                //
                // fn provide<'_request>(&'_request self, request: &mut std::error::Request<'_request>) {
                //     #[allow(deprecated)]
                //     match self {
                //         MyError::External { source: source, .. } => {
                //             use thiserror::__private::ThiserrorProvide as _;
                //             source.thiserror_provide(request);
                //         }
                //     }
                // }
                //
                // But there is nothing specific to backtrace, so can be used more broadly.
                #[backtrace]
                source: ExternalError,
            },
        }

        fn my_read_two_files() -> Result<Vec<u8>, MyError> {
            Ok(read_two_files("file1".as_ref(), "file2".as_ref())?)
        }

        let res = my_read_two_files();
        match res {
            Ok(buf) => println!("We read buf: {buf:?}"),
            Err(err) => {
                print_err!(err);
                if let Some(backtrace) = std::error::request_ref::<std::backtrace::Backtrace>(&err)
                {
                    eprintln!("With backtrace: {backtrace}");
                } else {
                    eprintln!("No backtrace..");
                }
            }
        }

        // We requested a backtrace from `MyError` and it's provided from `ExternalError`.
        // Part of backtrace:
        //
        //    2: thiserror::main::read_two_files
        //              at ./my/crates_usage/src/bin/thiserror.rs:311:28
        //    3: thiserror::main::my_read_two_files
        //              at ./my/crates_usage/src/bin/thiserror.rs:343:16
        //    4: thiserror::main
        //              at ./my/crates_usage/src/bin/thiserror.rs:346:19

        fn other() -> Result<(), ExternalError> {
            Err(ExternalError::Other)
        }

        fn my_other() -> Result<(), MyError> {
            Ok(other()?)
        }

        let res = my_other();
        match res {
            Ok(buf) => println!("Success!"),
            Err(err) => {
                print_err!(err);
                if let Some(backtrace) = std::error::request_ref::<std::backtrace::Backtrace>(&err)
                {
                    eprintln!("With backtrace: {backtrace}");
                } else {
                    eprintln!("No backtrace..");
                }
            }
        }

        // But in such case there would be no backtrace provided.
    }

    //

    // Or we also can have our own backtrace.

    {
        // External error from some sub-crate we nothing knows about.
        #[derive(Debug, thiserror::Error)]
        enum ExternalError {
            #[error("Io error")]
            Io(#[from] std::io::Error, std::backtrace::Backtrace),

            #[error("Other error")]
            Other,
        }

        fn read_two_files(
            path1: &std::path::Path,
            path2: &std::path::Path,
        ) -> Result<Vec<u8>, ExternalError> {
            let mut buf1 = std::fs::read(path1)?;
            let buf2 = std::fs::read(path2)?;
            buf1.extend(buf2);
            Ok(buf1)
        }

        // Our own error.
        #[derive(Debug, thiserror::Error)]
        enum MyError {
            // Variant for storing external error.
            #[error("External error")]
            External {
                #[from]
                // Adds `provide` for taking backtrace from source (if exists).
                source: ExternalError,
                // #[backtrace] // This will discard source backtrace if used. Why??
                backtrace: std::backtrace::Backtrace,
            },
        }

        fn my_read_two_files() -> Result<Vec<u8>, MyError> {
            Ok(read_two_files("file1".as_ref(), "file2".as_ref())?)
        }

        let res = my_read_two_files();
        match res {
            Ok(buf) => println!("We read buf: {buf:?}"),
            Err(err) => {
                print_err!(err);
                if let Some(backtrace) = std::error::request_ref::<std::backtrace::Backtrace>(&err)
                {
                    eprintln!("With backtrace: {backtrace}");
                } else {
                    eprintln!("No backtrace..");
                }
            }
        }

        // We requested a backtrace from `MyError` and it's provided from `ExternalError`.
        // Part of backtrace:
        //
        //    2: thiserror::main::read_two_files
        //              at ./my/crates_usage/src/bin/thiserror.rs:414:28
        //    3: thiserror::main::my_read_two_files
        //              at ./my/crates_usage/src/bin/thiserror.rs:435:16
        //    4: thiserror::main
        //              at ./my/crates_usage/src/bin/thiserror.rs:438:19

        fn other() -> Result<(), ExternalError> {
            Err(ExternalError::Other)
        }

        fn my_other() -> Result<(), MyError> {
            Ok(other()?)
        }

        let res = my_other();
        match res {
            Ok(buf) => println!("Success!"),
            Err(err) => {
                print_err!(err);
                if let Some(backtrace) = std::error::request_ref::<std::backtrace::Backtrace>(&err)
                {
                    eprintln!("With backtrace: {backtrace}");
                } else {
                    eprintln!("No backtrace..");
                }
            }
        }

        // There is no backtrace from `ExternalError`, but there is from `MyError`.
        //
        //    2: thiserror::main::my_other
        //              at ./my/crates_usage/src/bin/thiserror.rs:467:16
        //    3: thiserror::main
        //              at ./my/crates_usage/src/bin/thiserror.rs:470:19
    }
}
