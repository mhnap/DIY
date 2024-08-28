#![feature(error_generic_member_access)]

use anyhow::Context;

macro_rules! print_anyhow {
    ($err:expr) => {
        eprintln!("----- {} at {} -----", stringify!($err), line!());
        eprintln!("Display:\n{}", $err);
        eprintln!("Display alternate:\n{:#}", $err);
        eprintln!("Debug:\n{:?}", $err);
        eprintln!("Debug alternate:\n{:#?}", $err);
        eprintln!("----------------------");
    };
}

macro_rules! print_err {
    ($err:expr) => {
        eprintln!("----- {} at {} -----", stringify!($err), line!());
        eprintln!("Display:\n{}", $err);
        eprintln!("Display alternate:\n{:#}", $err);
        eprintln!("Debug:\n{:?}", $err);
        eprintln!("Debug alternate:\n{:#?}", $err);
        error_chain(&$err);
        eprintln!("----------------------");
    };
}

fn error_chain(e: &impl std::error::Error) {
    let mut current = e.source();
    while let Some(cause) = current {
        eprintln!("Caused by: {cause}");
        current = cause.source();
    }
}

fn main() {
    // Use Result<T, anyhow::Error>, or equivalently anyhow::Result<T>, as the return type of any fallible function.
    // Within the function, use ? to easily propagate any error that implements the std::error::Error trait.

    fn read_env_var(key: &str) -> anyhow::Result<u8> {
        let var = std::env::var(key)?;
        let num: u8 = var.parse()?;
        Ok(num)
    }

    let res = read_env_var("MY_ENV");
    match res {
        Ok(buf) => println!("We read env var: {buf:?}"),
        Err(err) => {
            print_anyhow!(err);
        }
    }

    //

    // Attach context to help the person troubleshooting the error understand where things went wrong.

    let res = read_env_var("MY_ENV")
        .context("Failed to read env var")
        .context("One more outer error");
    match res {
        Ok(buf) => println!("We read env var: {buf:?}"),
        Err(err) => {
            print_anyhow!(err);
        }
    }

    //

    // Can get external error with backtrace.

    {
        // External error from some sub-crate we nothing knows about.
        #[derive(Debug, thiserror::Error)]
        enum ExternalError {
            #[error(transparent)]
            Io(anyhow::Error),

            #[error("Other error")]
            Other,
        }

        impl From<std::io::Error> for ExternalError {
            fn from(value: std::io::Error) -> Self {
                Self::Io(value.into())
            }
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
            External(#[from] ExternalError),
        }

        fn my_read_two_files() -> Result<Vec<u8>, MyError> {
            Ok(read_two_files("file1".as_ref(), "file2".as_ref())?)
        }

        let res = my_read_two_files();
        match res {
            Ok(buf) => println!("We read buf: {buf:?}"),
            Err(err) => {
                print_err!(err);
            }
        }

        // Part of backtrace:
        //
        //    4: anyhow::main::read_two_files
        //              at ./my/crates_usage/src/bin/anyhow.rs:93:28
        //    5: anyhow::main::my_read_two_files
        //              at ./my/crates_usage/src/bin/anyhow.rs:108:16
        //    6: anyhow::main
        //              at ./my/crates_usage/src/bin/anyhow.rs:111:19

        fn other() -> Result<(), ExternalError> {
            Err(ExternalError::Other)
        }

        fn my_other() -> Result<(), MyError> {
            Ok(other()?)
        }

        let res = my_other();
        match res {
            Ok(()) => println!("Success!"),
            Err(err) => {
                print_err!(err);
            }
        }

        // But in such case there would be no backtrace.
    }

    //

    // But if our error also wants to use backtrace, only it would be used, and not from external error.

    {
        // External error from some sub-crate we nothing knows about.
        #[derive(Debug, thiserror::Error)]
        enum ExternalError {
            #[error(transparent)]
            Io(anyhow::Error),

            #[error("Other error")]
            Other,
        }

        impl From<std::io::Error> for ExternalError {
            fn from(value: std::io::Error) -> Self {
                Self::Io(value.into())
            }
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
            External(anyhow::Error),
        }

        impl From<ExternalError> for MyError {
            fn from(value: ExternalError) -> Self {
                Self::External(value.into())
            }
        }

        fn my_read_two_files() -> Result<Vec<u8>, MyError> {
            Ok(read_two_files("file1".as_ref(), "file2".as_ref())?)
        }

        let res = my_read_two_files();
        match res {
            Ok(buf) => println!("We read buf: {buf:?}"),
            Err(err) => {
                print_err!(err);
            }
        }

        // Part of backtrace:
        //
        //    4: anyhow::main::my_read_two_files
        //              at ./my/crates_usage/src/bin/anyhow.rs:193:16
        //    5: anyhow::main
        //              at ./my/crates_usage/src/bin/anyhow.rs:196:19

        fn other() -> Result<(), ExternalError> {
            Err(ExternalError::Other)
        }

        fn my_other() -> Result<(), MyError> {
            Ok(other()?)
        }

        let res = my_other();
        match res {
            Ok(()) => println!("Success!"),
            Err(err) => {
                print_err!(err);
            }
        }

        // Part of backtrace:
        //
        //    4: anyhow::main::my_other
        //              at ./my/crates_usage/src/bin/anyhow.rs:216:16
        //    5: anyhow::main
        //              at ./my/crates_usage/src/bin/anyhow.rs:219:19
    }

    //

    // But we can add some manual logic to handle this.

    {
        // External error from some sub-crate we nothing knows about.
        #[derive(Debug, thiserror::Error)]
        enum ExternalError {
            #[error(transparent)]
            Io(anyhow::Error),

            #[error("Other error")]
            Other,
        }

        impl From<std::io::Error> for ExternalError {
            fn from(value: std::io::Error) -> Self {
                Self::Io(value.into())
            }
        }

        impl ExternalError {
            fn into_anyhow_err(self) -> Result<anyhow::Error, Self> {
                match self {
                    ExternalError::Io(anyhow_err) => Ok(anyhow_err),
                    other_err => Err(other_err),
                }
            }
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
            External(anyhow::Error),
        }

        impl From<ExternalError> for MyError {
            fn from(value: ExternalError) -> Self {
                match value.into_anyhow_err() {
                    Ok(anyhow_err) => Self::External(anyhow_err),
                    Err(other_err) => Self::External(other_err.into()),
                }
            }
        }

        fn my_read_two_files() -> Result<Vec<u8>, MyError> {
            Ok(read_two_files("file1".as_ref(), "file2".as_ref())?)
        }

        let res = my_read_two_files();
        match res {
            Ok(buf) => println!("We read buf: {buf:?}"),
            Err(err) => {
                print_err!(err);
            }
        }

        // We get the original error and backtrace.
        //
        // Part of backtrace:
        //
        //    4: anyhow::main::read_two_files
        //              at ./my/crates_usage/src/bin/anyhow.rs:269:28
        //    5: anyhow::main::my_read_two_files
        //              at ./my/crates_usage/src/bin/anyhow.rs:293:16
        //    6: anyhow::main
        //              at ./my/crates_usage/src/bin/anyhow.rs:296:19

        fn other() -> Result<(), ExternalError> {
            Err(ExternalError::Other)
        }

        fn my_other() -> Result<(), MyError> {
            Ok(other()?)
        }

        let res = my_other();
        match res {
            Ok(()) => println!("Success!"),
            Err(err) => {
                print_err!(err);
            }
        }

        // Part of backtrace:
        //
        //    4: anyhow::main::my_other
        //              at ./my/crates_usage/src/bin/anyhow.rs:320:16
        //    5: anyhow::main
        //              at ./my/crates_usage/src/bin/anyhow.rs:323:19
    }

    //

    // Or we can use `error_generic_member_access` feature on nightly.

    {
        // External error from some sub-crate we nothing knows about.
        #[derive(Debug, thiserror::Error)]
        enum ExternalError {
            #[error(transparent)]
            // Io(#[backtrace] anyhow::Error), // Gives error. Why??
            Io {
                #[backtrace]
                source: anyhow::Error,
            },

            #[error("Other error")]
            Other,
        }

        impl From<std::io::Error> for ExternalError {
            fn from(value: std::io::Error) -> Self {
                Self::Io {
                    source: value.into(),
                }
            }
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
            External(anyhow::Error),
        }

        impl From<ExternalError> for MyError {
            fn from(value: ExternalError) -> Self {
                Self::External(value.into())
            }
        }

        fn my_read_two_files() -> Result<Vec<u8>, MyError> {
            Ok(read_two_files("file1".as_ref(), "file2".as_ref())?)
        }

        let res = my_read_two_files();
        match res {
            Ok(buf) => println!("We read buf: {buf:?}"),
            Err(err) => {
                print_err!(err);
            }
        }

        // We get the original error and backtrace.
        //
        // Part of backtrace:
        //
        //    4: anyhow::main::read_two_files
        //              at ./my/crates_usage/src/bin/anyhow.rs:370:28
        //    5: anyhow::main::my_read_two_files
        //              at ./my/crates_usage/src/bin/anyhow.rs:391:16
        //    6: anyhow::main
        //              at ./my/crates_usage/src/bin/anyhow.rs:394:19

        fn other() -> Result<(), ExternalError> {
            Err(ExternalError::Other)
        }

        fn my_other() -> Result<(), MyError> {
            Ok(other()?)
        }

        let res = my_other();
        match res {
            Ok(()) => println!("Success!"),
            Err(err) => {
                print_err!(err);
            }
        }

        // Part of backtrace:
        //
        //    4: anyhow::main::my_other
        //              at ./my/crates_usage/src/bin/anyhow.rs:416:16
        //    5: anyhow::main
        //              at ./my/crates_usage/src/bin/anyhow.rs:419:19
    }
}

#[test]
#[ignore = "https://github.com/dtolnay/thiserror/issues/174"]
fn backtrace_preserved_after_thiserror_derive() {
    #[derive(Debug, thiserror::Error)]
    enum SomeError {
        #[error(transparent)]
        Anyhow(
            #[from]
            // #[backtrace] // <-- Passing with this attribute
            anyhow::Error,
        ),
    }

    let mut errs: Vec<_> = (0..2).map(|_| anyhow::anyhow!("aaa")).collect();

    let wrapped = anyhow::Error::from(SomeError::Anyhow(errs.pop().unwrap()));
    let notwrapped = errs.pop().unwrap();

    assert_eq!(
        wrapped.backtrace().to_string(),
        notwrapped.backtrace().to_string()
    );
}

#[test]
fn source_preserved() {
    #[derive(Debug, thiserror::Error)]
    pub enum SomeError {
        #[error("I/O")]
        Io(#[source] std::io::Error),
    }

    let some_error = SomeError::Io(std::io::Error::new(std::io::ErrorKind::Other, "io error"));
    assert!(some_error.source().is_some());

    let anyhow_error = anyhow::Error::from(some_error);
    assert!(anyhow_error.source().is_some());
}
