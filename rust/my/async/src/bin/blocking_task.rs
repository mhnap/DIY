// https://ryhl.io/blog/async-what-is-blocking/

use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator as _};

async fn sleep_blocking(timer: i32) {
    println!("Start timer {}.", timer);
    // No .await here!
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("Timer {} done.", timer);
}

async fn sleep(timer: i32) {
    println!("Start timer {}.", timer);
    // Execution can be paused on await.
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("Timer {} done.", timer);
}

async fn parallel_sum(nums: Vec<i32>) -> i32 {
    let (send, recv) = tokio::sync::oneshot::channel();

    // Spawn a task on rayon.
    rayon::spawn(move || {
        // Perform an expensive computation.
        // Compute the sum on multiple threads.
        let sum = nums.par_iter().sum();

        // Send the result back to Tokio.
        let _ = send.send(sum);
    });

    // Wait for the rayon task.
    recv.await.expect("Panic in rayon::spawn")
}

#[tokio::main]
async fn main() {
    // The join! macro run multiple things concurrently on the same thread.
    tokio::join!(sleep_blocking(1), sleep_blocking(2), sleep_blocking(3));
    println!();
    tokio::join!(sleep(1), sleep(2), sleep(3));
    println!();
    let sum = parallel_sum((1..1000).collect()).await;
    println!("Sum is {sum}.");
}

// Start timer 1.
// Timer 1 done.
// Start timer 2.
// Timer 2 done.
// Start timer 3.
// Timer 3 done.

// Start timer 1.
// Start timer 2.
// Start timer 3.
// Timer 2 done.
// Timer 3 done.
// Timer 1 done.

// Sum is 499500.
