// https://hegdenu.net/posts/track-caller
// https://blog.rust-lang.org/2020/03/12/Rust-1.42.html#useful-line-numbers-in-option-and-result-panic-messages
// https://blog.rust-lang.org/2020/08/27/Rust-1.46.0.html#track_caller
// https://doc.rust-lang.org/reference/attributes/codegen.html#the-track_caller-attribute
// https://rust-lang.github.io/rfcs/2091-inline-semantic.html

fn main() {
    pub fn do_not_call_with_zero_v1(val: u64) -> u64 {
        if val == 0 {
            panic!("We told you not to do that");
        }
        val
    }

    // It will write location where panic was (in inner function).
    // do_not_call_with_zero_v1(0); // thread 'main' panicked at my/experiments/src/bin/track_caller.rs:10:13

    // If we mark as `track_caller`, it will report function call itself as panic location.
    #[track_caller]
    pub fn do_not_call_with_zero_v2(val: u64) -> u64 {
        if val == 0 {
            panic!("We told you not to do that");
        }
        val
    }

    // do_not_call_with_zero_v2(0); // thread 'main' panicked at my/experiments/src/bin/track_caller.rs:27:5

    //

    // The track_caller attribute must be on the whole call stack.
    // Every function from the panic, upwards.
    // Otherwise it won't work.

    #[track_caller]
    pub fn do_not_call_with_one_v1(val: u64) -> u64 {
        panic_on_bad_value_v1(val, 1);
        val
    }

    fn panic_on_bad_value_v1(val: u64, bad: u64) {
        if val == bad {
            panic!("We told you not to provide bad value: {}", bad);
        }
    }

    // do_not_call_with_one_v1(1); // thread 'main' panicked at my/experiments/src/bin/track_caller.rs:43:13

    #[track_caller]
    pub fn do_not_call_with_one_v2(val: u64) -> u64 {
        panic_on_bad_value_v2(val, 1);
        val
    }

    #[track_caller]
    fn panic_on_bad_value_v2(val: u64, bad: u64) {
        if val == bad {
            panic!("We told you not to provide bad value: {}", bad);
        }
    }

    // do_not_call_with_one_v2(1); // thread 'main' panicked at my/experiments/src/bin/track_caller.rs:62:5

    //

    // But it doesn't work on closures for now.
    //
    // let do_not_call_with_zero_v3 = #[track_caller]
    // |val: u64| -> u64 {
    //     if val == 0 {
    //         panic!("We told you not to do that");
    //     }
    //     val
    // };
    // do_not_call_with_zero_v3(0);
    //
    //     error[E0658]: `#[track_caller]` on closures is currently unstable
    //     --> my/experiments/src/bin/track_caller.rs:67:36
    //      |
    //   67 |     let do_not_call_with_zero_v3 = #[track_caller]
    //      |                                    ^^^^^^^^^^^^^^^
    //      |
    //      = note: see issue #87417 <https://github.com/rust-lang/rust/issues/87417> for more information

    //

    // The same mechanism that panic uses to get the calling location is available for all.
    // It's called with std::panic::Location::caller().

    /// Calls (prints) the `name` together with the calling location.
    #[track_caller]
    pub fn call_me(name: &str) {
        let caller = std::panic::Location::caller();

        println!(
            "Calling '{name}' from {file}:{line}",
            name = name,
            file = caller.file(),
            line = caller.line(),
        );
    }

    call_me("myh");

    //

    // One important note is that `track_caller` will save location even when backtrace cannot (e.g. release build without debug info or stripped).
    //
    // $ RUST_BACKTRACE=1 cargo run --bin track_caller --release
    // thread 'main' panicked at my/experiments/src/bin/track_caller.rs:27:5:
    // We told you not to do that
    // stack backtrace:
    //    0: rust_begin_unwind
    //    1: core::panicking::panic_fmt
    //    2: track_caller::main
    // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    //
    // But, there was and is counter RFC:
    // https://github.com/rust-lang/rfcs/pull/2091#issuecomment-333306342
    // https://github.com/rust-lang/rfcs/pull/2154

    //

    // Worth noting, it's not always appropriate to mark any panicking function with `track_caller`.
    // It's mostly used to state that some precondition was violated and not for implementation bugs.
    // https://github.com/rust-lang/rfcs/blob/master/text/2091-inline-semantic.md#my-fault-vs-your-fault

    //

    // `std::panic::Location` can be used together with `track_caller` to add location into errors.

    {
        // Without any additional info.

        enum MyError {
            Io(std::io::Error),
        }

        impl std::fmt::Display for MyError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    MyError::Io(err) => write!(f, "{err}"),
                }
            }
        }

        impl From<std::io::Error> for MyError {
            fn from(err: std::io::Error) -> Self {
                MyError::Io(err)
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
            Err(err) => eprintln!("We got err: {err}"),
        }
    }

    {
        // Using backtrace.

        enum MyError {
            Io(std::io::Error, std::backtrace::Backtrace),
        }

        impl std::fmt::Display for MyError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    MyError::Io(err, backtrace) => write!(f, "{err} with backtrace:\n{backtrace}"),
                }
            }
        }

        impl From<std::io::Error> for MyError {
            fn from(err: std::io::Error) -> Self {
                MyError::Io(err, std::backtrace::Backtrace::capture())
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
                eprintln!("We got err: {err}");
            }
        }
    }

    {
        // Using location.

        enum MyError {
            Io(std::io::Error, std::panic::Location<'static>),
        }

        impl std::fmt::Display for MyError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    MyError::Io(err, location) => write!(f, "{err} at {location}"),
                }
            }
        }

        impl From<std::io::Error> for MyError {
            #[track_caller] // <-- magic happens here
            fn from(err: std::io::Error) -> Self {
                MyError::Io(err, *std::panic::Location::caller())
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
            Err(err) => eprintln!("We got err: {err}"),
        }
    }
}
