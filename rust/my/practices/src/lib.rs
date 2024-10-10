#[macro_export]
macro_rules! print_err {
    ($err:expr) => {
        eprintln!("----- {} at {}:{}:{} -----", stringify!($err), file!(), line!(), column!());
        eprintln!("Display:\n{}", $err);
        eprintln!("Display alternate:\n{:#}", $err);
        eprintln!("Debug:\n{:?}", $err);
        eprintln!("Debug alternate:\n{:#?}", $err);
        my_practices::error_chain(&$err);
        eprintln!("------------------------------------------------------------------");
    };
}

pub fn error_chain(e: &impl std::error::Error) {
    let mut current = e.source();
    while let Some(cause) = current {
        eprintln!("Caused by: {cause}, dbg: {cause:?}");
        current = cause.source();
    }
}
