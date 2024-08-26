use crate::stack_error::StackError;

// Internal error from our sub-crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error")]
    Io(#[source] std::io::Error, std::panic::Location<'static>),

    #[error("Other")]
    Other,
}

impl From<std::io::Error> for Error {
    #[track_caller]
    fn from(value: std::io::Error) -> Self {
        Self::Io(value, *std::panic::Location::caller())
    }
}

impl StackError for Error {
    fn next(&self) -> Option<&dyn StackError> {
        match self {
            Error::Io(..) | Error::Other => None,
        }
    }

    fn location(&self) -> Option<std::panic::Location<'static>> {
        match self {
            Error::Io(_, location) => Some(*location),
            Error::Other => None,
        }
    }
}

pub fn read_file(path: &std::path::Path) -> Result<Vec<u8>, Error> {
    Ok(std::fs::read(path)?)
}
