// https://doc.rust-lang.org/cargo/reference/profiles.html#strip

fn main() {
    foo1();
}

fn foo1() {
    foo2();
}

fn foo2() {
    panic!("Oh noo...")
}

// RUST_BACKTRACE=1 is set.

// strip = "none" case:
//
// thread 'main' panicked at my/experiments/src/bin/backtrace_with_strip.rs:12:5:
// Oh noo...
// stack backtrace:
//    0: rust_begin_unwind
//              at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/std/src/panicking.rs:645:5
//    1: core::panicking::panic_fmt
//              at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/panicking.rs:72:14
//    2: backtrace_with_strip::foo2
//              at ./my/experiments/src/bin/backtrace_with_strip.rs:12:5
//    3: backtrace_with_strip::foo1
//              at ./my/experiments/src/bin/backtrace_with_strip.rs:8:5
//    4: backtrace_with_strip::main
//              at ./my/experiments/src/bin/backtrace_with_strip.rs:4:5
//    5: core::ops::function::FnOnce::call_once
//              at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/function.rs:250:5
// note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

// strip = "debuginfo"
//
// thread 'main' panicked at my/experiments/src/bin/backtrace_with_strip.rs:12:5:
// Oh noo...
// stack backtrace:
//    0: rust_begin_unwind
//    1: core::panicking::panic_fmt
//    2: backtrace_with_strip::foo2
//    3: backtrace_with_strip::foo1
//    4: backtrace_with_strip::main
//    5: core::ops::function::FnOnce::call_once
// note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

// strip = "symbols"
//
// thread 'main' panicked at my/experiments/src/bin/backtrace_with_strip.rs:12:5:
// Oh noo...
// stack backtrace:
// note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
