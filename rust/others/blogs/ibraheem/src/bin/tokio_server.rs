use std::time::Duration;
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
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

    loop {
        let (connection, _) = listener.accept().await.unwrap();

        tokio::spawn(async {
            if let Err(e) = handle_connection(connection).await {
                println!("failed to handle connection: {e}")
            }
        });
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
