use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    signal::ctrl_c,
    sync::Notify,
    time,
};

/*
 time ( curl -sS -o /dev/null http://localhost:3000/ & \
        curl -sS -o /dev/null http://localhost:3000/ & \
        wait )
*/

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let listener = TcpListener::bind("localhost:3000").await.unwrap();
    let state = Arc::new((AtomicUsize::new(0), Notify::new()));

    loop {
        tokio::select! {
            // New incoming connection.
            result = listener.accept() => {
                let (connection, _) = result.unwrap();
                let state = state.clone();

                // Increment the counter.
                state.0.fetch_add(1, Ordering::Relaxed);

                tokio::spawn(async move {
                    if let Err(e) = handle_connection(connection).await {
                        println!("failed to handle connection: {e}")
                    }

                    // Decrement the counter.
                    let count = state.0.fetch_sub(1, Ordering::Relaxed);
                    if count == 1 {
                        // We were the last active task.
                        state.1.notify_one();
                    }
                });
            }
            // Ctrl+C signal.
            _shutdown = ctrl_c() => {
                println!("Received ctrl_c.");

                // A 30 second timer.
                let timer = time::sleep(Duration::from_secs(30));
                // Notified by the last active task.
                let notification = state.1.notified();

                // If the count isn't zero, we have to wait.
                if state.0.load(Ordering::Relaxed) != 0 {
                    // Wait for either the timer or notification to resolve.
                    tokio::select! {
                        _ = timer => {}
                        _ = notification => {}
                    }
                }

                println!("Gracefully shutting down.");
                return;
            }
        }
    }
}

async fn handle_connection(mut connection: TcpStream) -> io::Result<()> {
    let request = read_request(&mut connection).await?;
    println!("{request}");
    time::sleep(Duration::from_secs(5)).await;
    write_response(&mut connection).await
}

async fn read_request(connection: &mut TcpStream) -> io::Result<String> {
    let mut read = 0;
    let mut request = [0u8; 1024];

    loop {
        // Try reading from the stream.
        let num_bytes = connection.read(&mut request[read..]).await?;

        // The client disconnected.
        if num_bytes == 0 {
            println!("client disconnected unexpectedly");
            return Ok(String::default());
        }

        // Keep track of how many bytes we've read.
        read += num_bytes;

        // Have we reached the end of the request?
        if request.get(read - 4..read) == Some(b"\r\n\r\n") {
            break;
        }
    }

    let request = String::from_utf8_lossy(&request[..read]);

    Ok(request.into_owned())
}

async fn write_response(connection: &mut TcpStream) -> io::Result<()> {
    // "Hello World!" in HTTP
    let response = concat!(
        "HTTP/1.1 200 OK\r\n",
        "Content-Length: 12\n",
        "Connection: close\r\n\r\n",
        "Hello world!"
    );

    let mut written = 0;

    loop {
        // Write the remaining response bytes.
        let num_bytes = connection.write(response[written..].as_bytes()).await?;

        // The client disconnected.
        if num_bytes == 0 {
            println!("client disconnected unexpectedly");
            return Ok(());
        }

        written += num_bytes;

        // Have we written the whole response yet?
        if written == response.len() {
            break;
        }
    }

    // Flush the response.
    connection.flush().await
}
