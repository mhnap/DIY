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

pub mod fibonacci {
    #[inline]
    pub fn number_recursive(n: u8) -> usize {
        if n < 2 {
            n as usize
        } else {
            number_recursive(n - 1) + number_recursive(n - 2)
        }
    }

    #[inline]
    pub fn number_iterative(n: u8) -> usize {
        let mut prev_number = 0;
        let mut number = if n == 0 { 0 } else { 1 };
        for _ in 1..n {
            let tmp = number;
            number += prev_number;
            prev_number = tmp;
        }
        number
    }

    #[inline]
    pub fn sequence(n: u8) -> Vec<usize> {
        if n == 0 {
            return vec![0];
        }
        let mut sequence = vec![0; n as usize + 1];
        sequence[1] = 1;
        for i in 2..=n as usize {
            sequence[i] = sequence[i - 1] + sequence[i - 2];
        }
        sequence
    }
}
