use error_stack::{ensure, Result, ResultExt};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("InnerBody Error")]
struct InnerBodyError;

fn inner_body() -> Result<(), InnerBodyError> {
    ensure!(false, InnerBodyError);

    Ok(())
}

#[derive(Debug, Error)]
enum BodyError {
    #[error("Body Error: Inner")]
    InnerBodyError,
    #[error("Body Error: Outer")]
    OuterBodyError,
}

fn body() -> Result<(), BodyError> {
    inner_body()
        .change_context(BodyError::InnerBodyError)
        .attach_printable("there's something more I want to tell you")?;

    Ok(())
}

#[derive(Debug, Error)]
#[error("App Error")]
struct AppError;
fn main() -> Result<(), AppError> {
    body().change_context(AppError)?;
    Ok(())
}
