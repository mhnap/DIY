fn main() {
    color_backtrace::install();

    // Can print own backtrace.
    let printer = color_backtrace::BacktracePrinter::default();

    // But cannot be used with `std::backtrace::Backtrace`.
    //
    // fn std_backtrace() -> std::backtrace::Backtrace {
    //     std::backtrace::Backtrace::capture()
    // }
    //
    // let str = printer.format_trace_to_string(&std_backtrace());
    //
    //     error[E0308]: mismatched types
    //     --> my/crates_usage/src/bin/color_backtrace.rs:12:46
    //      |
    //  12  |     let str = printer.format_trace_to_string(&std_backtrace());
    //      |                       ---------------------- ^^^^^^^^^^^^^^^^ expected `backtrace::capture::Backtrace`, found `Backtrace`
    //      |                       |
    //      |                       arguments to this method are incorrect
    //      |
    //      = note: `Backtrace` and `backtrace::capture::Backtrace` have similar names, but are actually distinct types

    fn backtrace() -> backtrace::Backtrace {
        backtrace::Backtrace::new()
    }

    let str = printer.format_trace_to_string(&backtrace()).unwrap();
    println!("{str}");

    //

    // Or good old panic.

    None::<bool>.unwrap();
}
