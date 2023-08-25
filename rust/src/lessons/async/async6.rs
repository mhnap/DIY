// https://hegdenu.net/posts/understanding-async-await-2
// The simplest `Future` with `Waker` implementation example.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// `YieldNow` can be yielded once.
struct YieldNow {
    yielded: bool,
}

impl Future for YieldNow {
    type Output = ();

    // Currently, `Context` struct basically wraps the `&Waker`.
    // But, possibly, it can be extended in the future.
    // By wrapping the waker like this, that extension would be possible.
    // If the poll function took the waker as a parameter directly, it wouldn't be.
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("YieldNow: poll(); yielded = {}", self.yielded);

        // Comment next lines to loop forever in `Pending` state.
        if self.yielded {
            return Poll::Ready(());
        }

        self.yielded = true;

        // The waker then calls to the async runtime to schedule() the current task.
        cx.waker().wake_by_ref();
        // Because we've already woken our task, we've indicate that we're ready to be polled again.
        // Now the task is scheduled.
        //
        // And so we see a difference when the task returns Poll::Pending back to the runtime.
        // The runtime now does have a task ready to poll (scheduled).

        Poll::Pending
    }
}

// `yield_now` function returns `YieldNow` task (that implements `Future` trait).
fn yield_now() -> YieldNow {
    YieldNow { yielded: false }
}

fn main() {
    let body = async {
        println!("Before yield_now().await");
        // When a future awaits another future and receives `Poll::Pending` it also returns `Poll::Pending`.
        //
        // Our yield_now() function is voluntarily yielding control to the runtime.
        // It's voluntarily because the task isn't actually waiting for anything.
        // The task could otherwise keep progressing.
        yield_now().await;
        println!("After yield_now().await");
    };

    // Build the Tokio runtime manually instead macro.
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime");

    // It's actually the async runtime that is calling `poll()` on the future which is driving the main task.
    rt.block_on(body);

    // Since the task that we called block_on with is finished, the runtime returns control to main().
    // And it returns the value from our future.
    // In this case there is no value, so it returns the unit type.
}
