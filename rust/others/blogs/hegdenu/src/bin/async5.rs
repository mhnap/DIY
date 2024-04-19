// https://hegdenu.net/posts/understanding-async-await-2

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// `Pending` task doesn't have states, it is always pending.
struct Pending;

impl Future for Pending {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Pending: poll()");

        Poll::Pending
    }
}

// `pending` function returns `Pending` task (that implements `Future` trait).
fn pending() -> Pending {
    Pending {}
}

#[tokio::main]
async fn main() {
    println!("Before pending().await");
    // It will hang there forever.
    // Our future is never polled again after returning Poll::Pending.
    // The reason is that the async runtime parks the thread until a task gets woken.
    pending().await;
    println!("After pending().await");
}
