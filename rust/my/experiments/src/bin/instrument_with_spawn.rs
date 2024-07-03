// https://github.com/tokio-rs/tracing/blob/master/examples/examples/tokio-spawny-thing.rs
/// This is a example showing how information is scoped with Tokio's `task::spawn`.
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
                // This future has already been instrumented, thus internal Tokio span will not apply.
                tokio::spawn(subtask_v1(number).instrument(span))
            } else {
                // Using instrument on async fn itself.
                debug!(message = "creating subtask;", number);
                // This future is instrumented inside, thus internal Tokio span will apply.
                // But it's not added to the parent span.
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
    // Can enable Tokio internal instrumentation to spawned tasks.
    // https://github.com/tokio-rs/tokio/pull/2655
    // https://github.com/tokio-rs/tokio/blob/master/examples/chat.rs
    // https://hackmd.io/@aws-rust-platform/B1Vu6YwN_

    tracing_subscriber::fmt()
        // Filter what traces are displayed based on the RUST_LOG environment variable.
        // Can add `tokio=trace` to RUST_LOG to enable additional traces emitted by Tokio itself.
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        // Log events when `tracing` spans are created, entered, exited, or closed.
        // When Tokio's internal tracing support is enabled (as described above), this can be used to track the lifecycle of spawned tasks on the Tokio runtime.
        // .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .try_init()?;

    // To enable tokio internal instrumentation:
    // 1) Add `tracing` feature to `tokio` crate.
    // 2) Pass `RUSTFLAGS="--cfg tokio_unstable"` to rustc.
    // 3) Enable `tokio=trace` in RUST_LOG.
    // Full cmd is: RUSTFLAGS="--cfg tokio_unstable" RUST_LOG=debug,tokio=trace cargo run --bin instrument_with_spawn

    parent_task(10).await?;

    Ok(())
}

// 2024-07-03T16:39:19.600924Z  INFO parent_task{subtasks=10}: instrument_with_spawn: spawning subtasks...
// 2024-07-03T16:39:19.600958Z DEBUG parent_task{subtasks=10}: instrument_with_spawn: creating subtask; number=1
// 2024-07-03T16:39:19.601014Z DEBUG parent_task{subtasks=10}: instrument_with_spawn: creating subtask; number=2
// 2024-07-03T16:39:19.601044Z DEBUG parent_task{subtasks=10}: instrument_with_spawn: creating subtask; number=3
// 2024-07-03T16:39:19.601075Z DEBUG parent_task{subtasks=10}: instrument_with_spawn: creating subtask; number=4
// 2024-07-03T16:39:19.601082Z  INFO runtime.spawn{kind=task task.name= task.id=18 loc.file="my/experiments/src/bin/instrument_with_spawn.rs" loc.line=24 loc.col=17}:subtask_v2{number=1}: instrument_with_spawn: polling subtask
// 2024-07-03T16:39:19.601105Z DEBUG parent_task{subtasks=10}: instrument_with_spawn: creating subtask; number=5
// 2024-07-03T16:39:19.601084Z  INFO parent_task{subtasks=10}:subtask_v1{number=2}: instrument_with_spawn: polling subtask
// 2024-07-03T16:39:19.601170Z DEBUG parent_task{subtasks=10}: instrument_with_spawn: creating subtask; number=6
// 2024-07-03T16:39:19.601158Z  INFO runtime.spawn{kind=task task.name= task.id=20 loc.file="my/experiments/src/bin/instrument_with_spawn.rs" loc.line=24 loc.col=17}:subtask_v2{number=3}: instrument_with_spawn: polling subtask
// 2024-07-03T16:39:19.601163Z  INFO parent_task{subtasks=10}:subtask_v1{number=4}: instrument_with_spawn: polling subtask
// 2024-07-03T16:39:19.601199Z DEBUG parent_task{subtasks=10}: instrument_with_spawn: creating subtask; number=7
// 2024-07-03T16:39:19.601207Z  INFO parent_task{subtasks=10}:subtask_v1{number=6}: instrument_with_spawn: polling subtask
// 2024-07-03T16:39:19.601237Z DEBUG parent_task{subtasks=10}: instrument_with_spawn: creating subtask; number=8
// 2024-07-03T16:39:19.601238Z  INFO runtime.spawn{kind=task task.name= task.id=22 loc.file="my/experiments/src/bin/instrument_with_spawn.rs" loc.line=24 loc.col=17}:subtask_v2{number=5}: instrument_with_spawn: polling subtask
// 2024-07-03T16:39:19.601266Z DEBUG parent_task{subtasks=10}: instrument_with_spawn: creating subtask; number=9
// 2024-07-03T16:39:19.601259Z  INFO runtime.spawn{kind=task task.name= task.id=24 loc.file="my/experiments/src/bin/instrument_with_spawn.rs" loc.line=24 loc.col=17}:subtask_v2{number=7}: instrument_with_spawn: polling subtask
// 2024-07-03T16:39:19.601299Z DEBUG parent_task{subtasks=10}: instrument_with_spawn: creating subtask; number=10
// 2024-07-03T16:39:19.601286Z  INFO parent_task{subtasks=10}:subtask_v1{number=8}: instrument_with_spawn: polling subtask
// 2024-07-03T16:39:19.601310Z  INFO runtime.spawn{kind=task task.name= task.id=26 loc.file="my/experiments/src/bin/instrument_with_spawn.rs" loc.line=24 loc.col=17}:subtask_v2{number=9}: instrument_with_spawn: polling subtask
// 2024-07-03T16:39:19.601370Z  INFO parent_task{subtasks=10}:subtask_v1{number=10}: instrument_with_spawn: polling subtask
// 2024-07-03T16:39:19.601446Z  INFO parent_task{subtasks=10}: instrument_with_spawn: all subtasks completed; calculated sum sum=55
