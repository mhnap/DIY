use crate::{external, internal, stack_error::StackError};

// Our own error from this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("External error")]
    External(#[source] external::Error, std::panic::Location<'static>),

    #[error("Internal error")]
    Internal(#[source] internal::Error, std::panic::Location<'static>),

    #[error("Other")]
    Other,
}

impl From<external::Error> for Error {
    #[track_caller]
    fn from(value: external::Error) -> Self {
        Self::External(value, *std::panic::Location::caller())
    }
}

impl From<internal::Error> for Error {
    #[track_caller]
    fn from(value: internal::Error) -> Self {
        Self::Internal(value, *std::panic::Location::caller())
    }
}

impl StackError for Error {
    fn next(&self) -> Option<&dyn StackError> {
        match self {
            Error::Internal(source, _) => Some(source),
            Error::External(..) | Error::Other => None,
        }
    }

    fn location(&self) -> Option<std::panic::Location<'static>> {
        match self {
            Error::External(_, location) | Error::Internal(_, location) => Some(*location),
            Error::Other => None,
        }
    }
}

pub fn read_two_files(path1: &std::path::Path, path2: &std::path::Path) -> Result<Vec<u8>, Error> {
    let mut buf1 = external::read_file(path1)?;
    let buf2 = internal::read_file(path2)?;
    buf1.extend(buf2);
    Ok(buf1)
}
