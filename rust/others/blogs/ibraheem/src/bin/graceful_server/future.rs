use std::sync::{Arc, Mutex};

use crate::runtime::Waker;

pub trait Future {
    type Output;

    fn poll(&mut self, waker: Waker) -> Option<Self::Output>;

    fn chain<F, T>(self, transition: F) -> Chain<Self, F, T>
    where
        F: FnOnce(Self::Output) -> T,
        T: Future,
        Self: Sized,
    {
        Chain::First { future1: self, transition: Some(transition) }
    }
}

pub enum Chain<T1, F, T2> {
    First { future1: T1, transition: Option<F> },
    Second { future2: T2 },
}

impl<T1, F, T2> Future for Chain<T1, F, T2>
where
    T1: Future,
    F: FnOnce(T1::Output) -> T2,
    T2: Future,
{
    type Output = T2::Output;

    fn poll(&mut self, waker: Waker) -> Option<Self::Output> {
        if let Chain::First { future1, transition } = self {
            // Poll the first future.
            match future1.poll(waker.clone()) {
                Some(value) => {
                    // The first future is done, transition into the second.
                    let future2 = (transition.take().unwrap())(value);
                    *self = Chain::Second { future2 };
                }
                // The first future is not ready, return.
                None => return None,
            }
        }

        if let Chain::Second { future2 } = self {
            // The first future is already done, poll the second.
            return future2.poll(waker);
        }

        None
    }
}

pub fn poll_fn<F, T>(f: F) -> impl Future<Output = T>
where
    F: FnMut(Waker) -> Option<T>,
{
    struct FromFn<F>(F);

    impl<F, T> Future for FromFn<F>
    where
        F: FnMut(Waker) -> Option<T>,
    {
        type Output = T;

        fn poll(&mut self, waker: Waker) -> Option<Self::Output> {
            (self.0)(waker)
        }
    }

    FromFn(f)
}

pub fn spawn_blocking(blocking_work: impl FnOnce() + Send + 'static) -> impl Future<Output = ()> {
    let state: Arc<Mutex<(bool, Option<Waker>)>> = Arc::default();
    let state_handle = state.clone();

    // Run the blocking work on a separate thread.
    std::thread::spawn(move || {
        // Perform the work.
        blocking_work();

        // Mark the task as done.
        let (done, waker) = &mut *state_handle.lock().unwrap();
        *done = true;

        // Wake the waker.
        if let Some(waker) = waker.take() {
            waker.wake();
        }
    });

    poll_fn(move |waker| match &mut *state.lock().unwrap() {
        // The work is not completed, store our waker and come back later.
        (false, state) => {
            *state = Some(waker);
            None
        }
        // The work was completed.
        (true, _) => Some(()),
    })
}

pub fn select<L, R>(left: L, right: R) -> Select<L, R> {
    Select { left, right }
}

pub struct Select<L, R> {
    left: L,
    right: R,
}

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L: Future, R: Future> Future for Select<L, R> {
    type Output = Either<L::Output, R::Output>;

    fn poll(&mut self, waker: Waker) -> Option<Self::Output> {
        if let Some(output) = self.left.poll(waker.clone()) {
            return Some(Either::Left(output));
        }

        if let Some(output) = self.right.poll(waker) {
            return Some(Either::Right(output));
        }

        None
    }
}

#[derive(Default)]
pub struct Counter {
    state: Mutex<(usize, Option<Waker>)>,
}

impl Counter {
    pub fn increment(&self) {
        let (count, _) = &mut *self.state.lock().unwrap();
        *count += 1;
    }

    pub fn decrement(&self) {
        let (count, waker) = &mut *self.state.lock().unwrap();
        *count -= 1;

        // We were the last task.
        if *count == 0 {
            // Wake the shutdown handler.
            if let Some(waker) = waker.take() {
                waker.wake();
            }
        }
    }

    pub fn wait_for_zero(self: Arc<Self>) -> impl Future<Output = ()> {
        poll_fn(move |waker| {
            match &mut *self.state.lock().unwrap() {
                // The work was completed.
                (0, _) => Some(()),
                // The work was not completed, store our waker and come back later.
                (_, state) => {
                    *state = Some(waker);
                    None
                }
            }
        })
    }
}
