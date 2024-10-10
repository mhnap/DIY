use my_practices::print_err;
use tracing::instrument;
use tracing_subscriber::prelude::*;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_max_level(tracing::Level::DEBUG)
        .finish()
        .with(tracing_error::ErrorLayer::default())
        .try_init()
        .unwrap();

    //

    // Using `SpanTrace` inside error variant.

    {
        #[derive(Debug)]
        enum ExternalError {
            Io(std::io::Error, tracing_error::SpanTrace),
            Other,
        }

        impl std::fmt::Display for ExternalError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    ExternalError::Io(source, ctx) => {
                        write!(f, "I/O error\n");
                        ctx.fmt(f)?;
                        Ok(())
                    }
                    ExternalError::Other => write!(f, "Other error"),
                }
            }
        }

        impl std::error::Error for ExternalError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match self {
                    ExternalError::Io(source, _) => Some(source),
                    ExternalError::Other => None,
                }
            }
        }

        impl From<std::io::Error> for ExternalError {
            fn from(value: std::io::Error) -> Self {
                Self::Io(value, tracing_error::SpanTrace::capture())
            }
        }

        #[instrument]
        fn read_two_files(
            path1: &std::path::Path,
            path2: &std::path::Path,
        ) -> Result<Vec<u8>, ExternalError> {
            tracing::info!("start reading");
            let mut buf1 = std::fs::read(path1)?;
            tracing::info!("read first file");
            let buf2 = std::fs::read(path2)?;
            tracing::info!("read second file");
            buf1.extend(buf2);
            Ok(buf1)
        }

        #[derive(Debug, thiserror::Error)]
        enum MyError {
            #[error(transparent)]
            External(#[from] ExternalError),
        }

        #[instrument]
        fn my_read_two_files() -> Result<Vec<u8>, MyError> {
            Ok(read_two_files("Cargo.toml".as_ref(), "file2".as_ref())?)
        }

        let res = my_read_two_files();
        match res {
            Ok(buf) => println!("We read buf: {buf:?}"),
            Err(err) => {
                print_err!(err);
            }
        }
    }

    //

    // Wrapping with `TracedError` whole error.

    {
        #[derive(Debug)]
        enum ExternalError {
            Io(std::io::Error),
            Other,
        }

        impl std::fmt::Display for ExternalError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    ExternalError::Io(source) => write!(f, "I/O error"),
                    ExternalError::Other => write!(f, "Other error"),
                }
            }
        }

        impl std::error::Error for ExternalError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match self {
                    ExternalError::Io(source) => Some(source),
                    ExternalError::Other => None,
                }
            }
        }

        impl From<std::io::Error> for ExternalError {
            fn from(value: std::io::Error) -> Self {
                Self::Io(value)
            }
        }

        #[instrument]
        fn read_two_files(
            path1: &std::path::Path,
            path2: &std::path::Path,
        ) -> Result<Vec<u8>, ExternalError> {
            tracing::info!("start reading");
            let mut buf1 = std::fs::read(path1)?;
            tracing::info!("read first file");
            let buf2 = std::fs::read(path2)?;
            tracing::info!("read second file");
            buf1.extend(buf2);
            Ok(buf1)
        }

        #[derive(Debug, thiserror::Error)]
        enum MyError {
            #[error(transparent)]
            External(#[from] ExternalError),
        }

        #[derive(Debug)]
        pub struct NewError(tracing_error::TracedError<MyError>);

        impl std::error::Error for NewError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                self.0.source()
            }
        }

        impl std::fmt::Display for NewError {
            fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.0, fmt)
            }
        }

        impl<E> From<E> for NewError
        where
            MyError: From<E>, // or E: Into<MyError>
        {
            fn from(source: E) -> Self {
                Self(tracing_error::TracedError::from(MyError::from(source)))
            }
        }

        #[instrument]
        fn my_read_two_files() -> Result<Vec<u8>, NewError> {
            Ok(read_two_files("Cargo.toml".as_ref(), "file2".as_ref())?)
        }

        let res = my_read_two_files();
        match res {
            Ok(buf) => println!("We read buf: {buf:?}"),
            Err(err) => {
                print_err!(err);
            }
        }
    }
}
