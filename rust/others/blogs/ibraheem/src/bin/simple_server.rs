use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("localhost:3000").unwrap();

    loop {
        let (connection, _) = listener.accept().unwrap();

        if let Err(e) = handle_connection(connection) {
            println!("failed to handle connection: {e}")
        }
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

See the problem?

Our server can only serve a single request at a time.

Reading and writing from/to a network connection isn't instantaneous, there's a lot of infrastructure between us and the user.
What would happen if two users made a request to our server at the same time, or ten, or ten thousand?
Obviously this isn't going to scale, so what do we do?

We have a couple of options, but by far the simplest one is to spawn some threads.
Spawn a thread for each request and our server becomes infinitely faster, right?

*/
