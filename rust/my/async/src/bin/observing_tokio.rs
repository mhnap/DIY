// https://www.youtube.com/watch?v=BlZlBdnizbw
// https://www.datocms-assets.com/98516/1707127998-stainsby_2023.pdf

use tokio::time::{self, Duration};
use tracing::{info, instrument};

#[instrument]
async fn task(num: u8) {
    info!("started");
    task_inner(num).await;
    info!("finished");
}

#[instrument]
async fn task_inner(num: u8) {
    time::sleep(Duration::from_secs(num.into())).await;
}

#[tokio::main]
async fn main() {
    println!("Start!");

    console_subscriber::init();

    // Construct a metrics task monitor.
    let metrics_monitor = tokio_metrics::TaskMonitor::new();

    // Print metrics every 1 second.
    {
        let metrics_monitor = metrics_monitor.clone();
        tokio::spawn(async move {
            for interval in metrics_monitor.intervals() {
                dbg!(interval);
                dbg!(metrics_monitor.cumulative());
                time::sleep(Duration::from_secs(1)).await;
            }
        });
    }

    let handle = tokio::runtime::Handle::current();

    // Construct the runtime metrics monitor.
    let runtime_monitor = tokio_metrics::RuntimeMonitor::new(&handle);

    // Print runtime metrics every 1 second.
    {
        tokio::spawn(async move {
            for interval in runtime_monitor.intervals() {
                dbg!(interval);
                time::sleep(Duration::from_secs(1)).await;
            }
        });
    }

    // Construct tasks dumping.
    {
        tokio::spawn(async move {
            loop {
                if let Ok(dump) = time::timeout(Duration::from_secs(1), handle.dump()).await {
                    for (i, task) in dump.tasks().iter().enumerate() {
                        let trace = task.trace();
                        println!("TASK {i}:");
                        println!("{trace}\n");
                    }
                }
                time::sleep(Duration::from_secs(1)).await;
            }
        });
    }

    // Spawn ten tasks.
    let tasks = futures::future::join_all((0..10).into_iter().map(|i| {
        // Can add a name to the task which will be displayed in the `tokio-console`.
        let task_name = format!("my task {i}");
        tokio::task::Builder::new()
            .name(&task_name)
            .spawn(metrics_monitor.instrument(task(i)))
            .unwrap()
    }));

    // Wait for all tasks.
    tasks.await;

    println!("All done!");
}
