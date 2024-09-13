use color_eyre::eyre::Context;

macro_rules! print_err {
    ($err:expr) => {
        eprintln!("----- {} at {}:{}:{} -----", stringify!($err), file!(), line!(), column!());
        eprintln!("Display:\n{}", $err);
        eprintln!("Display alternate:\n{:#}", $err);
        eprintln!("Debug:\n{:?}", $err);
        eprintln!("Debug alternate:\n{:#?}", $err);
        eprintln!("----------------------");
    };
}

fn main() {
    // The heart of this crate is its ability to swap out the Handler type to change what information is carried alongside errors and how the end report is formatted.
    // This crate is meant to be used alongside companion crates that customize its behavior.

    // To customize the format and content of error reports from eyre you must first define a new EyreHandler type to capture and store the extra context and to define the format of how to display the chain of errors and this stored context.
    // Once this type has been defined you must also define a global hook used to construct these handlers whenever Reports are constructed.

    // Set custom eyre hook.
    color_eyre::install().unwrap();

    //

    fn read_env_var(key: &str) -> color_eyre::eyre::Result<u8> {
        let var = std::env::var(key)?;
        let num: u8 = var.parse()?;
        Ok(num)
    }

    let res = read_env_var("MY_ENV");
    match res {
        Ok(buf) => println!("We read env var: {buf:?}"),
        Err(err) => {
            print_err!(err);
        }
    }

    //

    // Wrap a lower level error with a new error created from a message to help the person troubleshooting understand the chain of failures that occurred.

    let res =
        read_env_var("MY_ENV").context("Failed to read env var").context("One more outer error");
    match res {
        Ok(buf) => println!("We read env var: {buf:?}"),
        Err(err) => {
            print_err!(err);
        }
    }

    //

    {
        // Can be used to save the original error together with location and location.

        #[derive(Debug, thiserror::Error)]
        enum MyError {
            #[error(transparent)]
            Io(color_eyre::Report),
        }

        impl From<std::io::Error> for MyError {
            #[track_caller]
            fn from(err: std::io::Error) -> Self {
                MyError::Io(err.into())
            }
        }

        impl MyError {
            fn as_eyre_err(&self) -> Option<&color_eyre::Report> {
                match self {
                    MyError::Io(err) => Some(err),
                    _ => None,
                }
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

                if let Some(err) = err.as_eyre_err() {
                    eprintln!("{err:?}");
                }
            }
        }
    }

    //

    // Can get external error with location.

    {
        // External error from some sub-crate we nothing knows about.
        #[derive(Debug, thiserror::Error)]
        enum ExternalError {
            #[error(transparent)]
            Io(color_eyre::Report),

            #[error("Other error")]
            Other,
        }

        impl From<std::io::Error> for ExternalError {
            #[track_caller]
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

        // Location:
        //  my/crates_usage/src/bin/eyre.rs:136

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

    // But if our error also wants to use location, only it would be used, and not from external error.

    {
        // External error from some sub-crate we nothing knows about.
        #[derive(Debug, thiserror::Error)]
        enum ExternalError {
            #[error(transparent)]
            Io(color_eyre::Report),

            #[error("Other error")]
            Other,
        }

        impl From<std::io::Error> for ExternalError {
            #[track_caller]
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
            External(color_eyre::Report),
        }

        impl From<ExternalError> for MyError {
            #[track_caller]
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

        // Location:
        //  my/crates_usage/src/bin/eyre.rs:232

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

        // Location:
        //  my/crates_usage/src/bin/eyre.rs:251
    }

    //

    // But we can add some manual logic to handle this.

    {
        // External error from some sub-crate we nothing knows about.
        #[derive(Debug, thiserror::Error)]
        enum ExternalError {
            #[error(transparent)]
            Io(color_eyre::Report),

            #[error("Other error")]
            Other,
        }

        impl From<std::io::Error> for ExternalError {
            #[track_caller]
            fn from(value: std::io::Error) -> Self {
                Self::Io(value.into())
            }
        }

        impl ExternalError {
            fn into_eyre_report(self) -> Result<color_eyre::Report, Self> {
                match self {
                    ExternalError::Io(eyre_report) => Ok(eyre_report),
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
            External(color_eyre::Report),
        }

        impl From<ExternalError> for MyError {
            #[track_caller]
            fn from(value: ExternalError) -> Self {
                match value.into_eyre_report() {
                    Ok(eyre_report) => Self::External(eyre_report),
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

        // We get the original error and location.
        //
        // Location:
        //  my/crates_usage/src/bin/eyre.rs:301

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

        // Location:
        //  my/crates_usage/src/bin/eyre.rs:347
    }
}
