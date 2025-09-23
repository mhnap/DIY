use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("localhost:3000").unwrap();

    loop {
        let (connection, _) = listener.accept().unwrap();

        // Spawn a thread to handle each connection.
        std::thread::spawn(|| {
            if let Err(e) = handle_connection(connection) {
                println!("failed to handle connection: {e}")
            }
        });
    }
}

fn handle_connection(mut connection: TcpStream) -> io::Result<()> {
    let request = read_request(&mut connection)?;
    println!("{request}");
    write_response(&mut connection)
}

fn read_request(connection: &mut TcpStream) -> io::Result<String> {
    let mut read = 0;
    let mut request = [0u8; 1024];

    loop {
        // Try reading from the stream.
        let num_bytes = connection.read(&mut request[read..])?;

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

fn write_response(connection: &mut TcpStream) -> io::Result<()> {
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
        let num_bytes = connection.write(response[written..].as_bytes())?;

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
    connection.flush()
}

/*

The key insight regarding thread-per-request is that our server is I/O bound.
Most of the time inside handle_connection is not spent doing compute work, it's spent waiting to send or receive some data across the network.
Functions like read, write, and flush perform blocking I/O. We submit an I/O request, yielding control to the kernel, and it returns control to us when the operation completes.
In the meantime, the kernel can execute other runnable threads, which is exactly what we want!

In general, most of the time it takes to serve a web request is spent waiting for other tasks to complete, like database queries, or other HTTP requests.
Multithreading works great because we can utilize that time to handle other requests.

It seems like threads do exactly what we need, and they're easy to use, so why not stop here?

You may have heard that threads are too heavyweight and context switching is very expensive.
Nowadays, that's not really true.
Modern servers can manage tens of thousands of threads without breaking a sweat.

The issue is that blocking I/O yields complete control of our program to the kernel until the requested operation completes.
We have no say in when we get to run again.
This is problematic because it makes it very difficult to model two operations: cancellation, and selection.

*/
