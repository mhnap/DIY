use my_practices::print_err;
use snafu::{prelude::*, ResultExt};
use std::{fs, io, path::PathBuf};

fn main() {
    // Basic usage.

    {
        #[derive(Debug, Snafu)]
        enum MyError {
            #[snafu(display("Unable to read configuration from '{}'", path.display()))]
            ReadConfiguration { path: PathBuf, source: io::Error },

            #[snafu(display("Unable to write result to '{}'", path.display()))]
            WriteResult { path: PathBuf, source: io::Error },
        }

        fn process_data() -> Result<(), MyError> {
            let path = "config.toml";
            let configuration =
                fs::read_to_string(path).context(ReadConfigurationSnafu { path })?;
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

    //

    // Can add location that will be implicitly captured.

    {
        #[derive(Debug, Snafu)]
        enum Error {
            #[snafu(display("I'm the Only One"))]
            Only {
                #[snafu(implicit)]
                loc: snafu::Location,
            },
        }

        fn get_only() -> Result<(), Error> {
            OnlySnafu.fail()
        }

        match get_only() {
            Ok(()) => println!("All good"),
            Err(err) => {
                print_err!(err);
            }
        }
    }

    //

    // There is `snafu::Whatever` but it cannot be converted from any error like `anyhow` or `eyre`.

    {
        // fn read_env_var(key: &str) -> Result<u8, snafu::Whatever> {
        //     let var = std::env::var(key)?;
        //     let num: u8 = var.parse()?;
        //     Ok(num)
        // }
        //
        //     error[E0277]: `?` couldn't convert the error to `Whatever`
        //     --> my/crates_usage/src/bin/snafu.rs:90:41
        //      |
        //   89 |         fn read_env_var(key: &str) -> Result<u8, snafu::Whatever> {
        //      |                                       --------------------------- expected `Whatever` because of this
        //   90 |             let var = std::env::var(key)?;
        //      |                       ------------------^ the trait `From<VarError>` is not implemented for `Whatever`, which is required by `Result<u8, Whatever>: FromResidual<Result<Infallible, VarError>>`
        //      |                       |
        //      |                       this can't be annotated with `?` because it has type `Result<_, VarError>`
        //      |

        fn read_env_var(key: &str) -> Result<u8, snafu::Whatever> {
            let var = std::env::var(key).whatever_context("cannot read env var")?;
            let num: u8 =
                var.parse().with_whatever_context(|e| format!("cannot parse var: {e}"))?;
            Ok(num)
        }

        match read_env_var("PATH") {
            Ok(buf) => println!("We read env var: {buf:?}"),
            Err(err) => {
                print_err!(err);
            }
        }
    }

    //

    // Can combine the whatever! macro with an enum error type.

    {
        #[derive(Debug, Snafu)]
        enum Error {
            #[snafu(display("ID may not be less than 10, but it was {id}"))]
            InvalidId { id: u16 },

            #[snafu(whatever, display("{message}"))]
            Whatever { message: String },
        }

        fn is_valid_id(id: u16) -> Result<(), Error> {
            ensure!(id >= 10, InvalidIdSnafu { id });
            whatever!("Just kidding... this function always fails!");
            Ok(())
        }

        match is_valid_id(10) {
            Ok(buf) => println!("We read env var: {buf:?}"),
            Err(err) => {
                print_err!(err);
            }
        }
    }

    //

    // Can use `transparent` as `thiserror`.

    {
        #[derive(Debug, Snafu)]
        enum MyError {
            #[snafu(transparent)]
            Io {
                source: std::io::Error,
                #[snafu(implicit)]
                location: snafu::Location,
                backtrace: snafu::Backtrace, // Already implicit if named as `backtrace`
            },
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

        match read_two_files("file1".as_ref(), "file2".as_ref()) {
            Ok(buf) => println!("We read buf: {buf:?}"),
            Err(err) => {
                print_err!(err);
            }
        }
    }

    //

    // Can use doc comments for `Display`.

    {
        #[derive(Debug, Snafu)]
        enum Error {
            /// No user available.
            /// You may need to specify one.
            MissingUser,
            MissingPassword,
        }

        print_err!(MissingUserSnafu.build());
        print_err!(MissingPasswordSnafu.build());
    }
}
