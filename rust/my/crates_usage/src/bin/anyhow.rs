use anyhow::Context;

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
            print_err!(err);
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
            print_err!(err);
        }
    }
}
