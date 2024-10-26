// https://www.apriorit.com/dev-blog/system-rust-asynch-vs-c-coroutines

use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll, Wake, Waker},
    time::{Duration, Instant},
};

struct MyWaker {}

impl Wake for MyWaker {
    fn wake(self: Arc<Self>) {
        println!("MyWaker::wake");
    }
}

struct Timer {
    start: Instant,
    wait: u32,
}

impl Timer {
    fn new(wait: u32) -> Timer {
        Self { start: Instant::now(), wait }
    }

    fn wait(&mut self) {
        let waker = Waker::from(Arc::new(MyWaker {}));
        let mut context = Context::from_waker(&waker);

        loop {
            match Pin::new(&mut *self).poll(&mut context) {
                Poll::Ready(elapsed) => {
                    println!("timer finished: elapsed {elapsed:?} secs");
                    break;
                }
                Poll::Pending => {
                    // Task is not ready for now, so sleep.
                    std::thread::sleep(Duration::from_secs(1));
                }
            }
        }
    }
}

impl Future for Timer {
    type Output = u32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let time_point = Instant::now();
        let elapsed = time_point.saturating_duration_since(self.start).as_secs() as u32;
        return if elapsed > self.wait {
            println!("Poll::Ready");
            Poll::Ready(elapsed)
        } else {
            // Need to wake waker, so the Tokio executor will know that the task needs
            // to be polled again because it hasn't finished.
            cx.waker().wake_by_ref();
            println!("Poll::Pending");
            Poll::Pending
        };
    }
}

fn main() {
    let mut timer = Timer::new(5);
    // Our own event loop inside.
    timer.wait();

    let runtime = tokio::runtime::Builder::new_multi_thread().build().unwrap();
    let timer = Timer::new(5);
    // Will create event loop inside.
    let elapsed = runtime.block_on(timer);
    println!("timer finished: elapsed {elapsed:?} secs");
}
