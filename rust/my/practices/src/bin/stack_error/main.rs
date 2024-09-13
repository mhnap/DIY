use stack_error::StackError;

macro_rules! print_err {
    ($err:expr) => {
        eprintln!("----- {} at {}:{}:{} -----", stringify!($err), file!(), line!(), column!());
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

mod external;
mod internal;
mod my;
mod stack_error;

fn main() {
    let res = my::read_two_files("Cargo.toml".as_ref(), "file2".as_ref());
    match res {
        Ok(buf) => println!("We read buf: {buf:?}"),
        Err(err) => {
            print_err!(err);

            // Print err as `StackError`.
            println!("Stack Error:");
            let mut buf = vec![];
            err.format(0, &mut buf);
            dbg!(buf);
        }
    }
}
