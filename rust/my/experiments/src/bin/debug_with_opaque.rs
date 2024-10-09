use std::sync::Arc;

fn main() {
    #[derive(Debug, Clone)]
    struct SomeRegularStruct {
        msg: String,
    }

    let some_regular_struct = SomeRegularStruct { msg: "my small msg".to_owned() };

    dbg!(&some_regular_struct);

    //

    #[derive(Debug)]
    struct StructWithBox {
        boxed: Box<SomeRegularStruct>,
    }

    let struct_with_box = StructWithBox { boxed: Box::new(some_regular_struct.clone()) };

    dbg!(&struct_with_box);

    //

    #[derive(Debug)]
    struct StructWithArc {
        arced: Arc<SomeRegularStruct>,
    }

    let struct_with_arc = StructWithArc { arced: Arc::new(some_regular_struct) };

    dbg!(&struct_with_arc);

    //

    #[derive(Debug, Clone)]
    struct StructWithArcWithBox {
        arced: Arc<StructWithBox>,
    }

    let struct_with_arc_with_box = StructWithArcWithBox { arced: Arc::new(struct_with_box) };

    dbg!(&struct_with_arc_with_box);

    //

    impl std::fmt::Display for StructWithArcWithBox {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "msg is {}", self.arced.boxed.msg)
        }
    }

    #[derive(Debug)]
    struct StructWithAnyhowError {
        anyhow_err: anyhow::Error,
    }

    let struct_with_anyhow_error =
        StructWithAnyhowError { anyhow_err: anyhow::anyhow!(struct_with_arc_with_box.clone()) };

    dbg!(&struct_with_anyhow_error);

    // But regular `Debug` is different.
    dbg!(format!("{struct_with_anyhow_error:?}"));

    //

    color_eyre::install().unwrap();

    #[derive(Debug)]
    struct StructWithEyreReport {
        eyre_report: color_eyre::Report,
    }

    let struct_with_eyre_report =
        StructWithEyreReport { eyre_report: color_eyre::eyre::eyre!(struct_with_arc_with_box) };

    dbg!(&struct_with_eyre_report);

    // But regular `Debug` is different.
    dbg!(format!("{struct_with_eyre_report:?}"));

    //

    /// Opaque error wrapper used for better reporting via `Debug` impl.
    /// It includes [`std::backtrace::Backtrace`] and [`std::panic::Location`].
    ///
    /// It is better than to use [`anyhow`] or [`color_eyre`] instead,
    /// as they don't print backtrace nor location in the `Debug` alternate impl.
    /// Also this approach allows to see nested opaque error info (backtrace/location) in the `Debug` impl
    /// which [`anyhow`]/[`color_eyre`] won't show because they show only current error info (backtrace/location).
    #[derive(Debug)]
    pub struct Opaque {
        error: Box<dyn std::error::Error + Send + Sync>,
        #[expect(dead_code, reason = "used only in the `Debug` impl")]
        backtrace: std::backtrace::Backtrace,
        #[expect(dead_code, reason = "used only in the `Debug` impl")]
        location: std::panic::Location<'static>,
    }

    impl std::fmt::Display for Opaque {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Display::fmt(&self.error, f)
        }
    }

    impl std::ops::Deref for Opaque {
        type Target = dyn std::error::Error;

        fn deref(&self) -> &Self::Target {
            self.error.as_ref()
        }
    }

    impl<E: std::error::Error + Send + Sync + 'static> From<E> for Opaque {
        #[track_caller]
        fn from(value: E) -> Self {
            Self {
                error: value.into(),
                backtrace: std::backtrace::Backtrace::capture(),
                location: *std::panic::Location::caller(),
            }
        }
    }
}

// [my/experiments/src/bin/debug_with_opaque.rs:13:5] &some_regular_struct = SomeRegularStruct {
//     msg: "my small msg",
// }
// [my/experiments/src/bin/debug_with_opaque.rs:26:5] &struct_with_box = StructWithBox {
//     boxed: SomeRegularStruct {
//         msg: "my small msg",
//     },
// }
// [my/experiments/src/bin/debug_with_opaque.rs:39:5] &struct_with_arc = StructWithArc {
//     arced: SomeRegularStruct {
//         msg: "my small msg",
//     },
// }
// [my/experiments/src/bin/debug_with_opaque.rs:52:5] &struct_with_arc_with_box = StructWithArcWithBox {
//     arced: StructWithBox {
//         boxed: SomeRegularStruct {
//             msg: "my small msg",
//         },
//     },
// }
// [my/experiments/src/bin/debug_with_opaque.rs:71:5] &struct_with_anyhow_error = StructWithAnyhowError {
//     anyhow_err: StructWithArcWithBox {
//         arced: StructWithBox {
//             boxed: SomeRegularStruct {
//                 msg: "my small msg",
//             },
//         },
//     },
// }
// [my/experiments/src/bin/debug_with_opaque.rs:74:5] format!("{struct_with_anyhow_error:?}") = "StructWithAnyhowError { anyhow_err: msg is my small msg\n\nStack backtrace:\n   0: anyhow::kind::Adhoc::new\n             at /home/mhnap/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/backtrace.rs:27:14\n   1: debug_with_opaque::main\n             at ./my/experiments/src/bin/debug_with_opaque.rs:68:21\n   2: core::ops::function::FnOnce::call_once\n             at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/ops/function.rs:250:5\n   3: std::sys_common::backtrace::__rust_begin_short_backtrace\n             at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/std/src/sys_common/backtrace.rs:155:18\n   4: std::rt::lang_start::{{closure}}\n             at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/std/src/rt.rs:159:18\n   5: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once\n             at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/ops/function.rs:284:13\n   6: std::panicking::try::do_call\n             at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/std/src/panicking.rs:559:40\n   7: std::panicking::try\n             at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/std/src/panicking.rs:523:19\n   8: std::panic::catch_unwind\n             at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/std/src/panic.rs:149:14\n   9: std::rt::lang_start_internal::{{closure}}\n             at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/std/src/rt.rs:141:48\n  10: std::panicking::try::do_call\n             at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/std/src/panicking.rs:559:40\n  11: std::panicking::try\n             at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/std/src/panicking.rs:523:19\n  12: std::panic::catch_unwind\n             at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/std/src/panic.rs:149:14\n  13: std::rt::lang_start_internal\n             at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/std/src/rt.rs:141:20\n  14: std::rt::lang_start\n             at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/std/src/rt.rs:158:17\n  15: main\n  16: __libc_start_call_main\n             at ./csu/../sysdeps/nptl/libc_start_call_main.h:58:16\n  17: __libc_start_main_impl\n             at ./csu/../csu/libc-start.c:392:3\n  18: _start }"
// [my/experiments/src/bin/debug_with_opaque.rs:89:5] &struct_with_eyre_report = StructWithEyreReport {
//     eyre_report: StructWithArcWithBox {
//         arced: StructWithBox {
//             boxed: SomeRegularStruct {
//                 msg: "my small msg",
//             },
//         },
//     },
// }
// [my/experiments/src/bin/debug_with_opaque.rs:92:5] format!("{struct_with_eyre_report:?}") = "StructWithEyreReport { eyre_report: \n   0: \u{1b}[91mmsg is my small msg\u{1b}[0m\n\nLocation:\n   \u{1b}[35mmy/experiments/src/bin/debug_with_opaque.rs\u{1b}[0m:\u{1b}[35m86\u{1b}[0m\n\n  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ BACKTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n  \u{1b}[96m                              ⋮ 5 frames hidden ⋮                               \u{1b}[0m\n   6: \u{1b}[91mdebug_with_opaque::main\u{1b}[0m\u{1b}[90m::he07f3ad7c74cc731\u{1b}[0m\n      at \u{1b}[35m/home/mhnap/projects/DIY/rust/my/experiments/src/bin/debug_with_opaque.rs\u{1b}[0m:\u{1b}[35m86\u{1b}[0m\n   7: \u{1b}[32mcore::ops::function::FnOnce::call_once\u{1b}[0m\u{1b}[90m::hede3002440a8d65d\u{1b}[0m\n      at \u{1b}[35m/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/ops/function.rs\u{1b}[0m:\u{1b}[35m250\u{1b}[0m\n  \u{1b}[96m                              ⋮ 16 frames hidden ⋮                              \u{1b}[0m\n\nRun with COLORBT_SHOW_HIDDEN=1 environment variable to disable frame filtering.\nRun with RUST_BACKTRACE=full to include source snippets. }"
