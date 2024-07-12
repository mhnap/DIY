// https://stackoverflow.com/questions/69638710/when-should-you-use-tokiojoin-over-tokiospawn
// https://medium.com/@learnwithshobhit/demystifying-join-all-rust-a-guide-to-asynchronous-concurrency-b714decfb12a
// https://github.com/tokio-rs/tokio/issues/2478
// https://users.rust-lang.org/t/when-to-use-methods-macros-from-futures-and-from-tokio/85823

async fn print_numbers(prefix: &str) {
    for i in 0..5 {
        println!("{prefix}-{i}");
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

pub async fn measure<F, O>(f: F) -> tokio::time::Duration
where
    F: futures::Future<Output = O>,
{
    let start = tokio::time::Instant::now();
    f.await;
    start.elapsed()
}

#[tokio::main]
async fn main() {
    let prefixes = &["un", "dos", "tres", "cuatro"];

    println!("...TEST NUMERO CERO...");
    let duration = measure(async {
        for p in prefixes {
            print_numbers(p).await;
        }
    })
    .await;
    println!("DURATION CERO: {}", duration.as_millis());

    println!("...TEST NUMERO UNO...");
    let duration = measure(futures::future::join_all(
        prefixes.iter().map(|p| print_numbers(p)),
    ))
    .await;
    println!("DURATION UNO: {}", duration.as_millis());

    println!("...TEST NUMERO DOS...");
    let duration = measure(async {
        tokio::join!(
            print_numbers(prefixes[0]),
            print_numbers(prefixes[1]),
            print_numbers(prefixes[2]),
            print_numbers(prefixes[3]),
        )
    })
    .await;
    println!("DURATION DOS: {}", duration.as_millis());

    println!("...TEST NUMERO TRES...");
    let duration =
        measure(futures::future::join_all(prefixes.iter().map(|p| {
            tokio::task::spawn(async move { print_numbers(p).await })
        })))
        .await;
    println!("DURATION TRES: {}", duration.as_millis());
}
