use anyhow::Context;

macro_rules! print_err {
    ($err:expr) => {
        println!("----- {} -----", stringify!($err));
        println!("Display:\n{}", $err);
        println!("Display alternate:\n{:#}", $err);
        println!("Debug:\n{:?}", $err);
        println!("Debug alternate:\n{:#?}", $err);
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
        Err(err) => eprintln!("We got err: {err}"),
    }

    // Attach context to help the person troubleshooting the error understand where things went wrong.

    let res = read_env_var("MY_ENV").context("Failed to read env var");
    match res {
        Ok(buf) => println!("We read env var: {buf:?}"),
        Err(err) => eprintln!("We got err: {err}"),
    }

    // Above there would be used `Display` impl for printing, and thus only the error itself would be reported:
    // We got err: Failed to read env var

    // But can use different formatting with more information (source errors and backtrace).

    let err = read_env_var("MY_ENV")
        .context("Failed to read env var")
        .context("Outer context")
        .unwrap_err();

    print_err!(err);
}
