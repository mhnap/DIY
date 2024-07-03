// https://github.com/tokio-rs/tracing/blob/master/examples/examples/tokio-spawny-thing.rs
/// This is a example showing how information is scoped with tokio's `task::spawn`.
use futures::future::try_join_all;
use tracing::{debug, info, info_span, instrument, Instrument as _};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[instrument]
async fn parent_task(subtasks: usize) -> Result<(), Error> {
    info!("spawning subtasks...");
    let subtasks = (1..=subtasks)
        .map(|number| {
            if number % 2 == 0 {
                // With manually added instrument.
                let span = info_span!("subtask_v1", number);
                debug!(message = "creating subtask;", number);
                tokio::spawn(subtask_v1(number).instrument(span))
            } else {
                // Using instrument on async fn itself.
                debug!(message = "creating subtask;", number);
                tokio::spawn(subtask_v2(number))
            }
        })
        .collect::<Vec<_>>();

    // The returnable error would be if one of the subtasks panicked.
    let sum: usize = try_join_all(subtasks).await?.iter().sum();
    info!(%sum, "all subtasks completed; calculated sum");
    Ok(())
}

async fn subtask_v1(number: usize) -> usize {
    info!("polling subtask");
    number
}

#[instrument]
async fn subtask_v2(number: usize) -> usize {
    info!("polling subtask");
    number
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .try_init()?;
    parent_task(10).await?;
    Ok(())
}
