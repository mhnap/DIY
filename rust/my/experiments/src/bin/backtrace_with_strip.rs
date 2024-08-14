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

// Without RUST_BACKTRACE.

// strip = "none"
//
// thread 'main' panicked at my/experiments/src/bin/backtrace_with_strip.rs:12:5:
// Oh noo...
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

// strip = "debuginfo"
//
// thread 'main' panicked at my/experiments/src/bin/backtrace_with_strip.rs:12:5:
// Oh noo...
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

// strip = "symbols"
//
// thread 'main' panicked at my/experiments/src/bin/backtrace_with_strip.rs:12:5:
// Oh noo...
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

//

// RUST_BACKTRACE=1 is set.

// strip = "none"
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

//

// RUST_BACKTRACE=full is set.

// strip = "none"
//
// thread 'main' panicked at my/experiments/src/bin/backtrace_with_strip.rs:12:5:
// Oh noo...
// stack backtrace:
//    0:     0x6247c9a6c4bd - std::backtrace_rs::backtrace::libunwind::trace::h159528d8c188509b
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/std/src/../../backtrace/src/backtrace/libunwind.rs:116:5
//    1:     0x6247c9a6c4bd - std::backtrace_rs::backtrace::trace_unsynchronized::hd29c4432a5435e12
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
//    2:     0x6247c9a6c4bd - std::sys::backtrace::_print_fmt::h6d3b5ce224cd9670
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/std/src/sys/backtrace.rs:66:9
//    3:     0x6247c9a6c4bd - <std::sys::backtrace::BacktraceLock::print::DisplayBacktrace as core::fmt::Display>::fmt::ha817fa35f844343c
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/std/src/sys/backtrace.rs:39:26
//    4:     0x6247c9a8914b - core::fmt::rt::Argument::fmt::h5ea97e3e0016eb0c
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/core/src/fmt/rt.rs:173:76
//    5:     0x6247c9a8914b - core::fmt::write::h18792a013c06cf68
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/core/src/fmt/mod.rs:1178:21
//    6:     0x6247c9a69e83 - std::io::Write::write_fmt::h490d1250ab64f23a
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/std/src/io/mod.rs:1823:15
//    7:     0x6247c9a6d672 - std::sys::backtrace::BacktraceLock::print::hb1bb7734665d317c
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/std/src/sys/backtrace.rs:42:9
//    8:     0x6247c9a6d672 - std::panicking::default_hook::{{closure}}::h1af1ba80fbbab256
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/std/src/panicking.rs:266:22
//    9:     0x6247c9a6d2de - std::panicking::default_hook::h99117ef53c55281d
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/std/src/panicking.rs:293:9
//   10:     0x6247c9a6de8f - std::panicking::rust_panic_with_hook::hfbd77db16f1d98f6
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/std/src/panicking.rs:797:13
//   11:     0x6247c9a6dbd3 - std::panicking::begin_panic_handler::{{closure}}::ha71fdd1b99649238
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/std/src/panicking.rs:664:13
//   12:     0x6247c9a6c9a9 - std::sys::backtrace::__rust_end_short_backtrace::h864a6e233bda5366
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/std/src/sys/backtrace.rs:170:18
//   13:     0x6247c9a6d894 - rust_begin_unwind
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/std/src/panicking.rs:662:5
//   14:     0x6247c9a883c3 - core::panicking::panic_fmt::h3badd9e9c4581a42
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/core/src/panicking.rs:74:14
//   15:     0x6247c9a4d76d - backtrace_with_strip::foo2::hecd656d3b7069cc2
//                                at /home/mhnap/projects/DIY/rust/my/experiments/src/bin/backtrace_with_strip.rs:12:5
//   16:     0x6247c9a4d736 - backtrace_with_strip::foo1::he7cb8c262f6fae85
//                                at /home/mhnap/projects/DIY/rust/my/experiments/src/bin/backtrace_with_strip.rs:8:5
//   17:     0x6247c9a4d726 - backtrace_with_strip::main::ha9364d652f5bd07d
//                                at /home/mhnap/projects/DIY/rust/my/experiments/src/bin/backtrace_with_strip.rs:4:5
//   18:     0x6247c9a4d6db - core::ops::function::FnOnce::call_once::h1b9ccf7dd484328b
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/core/src/ops/function.rs:250:5
//   19:     0x6247c9a4d7de - std::sys::backtrace::__rust_begin_short_backtrace::hf19369b518a77b44
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/std/src/sys/backtrace.rs:154:18
//   20:     0x6247c9a4d851 - std::rt::lang_start::{{closure}}::h477322d35fd4b7bb
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/std/src/rt.rs:164:18
//   21:     0x6247c9a67e10 - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::h30082e2a4033c7d6
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/core/src/ops/function.rs:284:13
//   22:     0x6247c9a67e10 - std::panicking::try::do_call::h7dc52b3f964af0a2
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/std/src/panicking.rs:554:40
//   23:     0x6247c9a67e10 - std::panicking::try::h6236a12d08e1042b
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/std/src/panicking.rs:518:19
//   24:     0x6247c9a67e10 - std::panic::catch_unwind::hf1904911f068ac6e
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/std/src/panic.rs:345:14
//   25:     0x6247c9a67e10 - std::rt::lang_start_internal::{{closure}}::hea8416979721d430
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/std/src/rt.rs:143:48
//   26:     0x6247c9a67e10 - std::panicking::try::do_call::h7f11a796c9fc9114
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/std/src/panicking.rs:554:40
//   27:     0x6247c9a67e10 - std::panicking::try::h0461c37a9d160849
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/std/src/panicking.rs:518:19
//   28:     0x6247c9a67e10 - std::panic::catch_unwind::h94fa0dd3c12cd3d8
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/std/src/panic.rs:345:14
//   29:     0x6247c9a67e10 - std::rt::lang_start_internal::h51519c1f1208371e
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/std/src/rt.rs:143:20
//   30:     0x6247c9a4d82a - std::rt::lang_start::ha2432aa0f57f0cd8
//                                at /rustc/e57f3090aec33cdbf66063c866afaa5e1e78b9bb/library/std/src/rt.rs:163:17
//   31:     0x6247c9a4d78e - main
//   32:     0x71657ba29d90 - __libc_start_call_main
//                                at ./csu/../sysdeps/nptl/libc_start_call_main.h:58:16
//   33:     0x71657ba29e40 - __libc_start_main_impl
//                                at ./csu/../csu/libc-start.c:392:3
//   34:     0x6247c9a4d5e5 - _start
//   35:                0x0 - <unknown>

// strip = "debuginfo"
//
// thread 'main' panicked at my/experiments/src/bin/backtrace_with_strip.rs:12:5:
// Oh noo...
// stack backtrace:
//    0:     0x6029309f24ad - <std::sys::backtrace::BacktraceLock::print::DisplayBacktrace as core::fmt::Display>::fmt::ha817fa35f844343c
//    1:     0x602930a0f13b - core::fmt::write::h18792a013c06cf68
//    2:     0x6029309efe73 - std::io::Write::write_fmt::h490d1250ab64f23a
//    3:     0x6029309f3662 - std::panicking::default_hook::{{closure}}::h1af1ba80fbbab256
//    4:     0x6029309f32ce - std::panicking::default_hook::h99117ef53c55281d
//    5:     0x6029309f3e7f - std::panicking::rust_panic_with_hook::hfbd77db16f1d98f6
//    6:     0x6029309f3bc3 - std::panicking::begin_panic_handler::{{closure}}::ha71fdd1b99649238
//    7:     0x6029309f2999 - std::sys::backtrace::__rust_end_short_backtrace::h864a6e233bda5366
//    8:     0x6029309f3884 - rust_begin_unwind
//    9:     0x602930a0e3b3 - core::panicking::panic_fmt::h3badd9e9c4581a42
//   10:     0x6029309d376d - backtrace_with_strip::foo2::hdb9cc71a2e83fd0a
//   11:     0x6029309d3736 - backtrace_with_strip::foo1::h11bc266e34e111d7
//   12:     0x6029309d3726 - backtrace_with_strip::main::h40e43a258db10fb7
//   13:     0x6029309d385b - core::ops::function::FnOnce::call_once::hc828f1aa69c66648
//   14:     0x6029309d379e - std::sys::backtrace::__rust_begin_short_backtrace::hfaf31b79038bcb4b
//   15:     0x6029309d3701 - std::rt::lang_start::{{closure}}::h751e0f8a423b5ed9
//   16:     0x6029309ede00 - std::rt::lang_start_internal::h51519c1f1208371e
//   17:     0x6029309d36da - std::rt::lang_start::heae7e2c7b0dbd4b0
//   18:     0x6029309d378e - main
//   19:     0x79e3e4629d90 - __libc_start_call_main
//                                at ./csu/../sysdeps/nptl/libc_start_call_main.h:58:16
//   20:     0x79e3e4629e40 - __libc_start_main_impl
//                                at ./csu/../csu/libc-start.c:392:3
//   21:     0x6029309d35d5 - _start
//   22:                0x0 - <unknown>

// strip = "symbols"
//
// thread 'main' panicked at my/experiments/src/bin/backtrace_with_strip.rs:12:5:
// Oh noo...
// stack backtrace:
//    0:     0x6200d1a474ad - <unknown>
//    1:     0x6200d1a6413b - <unknown>
//    2:     0x6200d1a44e73 - <unknown>
//    3:     0x6200d1a48662 - <unknown>
//    4:     0x6200d1a482ce - <unknown>
//    5:     0x6200d1a48e7f - <unknown>
//    6:     0x6200d1a48bc3 - <unknown>
//    7:     0x6200d1a47999 - <unknown>
//    8:     0x6200d1a48884 - <unknown>
//    9:     0x6200d1a633b3 - <unknown>
//   10:     0x6200d1a2879d - <unknown>
//   11:     0x6200d1a28766 - <unknown>
//   12:     0x6200d1a28756 - <unknown>
//   13:     0x6200d1a2882b - <unknown>
//   14:     0x6200d1a286ae - <unknown>
//   15:     0x6200d1a28721 - <unknown>
//   16:     0x6200d1a42e00 - <unknown>
//   17:     0x6200d1a286fa - <unknown>
//   18:     0x6200d1a287be - <unknown>
//   19:     0x709f73229d90 - __libc_start_call_main
//                                at ./csu/../sysdeps/nptl/libc_start_call_main.h:58:16
//   20:     0x709f73229e40 - __libc_start_main_impl
//                                at ./csu/../csu/libc-start.c:392:3
//   21:     0x6200d1a285d5 - <unknown>
//   22:                0x0 - <unknown>
