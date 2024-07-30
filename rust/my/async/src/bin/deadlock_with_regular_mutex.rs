// https://fasterthanli.me/articles/a-rust-match-made-in-hell

use futures::future::join_all;
use std::{sync::Mutex, time::Duration};
use tokio::time::sleep;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // There is no deadlock here.
    let res: Mutex<String> = Default::default();
    join_all("abc".chars().map(|name| {
        let res = &res;
        async move {
            for _ in 0..5 {
                sleep(Duration::from_millis(10)).await;
                res.lock().unwrap().push(name);
            }
        }
    }))
    .await;
    println!("res = {}", res.into_inner().unwrap());

    // There is a deadlock here.
    let res: Mutex<String> = Default::default();
    join_all("abc".chars().map(|name| {
        let res = &res;
        async move {
            for _ in 0..5 {
                let mut guard = res.lock().unwrap();
                sleep(Duration::from_millis(10)).await;
                guard.push(name);
            }
        }
    }))
    .await;
    println!("res = {}", res.into_inner().unwrap());

    // There would be no deadlock with [`futures::lock::Mutex`] or [`tokio::sync::Mutex`].
}
