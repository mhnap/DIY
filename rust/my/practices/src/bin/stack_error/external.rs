// External error from some sub-crate we nothing knows about.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Other")]
    Other,
}

pub fn read_file(path: &std::path::Path) -> Result<Vec<u8>, Error> {
    Ok(std::fs::read(path)?)
}
