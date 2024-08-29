macro_rules! print_err {
    ($err:expr) => {
        eprintln!(
            "----- {} at {}:{}:{} -----",
            stringify!($err),
            file!(),
            line!(),
            column!()
        );
        eprintln!("Display:\n{}", $err);
        eprintln!("Display alternate:\n{:#}", $err);
        eprintln!("Debug:\n{:?}", $err);
        eprintln!("Debug alternate:\n{:#?}", $err);
        error_chain(&$err);
        eprintln!("------------------------------------------------------------------");
    };
}

fn error_chain(e: &impl std::error::Error) {
    let mut current = e.source();
    while let Some(cause) = current {
        eprintln!("Caused by: {cause}, dbg: {cause:?}");
        current = cause.source();
    }
}

fn main() {
    // With `anyhow`.

    {
        #[derive(thiserror::Error, Debug)]
        enum MyError {
            #[error(transparent)]
            Other(#[from] anyhow::Error),
        }

        impl From<std::env::VarError> for MyError {
            fn from(value: std::env::VarError) -> Self {
                Self::Other(value.into())
            }
        }

        impl From<std::num::ParseIntError> for MyError {
            fn from(value: std::num::ParseIntError) -> Self {
                Self::Other(value.into())
            }
        }

        fn read_env_var(key: &str) -> Result<u8, MyError> {
            let var = std::env::var(key)?;
            let num: u8 = var.parse()?;
            Ok(num)
        }

        match read_env_var("MY_ENV") {
            Ok(buf) => println!("We read env var: {buf:?}"),
            Err(err) => {
                print_err!(err);
            }
        }
    }

    //

    // With `Box<dyn std::error::Error>`.

    {
        #[derive(Debug)]
        enum MyError {
            Other(
                Box<dyn std::error::Error>,
                std::panic::Location<'static>,
                std::backtrace::Backtrace,
            ),
        }

        impl std::fmt::Display for MyError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    MyError::Other(_0, ..) => std::fmt::Display::fmt(_0, f),
                }
            }
        }

        impl std::error::Error for MyError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match self {
                    MyError::Other(_0, ..) => std::error::Error::source(_0.as_ref()),
                }
            }
        }

        impl From<std::env::VarError> for MyError {
            #[track_caller]
            fn from(value: std::env::VarError) -> Self {
                Self::Other(
                    value.into(),
                    *std::panic::Location::caller(),
                    std::backtrace::Backtrace::capture(),
                )
            }
        }

        impl From<std::num::ParseIntError> for MyError {
            #[track_caller]
            fn from(value: std::num::ParseIntError) -> Self {
                Self::Other(
                    value.into(),
                    *std::panic::Location::caller(),
                    std::backtrace::Backtrace::capture(),
                )
            }
        }

        fn read_env_var(key: &str) -> Result<u8, MyError> {
            let var = std::env::var(key)?;
            let num: u8 = var.parse()?;
            Ok(num)
        }

        match read_env_var("MY_ENV") {
            Ok(buf) => println!("We read env var: {buf:?}"),
            Err(err) => {
                print_err!(err);
            }
        }
    }

    //

    // Can get external error with backtrace.

    {
        #[derive(Debug, thiserror::Error)]
        enum ExternalError {
            #[error(transparent)]
            Io(#[from] anyhow::Error),

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

        #[derive(Debug, thiserror::Error)]
        enum MyError {
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
        //    4: anyhow_vs_box_error::main::read_two_files
        //              at ./my/crates_usage/src/bin/anyhow_vs_box_error.rs:153:28
        //    5: anyhow_vs_box_error::main::my_read_two_files
        //              at ./my/crates_usage/src/bin/anyhow_vs_box_error.rs:166:16
        //    6: anyhow_vs_box_error::main
        //              at ./my/crates_usage/src/bin/anyhow_vs_box_error.rs:169:19

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

    {
        #[derive(Debug)]
        enum ExternalError {
            Io(
                Box<dyn std::error::Error>,
                std::panic::Location<'static>,
                std::backtrace::Backtrace,
            ),
            Other,
        }

        impl std::fmt::Display for ExternalError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    ExternalError::Io(_0, ..) => std::fmt::Display::fmt(_0, f),
                    ExternalError::Other => write!(f, "Other"),
                }
            }
        }

        impl std::error::Error for ExternalError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match self {
                    ExternalError::Io(_0, ..) => std::error::Error::source(_0.as_ref()),
                    ExternalError::Other => None,
                }
            }
        }

        impl From<std::io::Error> for ExternalError {
            #[track_caller]
            fn from(value: std::io::Error) -> Self {
                Self::Io(
                    value.into(),
                    *std::panic::Location::caller(),
                    std::backtrace::Backtrace::capture(),
                )
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

        #[derive(Debug, thiserror::Error)]
        enum MyError {
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

        // Location: 249

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

        // But in such case there would be no location.
    }

    //

    // But if our error also wants to use backtrace, only it would be used, and not from external error.

    {
        #[derive(Debug, thiserror::Error)]
        enum ExternalError {
            #[error(transparent)]
            Io(#[from] anyhow::Error),

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

        #[derive(Debug, thiserror::Error)]
        enum MyError {
            #[error(transparent)]
            External(#[from] anyhow::Error),
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
        //    4: anyhow_vs_box_error::main::my_read_two_files
        //              at ./my/crates_usage/src/bin/anyhow_vs_box_error.rs:337:16
        //    5: anyhow_vs_box_error::main
        //              at ./my/crates_usage/src/bin/anyhow_vs_box_error.rs:340:19

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
        //    4: anyhow_vs_box_error::main::my_other
        //              at ./my/crates_usage/src/bin/anyhow_vs_box_error.rs:360:16
        //    5: anyhow_vs_box_error::main
        //              at ./my/crates_usage/src/bin/anyhow_vs_box_error.rs:363:19
    }

    {
        #[derive(Debug)]
        enum ExternalError {
            Io(
                Box<dyn std::error::Error>,
                std::panic::Location<'static>,
                std::backtrace::Backtrace,
            ),
            Other,
        }

        impl std::fmt::Display for ExternalError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    ExternalError::Io(_0, ..) => std::fmt::Display::fmt(_0, f),
                    ExternalError::Other => write!(f, "Other"),
                }
            }
        }

        impl std::error::Error for ExternalError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match self {
                    ExternalError::Io(_0, ..) => std::error::Error::source(_0.as_ref()),
                    ExternalError::Other => None,
                }
            }
        }

        impl From<std::io::Error> for ExternalError {
            #[track_caller]
            fn from(value: std::io::Error) -> Self {
                Self::Io(
                    value.into(),
                    *std::panic::Location::caller(),
                    std::backtrace::Backtrace::capture(),
                )
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

        #[derive(Debug)]
        enum MyError {
            External(
                Box<dyn std::error::Error>,
                std::panic::Location<'static>,
                std::backtrace::Backtrace,
            ),
        }

        impl std::fmt::Display for MyError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    MyError::External(_0, ..) => std::fmt::Display::fmt(_0, f),
                }
            }
        }

        impl std::error::Error for MyError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match self {
                    MyError::External(_0, ..) => std::error::Error::source(_0.as_ref()),
                }
            }
        }

        impl From<ExternalError> for MyError {
            #[track_caller]
            fn from(value: ExternalError) -> Self {
                Self::External(
                    value.into(),
                    *std::panic::Location::caller(),
                    std::backtrace::Backtrace::capture(),
                )
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

        // Location: 423

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

        // Location: 484
    }
}
