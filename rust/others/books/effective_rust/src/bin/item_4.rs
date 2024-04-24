// https://www.lurklurk.org/effective-rust/errors.html
// Item 4: Prefer idiomatic Error variants

use std::{error::Error, fmt::Display};

fn main() {
    //  The E type parameter for a Result doesn't have to be a type that implements Error, but it's a common convention that allows wrappers to express appropriate trait bounds – so **prefer to implement Error for your error types**.

    // Writing a complete error type can involve a fair amount of boilerplate; **consider using the thiserror crate** to help with this, as it reduces the effort involved without adding an extra runtime dependency.

    //

    // The first approach to nested errors threw away all of the sub-error detail, just preserving some string output (format!("{:?}", err)).
    // The second approach preserved the full type information for all possible sub-errors, but required a full enumeration of all possible types of sub-error.

    // This raises the question: is there a half-way house between these two approaches, preserving sub-error information without needing to manually include every possible error type?

    // Encoding the sub-error information as a trait object avoids the need for an enum variant for every possibility, but erases the details of the specific underlying error types.
    // The receiver of such an object would have access to the methods of the Error trait – display(), debug() and source() in turn – but wouldn't know the original static type of the sub-error.

    #[derive(Debug)]
    pub enum WrappedError {
        Wrapped(Box<dyn Error>),
        General(String),
    }

    impl Display for WrappedError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                WrappedError::Wrapped(wrapped) => write!(f, "{}", wrapped),
                WrappedError::General(general) => write!(f, "{}", general),
            }
        }
    }

    impl Error for WrappedError {}

    // impl<E: 'static + Error> From<E> for WrappedError {
    //     fn from(value: E) -> Self {
    //         todo!()
    //     }
    // }
    //     error[E0119]: conflicting implementations of trait `From<WrappedError>` for type `WrappedError`
    //     --> others/books/effective_rust/src/bin/item_4.rs:38:5
    //      |
    //   38 |     impl<E: 'static + Error> From<E> for WrappedError {
    //      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //      |
    //      = note: conflicting implementation in crate `core`:
    //              - impl<T> From<T> for T;

    let err1 = WrappedError::General("err".into());
    dbg!(&err1);

    let err2 = WrappedError::Wrapped(Box::new(err1));
    dbg!(&err2);

    // let dyn_err: Box<dyn Error> = Box::new(err2);
    // let err3 = WrappedError::from(dyn_err);
    // dbg!(&err3);

    // It turns out that this is possible, but it's surprisingly subtle.
    // Part of the difficulty comes from the object safety constraints on trait objects (Item 12), but Rust's coherence rules also come into play, which (roughly) say that there can be at most one implementation of a trait for a type.

    // A putative WrappedError would naively be expected to both implement the Error trait, and also to implement the From<Error> trait to allow sub-errors to be easily wrapped.
    // That means that a WrappedError can be created from an inner WrappedError, as WrappedError implements Error, and that clashes with the blanket reflexive implementation of From.

    // David Tolnay's anyhow is a crate that has already solved these problems, and which adds other helpful features (such as stack traces) besides.
    // As a result, it is rapidly becoming the standard recommendation for error handling – a recommendation seconded here: **consider using the anyhow crate for error handling in applications**.

    //

    // The final advice of the previous section included the qualification "…for error handling in applications".
    // That's because there's often a distinction between code that's written for re-use in a library, and code that forms a top-level application.

    // Code that's written for a library can't predict the environment in which the code is used, so it's preferable to emit concrete, detailed error information, and leave the caller to figure out how to use that information.
    // This leans towards the enum-style nested errors described previously (and also avoids a dependency on anyhow in the public API of the library, cf. Item 24).

    // However, application code typically needs to concentrate more on how to present errors to the user.
    // It also potentially has to cope with all of the different error types emitted by all of the libraries that are present in its dependency graph (Item 25).
    // As such, a more dynamic error type (such as anyhow::Error) makes error handling simpler and more consistent across the application.

    //

    // This item has covered a lot of ground, so a summary is in order:
    // The standard Error trait requires little of you, so prefer to implement it for your error types.
    // When dealing with heterogeneous underlying error types, decide whether preserving those types is needed.
    //  If not, consider using anyhow to wrap sub-errors in application code.
    //  If they are needed, encode them in an enum and provide conversions. Consider using thiserror to help with this.
    // Consider using the anyhow crate for convenient, idiomatic error handling.
}
