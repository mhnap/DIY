use snafu::prelude::*;
use std::{fs, io, path::PathBuf};

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
    #[derive(Debug, Snafu)]
    enum Error {
        #[snafu(display("Unable to read configuration from {}", path.display()))]
        ReadConfiguration { source: io::Error, path: PathBuf },

        #[snafu(display("Unable to write result to {}", path.display()))]
        WriteResult { source: io::Error, path: PathBuf },
    }

    type Result<T, E = Error> = std::result::Result<T, E>;

    fn process_data() -> Result<()> {
        let path = "config.toml";
        let configuration = fs::read_to_string(path).context(ReadConfigurationSnafu { path })?;
        let path = unpack_config(&configuration);
        fs::write(&path, b"My complex calculation").context(WriteResultSnafu { path })?;
        Ok(())
    }

    fn unpack_config(data: &str) -> &str {
        "/some/path/that/does/not/exist"
    }

    match process_data() {
        Ok(()) => println!("All good"),
        Err(err) => {
            print_err!(err);
        }
    }
}
