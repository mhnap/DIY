// https://cybernetist.com/2024/04/19/rust-tokio-task-cancellation-patterns/
// https://github.com/tokio-rs/tokio/issues/1830
// https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html

use chrono::Utc;
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    // Dropping JoinHandle DOES NOT CANCEL THE TASK.
    let handle = tokio::spawn(async {
        println!("Task started {}", Utc::now());
        tokio::time::sleep(Duration::from_secs(2)).await;
        println!("Task completed {}", Utc::now());
    });
    println!("Task spawned {}", Utc::now());
    time::sleep(Duration::from_secs(1)).await;
    drop(handle);
    println!("Task cancelled {}", Utc::now());
    time::sleep(Duration::from_secs(2)).await;
    println!("Finish {}", Utc::now());

    // A JoinHandle detaches the associated task when it is dropped, which means that there is no longer any handle to the task, and no way to join on it.
    // If a JoinHandle is dropped, then the task continues running in the background and its return value is lost.

    println!();

    // `abort()` should be called explicitly.
    let handle = tokio::spawn(async {
        println!("Task started {}", Utc::now());
        tokio::time::sleep(Duration::from_secs(2)).await;
        println!("Task completed {}", Utc::now());
    });
    println!("Task spawned {}", Utc::now());
    time::sleep(Duration::from_secs(1)).await;
    handle.abort();
    println!("Task cancelled {}", Utc::now());
    time::sleep(Duration::from_secs(2)).await;
    println!("Finish {}", Utc::now());

    println!();

    // tokio::select! does not abort the spawned task.
    let handle_quicker = tokio::spawn(async {
        println!("Task 1 started {}", Utc::now());
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("Task 1 completed {}", Utc::now());
    });
    println!("Task 1 spawned {}", Utc::now());

    let handle_slower = tokio::spawn(async {
        println!("Task 2 started {}", Utc::now());
        tokio::time::sleep(Duration::from_secs(2)).await;
        println!("Task 2 completed {}", Utc::now());
    });
    println!("Task 2 spawned {}", Utc::now());

    tokio::select! {
        _ = handle_quicker => {
            println!("Finish task 1 {}", Utc::now());
        }
        _ = handle_slower => {
            println!("Finish task 2 {}", Utc::now());
        }
    }

    time::sleep(Duration::from_secs(2)).await;
    println!("Finish {}", Utc::now());
}
