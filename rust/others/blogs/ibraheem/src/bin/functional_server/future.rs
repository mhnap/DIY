// use std::marker::PhantomData;

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

// pub struct WithData<'data, D, F> {
//     data: D,
//     future: F,
//     _data: PhantomData<&'data D>,
// }

// impl<'data, D, F> WithData<'data, D, F>
// where
//     F: Future + 'data,
// {
//     pub fn new(data: D, construct: impl Fn(&'data D) -> F) -> WithData<'data, D, F> {
//         let future = construct(&data);
//
//         // error[E0597]: `data` does not live long enough
//         //   --> others/blogs/ibraheem/src/bin/functional_server/future.rs:87:32
//         //    |
//         // 82 | impl<'data, D, F> WithData<'data, D, F>
//         //    |      ----- lifetime `'data` defined here
//         // ...
//         // 86 |     pub fn new(data: D, construct: impl Fn(&'data D) -> F) -> WithData<'data, D, F> {
//         //    |                ---- binding `data` declared here
//         // 87 |         let future = construct(&data);
//         //    |                      ----------^^^^^-
//         //    |                      |         |
//         //    |                      |         borrowed value does not live long enough
//         //    |                      argument requires that `data` is borrowed for `'data`
//         // 88 |         WithData { data, future, _data: PhantomData }
//         // 89 |     }
//         //    |     - `data` dropped here while still borrowed
//
//         WithData { data, future, _data: PhantomData }
//
//         // error[E0505]: cannot move out of `data` because it is borrowed
//         //    --> others/blogs/ibraheem/src/bin/functional_server/future.rs:108:20
//         //     |
//         // 82  | impl<'data, D, F> WithData<'data, D, F>
//         //     |      ----- lifetime `'data` defined here
//         // ...
//         // 86  |     pub fn new(data: D, construct: impl Fn(&'data D) -> F) -> WithData<'data, D, F> {
//         //     |                ---- binding `data` declared here
//         // 87  |         let future = construct(&data);
//         //     |                      ----------------
//         //     |                      |         |
//         //     |                      |         borrow of `data` occurs here
//         //     |                      argument requires that `data` is borrowed for `'data`
//         // ...
//         // 108 |         WithData { data, future, _data: PhantomData }
//         //     |                    ^^^^ move out of `data` occurs here
//         //     |
//         // help: if `D` implemented `Clone`, you could clone the value
//         //    --> others/blogs/ibraheem/src/bin/functional_server/future.rs:82:13
//         //     |
//         // 82  | impl<'data, D, F> WithData<'data, D, F>
//         //     |             ^ consider constraining this type parameter with `Clone`
//         // ...
//         // 87  |         let future = construct(&data);
//         //     |                                 ---- you could clone this value
//     }
// }

// impl<'data, D, F> Future for WithData<'data, D, F>
// where
//     F: Future,
// {
//     type Output = F::Output;
//
//     fn poll(&mut self, waker: Waker) -> Option<Self::Output> {
//         self.future.poll(waker)
//     }
// }
