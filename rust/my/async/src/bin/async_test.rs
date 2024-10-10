use std::sync::Arc;
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() {
    let semaphore = Arc::new(Semaphore::new(2));
    let mut join_handles = Vec::new();

    for _ in 0..5 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        join_handles.push(tokio::spawn(async move {
            println!("start!");
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            // perform task...
            // explicitly own `permit` in the task
            drop(permit);
            println!("exit!");
        }));
    }

    for handle in join_handles {
        handle.await.unwrap();
    }
}
