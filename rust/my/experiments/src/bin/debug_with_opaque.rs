macro_rules! print_err {
    ($err:expr) => {
        eprintln!("----- {} at {}:{}:{} -----", stringify!($err), file!(), line!(), column!());
        eprintln!("Display:\n{}", $err);
        eprintln!("Display alternate:\n{:#}", $err);
        eprintln!("Debug:\n{:?}", $err);
        eprintln!("Debug alternate:\n{:#?}", $err);
        error_chain(&$err);
        eprintln!("------------------------------------------------------------------");
    };
}

fn error_chain(e: &impl std::error::Error) {
    let mut current = e.source();
    while let Some(cause) = current {
        eprintln!("Caused by: {cause}, dbg: {cause:?}");
        current = cause.source();
    }
}

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
        arced: std::sync::Arc<SomeRegularStruct>,
    }

    let struct_with_arc = StructWithArc { arced: std::sync::Arc::new(some_regular_struct) };

    dbg!(&struct_with_arc);

    //

    #[derive(Debug, Clone)]
    struct StructWithArcWithBox {
        arced: std::sync::Arc<StructWithBox>,
    }

    let struct_with_arc_with_box =
        StructWithArcWithBox { arced: std::sync::Arc::new(struct_with_box) };

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
        #[expect(dead_code, reason = "used only in the `Debug` impl")]
        type_name: &'static str,
    }

    impl std::fmt::Display for Opaque {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Display::fmt(&self.error, f)
        }
    }

    /// Used inside [`thiserror`].
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
                type_name: std::any::type_name::<E>(),
            }
        }
    }

    #[derive(Debug, thiserror::Error)]
    enum MyError1 {
        #[error("I/O")]
        Io(#[source] std::io::Error),
    }

    let error = MyError1::Io(std::io::Error::new(std::io::ErrorKind::Other, "other error"));
    print_err!(error);

    let error: Opaque = error.into();
    // [`Opaque`] itself doesn't implement [`std::error::Error`].
    // print_err!(error);
    //
    // error[E0277]: the trait bound `Opaque: std::error::Error` is not satisfied
    //    --> my/experiments/src/bin/debug_with_opaque.rs:11:21
    //     |
    // 11  |         error_chain(&$err);
    //     |         ----------- ^^^^^ the trait `std::error::Error` is not implemented for `Opaque`
    //     |         |
    //     |         required by a bound introduced by this call
    // ...
    // 163 |     print_err!(error);
    //     |     ----------------- in this macro invocation

    #[derive(Debug, thiserror::Error)]
    enum MyError2 {
        #[error(transparent)]
        Opaque(Opaque),

        #[error("Another error")]
        Another,
    }

    let my_error = MyError2::Another;
    print_err!(my_error);

    // But together with [`thiserror`] it "can" because of [`Deref`] impl.
    let my_error = MyError2::Opaque(error);
    print_err!(my_error);

    // But worth noting [`Opaque`] size much bigger than [`anyhow::Error`]
    dbg!(size_of::<Opaque>()); // 104
}

// [my/experiments/src/bin/debug_with_opaque.rs:32:5] &some_regular_struct = SomeRegularStruct {
//     msg: "my small msg",
// }
// [my/experiments/src/bin/debug_with_opaque.rs:43:5] &struct_with_box = StructWithBox {
//     boxed: SomeRegularStruct {
//         msg: "my small msg",
//     },
// }
// [my/experiments/src/bin/debug_with_opaque.rs:54:5] &struct_with_arc = StructWithArc {
//     arced: SomeRegularStruct {
//         msg: "my small msg",
//     },
// }
// [my/experiments/src/bin/debug_with_opaque.rs:65:5] &struct_with_arc_with_box = StructWithArcWithBox {
//     arced: StructWithBox {
//         boxed: SomeRegularStruct {
//             msg: "my small msg",
//         },
//     },
// }
// [my/experiments/src/bin/debug_with_opaque.rs:83:5] &struct_with_anyhow_error = StructWithAnyhowError {
//     anyhow_err: StructWithArcWithBox {
//         arced: StructWithBox {
//             boxed: SomeRegularStruct {
//                 msg: "my small msg",
//             },
//         },
//     },
// }
// [my/experiments/src/bin/debug_with_opaque.rs:86:5] format!("{struct_with_anyhow_error:?}") = "StructWithAnyhowError { anyhow_err: msg is my small msg }"
// [my/experiments/src/bin/debug_with_opaque.rs:100:5] &struct_with_eyre_report = StructWithEyreReport {
//     eyre_report: StructWithArcWithBox {
//         arced: StructWithBox {
//             boxed: SomeRegularStruct {
//                 msg: "my small msg",
//             },
//         },
//     },
// }
// [my/experiments/src/bin/debug_with_opaque.rs:103:5] format!("{struct_with_eyre_report:?}") = "StructWithEyreReport { eyre_report: \n   0: \u{1b}[91mmsg is my small msg\u{1b}[0m\n\nLocation:\n   \u{1b}[35mmy/experiments/src/bin/debug_with_opaque.rs\u{1b}[0m:\u{1b}[35m98\u{1b}[0m\n\nBacktrace omitted. Run with RUST_BACKTRACE=1 environment variable to display it.\nRun with RUST_BACKTRACE=full to include source snippets. }"
// ----- error at my/experiments/src/bin/debug_with_opaque.rs:159:5 -----
// Display:
// I/O
// Display alternate:
// I/O
// Debug:
// Io(Custom { kind: Other, error: "other error" })
// Debug alternate:
// Io(
//     Custom {
//         kind: Other,
//         error: "other error",
//     },
// )
// Caused by: other error, dbg: Custom { kind: Other, error: "other error" }
// ------------------------------------------------------------------
// ----- my_error at my/experiments/src/bin/debug_with_opaque.rs:186:5 -----
// Display:
// Another error
// Display alternate:
// Another error
// Debug:
// Another
// Debug alternate:
// Another
// ------------------------------------------------------------------
// ----- my_error at my/experiments/src/bin/debug_with_opaque.rs:190:5 -----
// Display:
// I/O
// Display alternate:
// I/O
// Debug:
// Opaque(Opaque { error: Io(Custom { kind: Other, error: "other error" }), backtrace: <disabled>, location: Location { file: "my/experiments/src/bin/debug_with_opaque.rs", line: 161, col: 31 }, type_name: "debug_with_opaque::main::MyError1" })
// Debug alternate:
// Opaque(
//     Opaque {
//         error: Io(
//             Custom {
//                 kind: Other,
//                 error: "other error",
//             },
//         ),
//         backtrace: <disabled>,
//         location: Location {
//             file: "my/experiments/src/bin/debug_with_opaque.rs",
//             line: 161,
//             col: 31,
//         },
//         type_name: "debug_with_opaque::main::MyError1",
//     },
// )
// Caused by: other error, dbg: Custom { kind: Other, error: "other error" }
// ------------------------------------------------------------------
// [my/experiments/src/bin/debug_with_opaque.rs:191:5] size_of::<Opaque>() = 104
