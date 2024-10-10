use std::error::Error;

fn main() {
    // With [`thiserror`]:

    {
        // Error would be duplicated in `Display` and in `Error::source()`.
        #[derive(Debug, thiserror::Error)]
        enum MyError {
            #[error("{0}")]
            Io(#[from] std::io::Error),
        }

        let err: MyError = std::io::Error::new(std::io::ErrorKind::Other, "io error").into();
        assert_eq!(err.to_string(), "io error");
        assert_eq!(err.source().unwrap().to_string(), "io error");

        // Recursive expansion of thiserror::Error macro
        // ==============================================
        //
        // #[allow(unused_qualifications)]
        // impl std::error::Error for MyError {
        //     fn source(&self) -> ::core::option::Option<&(dyn std::error::Error + 'static)> {
        //         use thiserror::__private::AsDynError as _;
        //         #[allow(deprecated)]
        //         match self {
        //             MyError::Io { 0: source, .. } => ::core::option::Option::Some(source.as_dyn_error()),
        //         }
        //     }
        // }
        //
        // #[allow(unused_qualifications)]
        // impl ::core::fmt::Display for MyError {
        //     fn fmt(&self, __formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        //         use thiserror::__private::AsDisplay as _;
        //         #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
        //         match self {
        //             MyError::Io(_0) => {
        //                 ::core::write!(__formatter, "{field__0}", field__0 = _0.as_display())
        //             }
        //         }
        //     }
        // }
        //
        // #[allow(unused_qualifications)]
        // impl ::core::convert::From<std::io::Error> for MyError {
        //     #[allow(deprecated)]
        //     fn from(source: std::io::Error) -> Self {
        //         MyError::Io { 0: source }
        //     }
        // }
    }

    {
        // Errors may use `#[error(transparent)]` to forward the source and Display methods
        // straight through to an underlying error without adding an additional message.
        #[derive(Debug, thiserror::Error)]
        enum MyError {
            #[error(transparent)]
            Io(#[from] std::io::Error),
        }

        let err: MyError = std::io::Error::new(std::io::ErrorKind::Other, "io error").into();
        assert_eq!(err.to_string(), "io error");
        assert!(err.source().is_none());

        // Recursive expansion of thiserror::Error macro
        // ==============================================
        //
        // #[allow(unused_qualifications)]
        // impl std::error::Error for MyError {
        //     fn source(&self) -> ::core::option::Option<&(dyn std::error::Error + 'static)> {
        //         use thiserror::__private::AsDynError as _;
        //         #[allow(deprecated)]
        //         match self {
        //             MyError::Io { 0: transparent } => std::error::Error::source(transparent.as_dyn_error()),
        //         }
        //     }
        // }
        //
        // #[allow(unused_qualifications)]
        // impl ::core::fmt::Display for MyError {
        //     fn fmt(&self, __formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        //         #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
        //         match self {
        //             MyError::Io(_0) => ::core::fmt::Display::fmt(_0, __formatter),
        //         }
        //     }
        // }
        //
        // #[allow(unused_qualifications)]
        // impl ::core::convert::From<std::io::Error> for MyError {
        //     #[allow(deprecated)]
        //     fn from(source: std::io::Error) -> Self {
        //         MyError::Io { 0: source }
        //     }
        // }
    }

    //

    // With [`derive_more`]:

    {
        // Error would be duplicated in `Display` and in `Error::source()`.
        #[derive(Debug, derive_more::Error, derive_more::Display, derive_more::From)]
        enum MyError {
            Io(std::io::Error),
        }

        let err: MyError = std::io::Error::new(std::io::ErrorKind::Other, "io error").into();
        assert_eq!(err.to_string(), "io error");
        assert_eq!(err.source().unwrap().to_string(), "io error");

        // Recursive expansion of derive_more::Error macro
        // ================================================
        //
        // #[automatically_derived]
        // impl derive_more::Error for MyError {
        //     fn source(&self) -> Option<&(dyn derive_more::Error + 'static)> {
        //         use derive_more::__private::AsDynError;
        //         match self {
        //             MyError::Io(source) => Some(source.as_dyn_error()),
        //         }
        //     }
        // }
        //
        // #[automatically_derived]
        // impl derive_more::Display for MyError {
        //     fn fmt(
        //         &self,
        //         __derive_more_f: &mut derive_more::core::fmt::Formatter<'_>,
        //     ) -> derive_more::core::fmt::Result {
        //         match self {
        //             Self::Io(_0) => derive_more::core::fmt::Display::fmt(_0, __derive_more_f),
        //         }
        //     }
        // }
        //
        // #[automatically_derived]
        // impl derive_more::From<(std::io::Error)> for MyError {
        //     #[inline]
        //     fn from(value: (std::io::Error)) -> Self {
        //         MyError::Io(value)
        //     }
        // }
    }

    // Not possible to get the same behaviour for [`derive_more`] as [`thiserror`]'s `#[error(transparent)]` provide.
}
