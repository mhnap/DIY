// The simplest async "Hello world" example.

async fn hello(name: &'static str) {
    println!("hello, {name}!");
}

#[tokio::main]
async fn main() {
    hello("world").await;
}
