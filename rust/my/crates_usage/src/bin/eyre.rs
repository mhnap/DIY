use color_eyre::eyre::Context;

macro_rules! print_err {
    ($err:expr) => {
        eprintln!("----- {} at {} -----", stringify!($err), line!());
        eprintln!("Display:\n{}", $err);
        eprintln!("Display alternate:\n{:#}", $err);
        eprintln!("Debug:\n{:?}", $err);
        eprintln!("Debug alternate:\n{:#?}", $err);
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

    let res = read_env_var("MY_ENV")
        .context("Failed to read env var")
        .context("One more outer error");
    match res {
        Ok(buf) => println!("We read env var: {buf:?}"),
        Err(err) => {
            print_err!(err);
        }
    }

    //

    {
        // Can be used to save the original error together with backtrace and location.

        #[derive(Debug, thiserror::Error)]
        enum MyError {
            #[error(transparent)]
            Io(color_eyre::eyre::Error),
        }

        impl From<std::io::Error> for MyError {
            #[track_caller]
            fn from(err: std::io::Error) -> Self {
                MyError::Io(err.into())
            }
        }

        impl MyError {
            fn as_eyre_err(&self) -> Option<&color_eyre::eyre::Error> {
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
}
