// Can configure the Tokio runtime manually instead of using #[tokio::main] macro.

async fn hello(name: &'static str) {
    println!("hello, {name}!");
}

fn main() {
    // Manually configure the Tokio runtime with a specific number of threads.
    let tokio_rt =
        tokio::runtime::Builder::new_multi_thread().worker_threads(2).enable_all().build().unwrap();

    tokio_rt.block_on(hello("world"));

    // Or can create additional new `async` block and use `.await`.
    tokio_rt.block_on(async { hello("world").await });
}
