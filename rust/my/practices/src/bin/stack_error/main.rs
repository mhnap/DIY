use my_practices::print_err;
use stack_error::StackError;

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
